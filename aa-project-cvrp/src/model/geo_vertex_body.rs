/*
 * This module implements a GeoVertex.
 *
 * GEO_VERTEX:
 *  - Methods:
 *    GET_ID: (&self) -> u16
 *    GET_LATITUDE: (&self) -> f32
 *    GET_LONGITUDE: (&self) -> f32
 */

use crate::model::VertexTrait;

struct GeoVertex
{

    id        : u16,
    longitude : f32,
    latitude  : f32

}

/* GeoVertex implements the Vertex trait. */
impl VertexTrait for GeoVertex
{

    fn get_id(&self) -> u16
    {

        self.id

    }

}

/* GeoVertex extends the Vertex trait
 * with two methods.               */
impl GeoVertex
{

    pub fn get_latitude (&self) -> f32 {
        self.latitude
    }

    pub fn get_longitude (&self) -> f32 {
        self.longitude
    }

}
