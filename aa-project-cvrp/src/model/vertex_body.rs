use crate::model::Vertex;

impl Vertex {

    // Constructor for Vertex
    pub fn create_vertex(lat : f32, lon : f32) -> Vertex {
        Vertex {
            longitude : lon,
            latitude  : lat
        }
    }

    pub fn get_latitude (&self) -> f32 {
        self.latitude
    }

    pub fn get_longitude (&self) -> f32 {
        self.longitude
    }

}
