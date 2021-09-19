/*
 * This module implements the Edge.
 *
 * EDGE:
 *  - Methods:
 *    GET_ID: (&self) -> u16
 *    GET_WEIGHT: (&self) -> f32
 *    GET_INCIDENT_VERTEX: (&self) -> f32
 *    SET_WEIGHT: (&mut self, f32) -> void
 */

use crate::model::{EdgeTrait, VertexTrait};

struct Edge
{

    id              : u16,
    incident_vertex : std::vec<dyn VertexTrait>,
    weight          : f32

}

impl EdgeTrait for Edge
{

    fn get_id(&self) -> u16
    {

        self.id

    }

    fn get_weight(&self) -> f32
    {

        self.weight

    }

    fn get_incident_vertex(&self) -> std::vec<dyn VertexTrait>
    {

        self.incident_vertex

    }

    fn set_weight(&mut self, w: f32)
    {

        self.weight = w;

    }


}
