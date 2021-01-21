/// Simple enum describing a type of object that does not change location over time.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum StaticObjectType {
    HOME,
    FOOD,
    OBSTACLE,
}
