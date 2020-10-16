
// Wrapper structs over Grid2D/SimpleGrid2D s to handle them as Amethyst resources.
pub use self::ants_grid::AntsGrid;
pub use self::obstacles_grid::ObstaclesGrid;
pub use self::sites_grid::SitesGrid;
pub use self::to_home_grid::ToHomeGrid;
pub use self::to_food_grid::ToFoodGrid;

mod ants_grid;
mod obstacles_grid;
mod sites_grid;
mod to_food_grid;
mod to_home_grid;