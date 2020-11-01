use std::{fmt, hash::Hash, hash::Hasher};

use amethyst::ecs::{Component, DenseVecStorage};
use abm::location::{Int2D, Location2D};
use rand::Rng;

use crate::{environment::{HEIGHT, WIDTH}, resources::AntsGrid, resources::ObstaclesGrid, resources::SitesGrid, resources::ToFoodGrid, resources::ToHomeGrid, static_object::StaticObjectType};
use crate::environment::TintEvent;
use amethyst::core::ecs::shrev::EventChannel;
use crate::environment::TintEvent::UpdateTint;

pub const REWARD: f64 = 1.;
pub const MOMENTUM_PROBABILITY: f64 = 0.8;
pub const RANDOM_ACTION_PROBABILITY: f64 = 0.1;
pub const UPDATE_CUTDOWN: f64 = 0.9;
/* arithmetic cannot be used in const fns yet, we're using lazy_static! to bypass this limitation
pub const fn DIAGONAL_CUTDOWN() -> f64 {
    UPDATE_CUTDOWN.powf((2 as f64).sqrt())
}*/

lazy_static! {
    static ref DIAGONAL_CUTDOWN: f64 = UPDATE_CUTDOWN.powf((2 as f64).sqrt());
}

/*
    Adapter that represents an entity as an agent for RustAB calculations.
    Back-end operations go here and they are handled by RustAB.
*/

#[derive(Clone, Copy)]
pub struct AgentAdapter {
    // An unique id.
    pub id: u128,
    // The position of the agent.
    pub loc: Int2D,
    // Last position of the agent, starts as None
    pub last: Option<Int2D>,
    // False means the agent will try to find food. True means the agent will try to return home.
    pub has_food: bool,
    // Value used to increase the pheromones in the nest and in the food source.
    // This will let the agents spread pheromones in the surrounding areas from point of interests
    // so that other agents will know which path to take to do their job.
    pub reward: f64,

}

impl AgentAdapter {
    pub fn new(id: u128, loc: Int2D, has_food: bool, reward: f64) -> AgentAdapter {
        AgentAdapter {
            id,
            loc,
            last: None,
            has_food,
            reward,
        }
    }

    // Deposit a pheromone related to food or home, so that other agents will take in account this value when choosing the next step's direction.
    pub fn deposit_pheromone(
        &mut self,
        to_home_grid: &mut ToHomeGrid,
        to_food_grid: &mut ToFoodGrid,
        event_channel: &mut EventChannel<TintEvent>,
    ) {
        let x = self.loc.x;
        let y = self.loc.y;

        // TODO support for multiple algorithms
        let (index, mut max) = if self.has_food {
            to_food_grid.grid.get_value_at_pos(&self.loc)
        } else {
            to_home_grid.grid.get_value_at_pos(&self.loc)
        }.unwrap();
        // Find the highest pheromone we care about in the surrounding 3x3 area
        for dx in -1..2 {
            for dy in -1..2 {
                let _x = dx + x;
                let _y = dy + y;
                if _x < 0 || _y < 0 || _x >= WIDTH || _y >= HEIGHT { // No going out of the field
                    continue;
                }
                // Fetch the pheromone in the surrounding area, decrease it a bit and add the reward to it
                let (_, pheromone) = if self.has_food {
                    to_food_grid.grid.get_value_at_pos(&Int2D { x: _x, y: _y }).unwrap()
                } else {
                    to_home_grid.grid.get_value_at_pos(&Int2D { x: _x, y: _y }).unwrap()
                };
                let m = pheromone
                    * {
                    if dx * dy != 0 {
                        *DIAGONAL_CUTDOWN // On the corners, we deposit less pheromones
                    } else {
                        UPDATE_CUTDOWN
                    }
                } + self.reward;
                if m > max {
                    max = m;
                }
            }
        }
        if self.has_food {
            to_food_grid.grid.set_value_at_pos(&self.loc, (index, max))
        } else {
            to_home_grid.grid.set_value_at_pos(&self.loc, (index, max))
        }
        event_channel.single_write(UpdateTint(index, max, self.has_food));
        self.reward = 0.;
    }

    // Handles movement
    pub fn act(
        &mut self,
        ant_grid: &mut AntsGrid,
        obstacles_grid: &ObstaclesGrid,
        sites_grid: &SitesGrid,
        to_home_grid: &ToHomeGrid,
        to_food_grid: &ToFoodGrid,
    ) {
        let mut rng = rand::thread_rng();
        let mut max = -1.; //impossibly bad pheromone

        let x = self.loc.x;
        let y = self.loc.y;

        let mut max_x = x; // current x loc
        let mut max_y = y; // current y loc
        let mut count = 2; // How many equal pheromones are there around us? Will be used to choose one randomly
        for dx in -1..2 { // 3x3 box around the anx
            for dy in -1..2 {
                let new_x = dx + x;
                let new_y = dy + y;
                let new_int2d = Int2D { x: new_x, y: new_y };
                if (dx == 0 && dy == 0) // Skip if we're trying to stay still, if we're trying to exit the field or we encounter an obstacle
                    || new_x < 0 || new_y < 0
                    || new_x >= WIDTH || new_y >= HEIGHT
                    || obstacles_grid.grid.get_value_at_pos(&new_int2d).is_some()
                {
                    continue;
                }
                let (_, m) = if self.has_food { // Consider the pheromone left by other agents
                    to_home_grid.grid.get_value_at_pos(&new_int2d).unwrap()
                } else {
                    to_food_grid.grid.get_value_at_pos(&new_int2d).unwrap()
                };
                if m > max { // We found a new maximum, reset the count
                    count = 2; // If we find two possible steps, chance will be 0.5. If we find a third, 0.33 etc...
                }
                if m > max || (m == max && rng.gen_bool(1. / count as f64)) { // Latter expression is to take a random step towards paths with a good pheromone
                    max = m;
                    max_x = new_x;
                    max_y = new_y;
                }
                count += 1;
            }
        }
        if max == 0. && self.last != None { // No tips from the pheromones, consider going straight
            if let Some(last_pos) = self.last {
                if rng.gen_bool(MOMENTUM_PROBABILITY) {
                    let xm = x + (x - last_pos.x);
                    let ym = y + (y - last_pos.y);
                    // Don't go outside the field or in an obstacle
                    if xm >= 0 && xm < WIDTH && ym >= 0 && ym < HEIGHT && obstacles_grid.grid.get_value_at_pos(&Int2D { x: xm, y: ym }).is_none() {
                        max_x = xm;
                        max_y = ym;
                    }
                }
            }
        } else if rng.gen_bool(RANDOM_ACTION_PROBABILITY) { // Consider going randomly
            let xd: i64 = rng.gen_range(-1, 2);
            let yd: i64 = rng.gen_range(-1, 2);
            let xm = x + xd;
            let ym = y + yd;
            // Don't go outside the field, in an obstacle and do not stay still
            if !(xd == 0 && yd == 0) && xm >= 0 && xm < WIDTH && ym >= 0 && ym < HEIGHT && obstacles_grid.grid.get_value_at_pos(&Int2D { x: xm, y: ym }).is_none() {
                max_x = xm;
                max_y = ym;
            }
        }
        self.loc = Int2D { x: max_x, y: max_y };
        ant_grid.grid.set_object_location(self, self.loc);
        self.last = Some(Int2D { x, y });
        // TODO cleanup?
        if let Some(site) = sites_grid.grid.get_value_at_pos(&self.loc) {
            match site {
                StaticObjectType::HOME => {
                    if self.has_food {
                        self.reward = REWARD;
                        self.has_food = !self.has_food;
                    }
                }
                StaticObjectType::FOOD => {
                    if !self.has_food {
                        self.reward = REWARD;
                        self.has_food = !self.has_food;
                    }
                }
                _ => ()
            }
        }
    }
}

// Implements Component so that we can attach it to entities and fetch it in systems.
impl Component for AgentAdapter {
    type Storage = DenseVecStorage<Self>;
}


impl Hash for AgentAdapter {
    fn hash<H>(&self, state: &mut H)
        where
            H: Hasher,
    {
        state.write_u128(self.id);
        state.finish();
    }
}

impl Eq for AgentAdapter {}

impl PartialEq for AgentAdapter {
    fn eq(&self, other: &AgentAdapter) -> bool {
        self.id == other.id
    }
}

impl Location2D<Int2D> for AgentAdapter {
    fn get_location(self) -> Int2D {
        self.loc
    }

    fn set_location(&mut self, loc: Int2D) {
        self.loc = loc;
    }
}

impl fmt::Display for AgentAdapter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}