use bevy::prelude::Component;

use crate::engine::location::{Int2D, Real2D};

// TODO improve naming?
#[derive(Component, Copy, Clone)]
pub struct Real2DTranslation(pub Real2D);

#[derive(Component, Copy, Clone)]
pub struct Int2DTranslation(pub Int2D);
