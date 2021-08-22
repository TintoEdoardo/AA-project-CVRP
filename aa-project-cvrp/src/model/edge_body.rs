use crate::model::{Edge, Vertex};

impl Edge {

    // Constructor for Edge
    pub fn create_edge (x : Vertex, y : Vertex) -> Edge {
        let vc = (x, y);
        // Create an edge with immutable incident vertex
        Edge {
            vertex_couple: vc,
            weight: 0.0
        }
    }

    pub fn set_weight (&mut self, w : f32) {
        self.weight = w;
    }

    pub fn get_weight (&self) -> f32 {
        self.weight
    }

    pub fn get_incident_vertex (&self) -> (Vertex, Vertex) {
        (self.vertex_couple.0, self.vertex_couple.1)
    }
}