#[derive(
    Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, bevy::ecs::component::Component,
)]
pub struct LayerId(i32);

impl Default for LayerId {
    fn default() -> Self {
        Self::new(-1)
    }
}

impl LayerId {
    pub fn new(last: i32) -> Self {
        LayerId(new_id(last))
    }

    pub fn get_id(&self) -> i32 {
        self.0
    }
}

pub fn new_id(last: i32) -> i32 {
    last + 1
}
