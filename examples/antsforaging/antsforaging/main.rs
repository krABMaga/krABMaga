use crate::resources::ant::Ant;
use crate::resources::state::State;
use crate::resources::static_objects::StaticObjectType;
use rand::Rng;
use rust_ab::engine::location::Int2D;
use rust_ab::engine::schedule::Schedule;

mod resources;

pub const WIDTH: i64 = 200;
pub const HEIGHT: i64 = 200;
pub const NUM_AGENT: u128 = 500;
pub const EVAPORATION: f64 = 0.999;
pub const STEP: u128 = 50000;
// Nest coordinate range
pub const HOME_XMIN: i64 = 175;
pub const HOME_XMAX: i64 = 175;
pub const HOME_YMIN: i64 = 175;
pub const HOME_YMAX: i64 = 175;
// Food coordinate range
pub const FOOD_XMIN: i64 = 25;
pub const FOOD_XMAX: i64 = 25;
pub const FOOD_YMIN: i64 = 25;
pub const FOOD_YMAX: i64 = 25;

fn main() {
    let mut schedule: Schedule<Ant> = Schedule::new();
    let mut state = State::new(WIDTH, HEIGHT);

    generate_nest(&mut state);
    generate_food(&mut state);
    generate_obstacles(&mut state);
    generate_ants(&mut state, &mut schedule);

    state.update_obstacles();
    state.update_sites();

    //let mut food_source_found_processed = false;
    //let start = Instant::now();

    // Start the simulation. Ants will move, deposit pheromones and, for each step, the pheromone grids
    // will evaporate slightly.
    for step in 1..STEP {
        if step % 100 == 0 {
            println!("Milestone {}", step);
        }
        state.step = step;

        schedule.step(&mut state);
    }

    /* Time is currently not tracked. The simulation stops the moment the first ant returns home with food.
        Stopping the schedule at such moment would require mutable access to the state, which is not possible now.
    let duration = start.elapsed();

    println!("Time elapsed in testing schedule is: {:?}", duration);
    println!(
        "Step for seconds: {:?}",
        STEP as f64 / (duration.as_secs() as f64) // Can return "inf" if the simulation was extremely fast
    );*/
}

/// Generate the nest site, at a specific location or in a random one within a range.
fn generate_nest(state: &mut State) {
    let mut rng = rand::thread_rng();

    // Generate the nest
    let x: i64 = if HOME_XMIN == HOME_XMAX {
        HOME_XMIN
    } else {
        rng.gen_range(HOME_XMIN, HOME_XMAX)
    };
    let y: i64 = if HOME_YMIN == HOME_YMAX {
        HOME_YMIN
    } else {
        rng.gen_range(HOME_YMIN, HOME_YMAX)
    };
    state.set_site(&Int2D { x, y }, StaticObjectType::HOME);
}

/// Generate the food site, at a specific location or in a random one within a range.
fn generate_food(state: &mut State) {
    let mut rng = rand::thread_rng();
    // Generate the food resource
    let x: i64 = if FOOD_XMIN == FOOD_XMAX {
        FOOD_XMIN
    } else {
        rng.gen_range(FOOD_XMIN, FOOD_XMAX)
    };
    let y: i64 = if FOOD_YMIN == FOOD_YMAX {
        FOOD_YMIN
    } else {
        rng.gen_range(FOOD_YMIN, FOOD_YMAX)
    };

    state.set_site(&Int2D { x, y }, StaticObjectType::FOOD);
}

/// Generate two obstacles, in the form of ellipses made of dense grid cells.
fn generate_obstacles(state: &mut State) {
    /* General formula to calculate an ellipsis, used to draw obstacles.
       x and y define a specific cell
       horizontal and vertical define the ellipsis position (bottom left: 0,0)
       size defines the ellipsis' size (smaller value = bigger ellipsis)
    */
    let ellipsis = |x: f32, y: f32, horizontal: f32, vertical: f32, size: f32| -> bool {
        ((x - horizontal) * size + (y - vertical) * size)
            * ((x - horizontal) * size + (y - vertical) * size)
            / 36.
            + ((x - horizontal) * size - (y - vertical) * size)
                * ((x - horizontal) * size - (y - vertical) * size)
                / 1024.
            <= 1.
    };
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            if ellipsis(i as f32, j as f32, 100., 145., 0.407)
                || ellipsis(i as f32, j as f32, 90., 55., 0.407)
            {
                state.set_obstacle(&Int2D { x: i, y: j });
            }
        }
    }
}

/// Generate our ant agents, by creating them in the nest.
fn generate_ants(state: &mut State, schedule: &mut Schedule<Ant>) {
    for ant_id in 0..NUM_AGENT {
        let x = (HOME_XMAX + HOME_XMIN) / 2;
        let y = (HOME_YMAX + HOME_YMIN) / 2;
        let loc = Int2D { x, y };
        // Generate the ant with an initial reward of 1, so that it starts spreading home pheromones
        // around the nest, the initial spawn point.
        let mut ant = Ant::new(ant_id, loc, false, 1.);
        state.set_ant_location(&mut ant, &loc);
        schedule.schedule_repeating(ant, 0., 0);
    }
}
