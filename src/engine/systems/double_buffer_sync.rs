use bevy::prelude::{Component, Query};

use crate::engine::components::double_buffer::{DBRead, DBWrite};

pub fn double_buffer_sync<T: Component + Copy + Send>(
    mut query: Query<(&mut DBRead<T>, &DBWrite<T>)>,
) {
    // TODO parallelize
    /*query.par_for_each_mut(50000/8, |(mut read, write)| {
        read.0 = write.0;
    });*/
    for (mut read, write) in &mut query {
        read.0 = write.0;
    }
}
