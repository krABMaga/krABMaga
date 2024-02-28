use super::id::LayerId;

#[derive(Clone)]
pub struct Layer {
    pub id: LayerId,
    pub file_name: String,
    pub geom_type: geo_types::Geometry,
    pub visible: bool,
}

#[derive(Clone)]
pub struct AllLayers {
    pub layers: Vec<Layer>,
    pub selected_layer_id: i32,
}

impl AllLayers {
    pub fn new() -> AllLayers {
        AllLayers {
            layers: vec![],
            selected_layer_id: 0,
        }
    }

    fn next_layer_id(&self) -> LayerId {
        LayerId::new(self.last_layer_id())
    }

    pub fn add(&mut self, geometry: geo_types::Geometry, file_name: String) {
        let id = self.next_layer_id();
        let layer = Layer {
            id,
            file_name,
            visible: false,
            geom_type: geometry,
        };

        self.layers.push(layer);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Layer> {
        self.layers.iter()
    }

    pub fn last_layer_id(&self) -> i32 {
        if self.layers.len() == 0 {
            return 0;
        }

        self.layers.last().unwrap().id.get_id()
    }
}
