use crate::state::State;
use crate::static_objects::StaticObjectType;
use crate::{GLOBAL_STATE, HEIGHT, WIDTH};
use abm::agent::Agent;
use abm::location::{Int2D, Location2D};
use rand::Rng;
use std::hash::{Hash, Hasher};

pub const REWARD: f64 = 1.;
pub const MOMENTUM_PROBABILITY: f64 = 0.8;
pub const RANDOM_ACTION_PROBABILITY: f64 = 0.1;
pub const UPDATE_CUTDOWN: f64 = 0.9;

// Lazy static needed to be able to calculate the diagonal cutdown based on the value of UPDATE_CUTDOWN.
lazy_static! {
    static ref DIAGONAL_CUTDOWN: f64 = UPDATE_CUTDOWN.powf((2 as f64).sqrt());
}

/// A struct representing an ant, with an id, a position, whether it's holding food or not and the
/// current reward, used to increase the pheromone on the location of the ant if a site is reached.
#[derive(Copy, Clone)]
pub struct Ant {
    /// An unique id.
    pub id: u128,
    /// The position of the agent.
    pub loc: Int2D,
    /// Last position of the agent, starts as None
    pub last: Option<Int2D>,
    /// False means the agent will try to find food by following food pheromones if possible, or by
    /// flooding the grid until it is found. True means the agent will try to return home by using the
    /// previously deposited pheromones.
    pub has_food: bool,
    /// Value used to increase the pheromones in the nest and in the food source.
    /// This will let the agents spread pheromones in the surrounding areas from point of interests
    /// so that other agents will know which path to take to do their job.
    pub reward: f64,
}

impl Ant {
    pub fn new(id: u128, loc: Int2D, has_food: bool, reward: f64) -> Ant {
        Ant {
            id,
            loc,
            last: None,
            has_food,
            reward,
        }
    }

    /// Deposit a home pheromone if self is not holding food, else deposit a food pheromone,
    /// so that other agents will take in account the pheromone value when choosing the next step's
    /// direction.
    pub fn deposit_pheromone(&mut self, state: &mut State) {
        let x = self.loc.x;
        let y = self.loc.y;

        // Fetch the value of the correct pheromone on our location, depending whether we're holding
        // food or not.
        let mut max = if self.has_food {
            state.get_food_pheromone(&self.loc)
        } else {
            state.get_home_pheromone(&self.loc)
        }
        .unwrap_or(0.);

        // Find the highest pheromone we care about in the surrounding 3x3 area to calculate the value
        // of the pheromone in our current area. Normally, the maximum pheromone in the 3x3 area is fetched
        // and it is decreased slightly, then it is assigned to our location.
        for dx in -1..2 {
            for dy in -1..2 {
                let _x = dx + x;
                let _y = dy + y;
                if _x < 0 || _y < 0 || _x >= WIDTH || _y >= HEIGHT {
                    // Do not try to take into account out of bounds grid cells
                    continue;
                }
                // Fetch the pheromone in the cell we're analyzing
                let pheromone = if self.has_food {
                    state.get_food_pheromone(&Int2D { x: _x, y: _y })
                } else {
                    state.get_home_pheromone(&Int2D { x: _x, y: _y })
                }
                .unwrap_or(0.);
                // Decrease the value a bit, with diagonal cells of our 3x3 grid considered farther
                let m = (pheromone * {
                    if dx * dy != 0 {
                        *DIAGONAL_CUTDOWN
                    } else {
                        UPDATE_CUTDOWN
                    }
                }) + self.reward;
                if m > max {
                    max = m;
                }
            }
        }
        // Set the new value of the pheromone we're considering
        if self.has_food {
            state.set_food_pheromone(&self.loc, max)
        } else {
            state.set_home_pheromone(&self.loc, max)
        }
        // We have used our reward, reset it
        self.reward = 0.;
    }

    /// Step to the next cell by taking into account pheromones. If no pheromones of the right type
    /// are found in a 3x3 grid centered on us, try to step in the same direction of the last frame
    /// with a probability of MOMENTUM_PROBABILITY. Otherwise, step in a random direction with a
    /// probability of RANDOM_ACTION_PROBABILITY.
    pub fn act(&mut self, state: &mut State) {
        let mut rng = rand::thread_rng();
        let mut max = -1.; // An initial, impossible pheromone.

        let x = self.loc.x;
        let y = self.loc.y;

        let mut max_x = x;
        let mut max_y = y;
        let mut count = 2; // How many equal pheromones are there around us? Will be used to choose one randomly

        // Check a 3x3 grid centered on us to get a hint on where to step next through the pheromones around us
        for dx in -1..2 {
            for dy in -1..2 {
                let new_x = dx + x;
                let new_y = dy + y;
                let new_int2d = Int2D { x: new_x, y: new_y };
                // Skip the cell we're considering if we're trying to stay still, if we're trying
                // to exit the field or of we encounter an obstacle
                if (dx == 0 && dy == 0)
                    || new_x < 0
                    || new_y < 0
                    || new_x >= WIDTH
                    || new_y >= HEIGHT
                    || state.get_obstacle(&new_int2d).is_some()
                {
                    continue;
                }

                let m = if self.has_food {
                    state.get_home_pheromone(&new_int2d)
                } else {
                    state.get_food_pheromone(&new_int2d)
                }
                .unwrap_or(0.);
                if m > max {
                    // We found a new maximum, reset the count
                    count = 2;
                }
                // A new maximum is found, or the maximux hasn't changed. In the latter case, we
                // randomly choose whether to consider the new cell for the next step or not with an
                // equal chance.
                if m > max || (m == max && rng.gen_bool(1. / count as f64)) {
                    // Latter expression is to take a random step towards paths with a good pheromone
                    max = m;
                    max_x = new_x;
                    max_y = new_y;
                }
                count += 1;
            }
        }

        if max == 0. && self.last != None {
            // No tips from pheromones, consider stepping in the same direction
            if let Some(last_pos) = self.last {
                if rng.gen_bool(MOMENTUM_PROBABILITY) {
                    let xm = x + (x - last_pos.x);
                    let ym = y + (y - last_pos.y);
                    // Don't go outside the field or in an obstacle
                    if xm >= 0
                        && xm < WIDTH
                        && ym >= 0
                        && ym < HEIGHT
                        && state.get_obstacle(&Int2D { x: xm, y: ym }).is_none()
                    {
                        max_x = xm;
                        max_y = ym;
                    }
                }
            }
        } else if rng.gen_bool(RANDOM_ACTION_PROBABILITY) {
            // All other ideas have failed, just choose a random direction
            let xd: i64 = rng.gen_range(-1, 2);
            let yd: i64 = rng.gen_range(-1, 2);
            let xm = x + xd;
            let ym = y + yd;
            // Don't go outside the field, in an obstacle and do not stay still
            if !(xd == 0 && yd == 0)
                && xm >= 0
                && xm < WIDTH
                && ym >= 0
                && ym < HEIGHT
                && state.get_obstacle(&Int2D { x: xm, y: ym }).is_none()
            {
                max_x = xm;
                max_y = ym;
            }
        }
        let loc = Int2D { x: max_x, y: max_y };
        self.loc = loc;
        state.set_ant_location(self, &loc);
        self.last = Some(Int2D { x, y });

        // Get rewarded if we've reached a site and update our food status
        if let Some(site) = state.get_site(&self.loc) {
            match site {
                StaticObjectType::HOME => {
                    if self.has_food {
                        if !state.food_returned_home {
                            state.food_returned_home = true;
                        }
                        self.reward = REWARD;
                        self.has_food = !self.has_food;
                    }
                }
                StaticObjectType::FOOD => {
                    if !self.has_food {
                        if !state.food_source_found {
                            state.food_source_found = true;
                        }
                        self.reward = REWARD;
                        self.has_food = !self.has_food;
                    }
                }
                _ => (),
            }
        }
    }
}

impl Agent for Ant {
    /// Each ant deposits a pheromone in its current location, then it steps in the next grid cell.
    fn step(&mut self) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        self.deposit_pheromone(&mut state);
        self.act(&mut state);
    }
}

impl Eq for Ant {}

impl PartialEq for Ant {
    fn eq(&self, other: &Ant) -> bool {
        self.id == other.id
    }
}

impl Location2D<Int2D> for Ant {
    fn get_location(self) -> Int2D {
        self.loc
    }

    fn set_location(&mut self, loc: Int2D) {
        self.loc = loc;
    }
}

impl Hash for Ant {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}
