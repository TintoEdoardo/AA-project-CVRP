/*
 * This module defines the entities of a graph.
 */

mod geo_vertex_body;
mod edge_body;
mod graph_body;
mod std_vertex_body;

/* Vertex trait definition. */
pub trait VertexTrait
{

    fn get_id(&self) -> u16;

}

/* Edge trait definition. */
pub trait EdgeTrait
{

    fn get_id(&self) -> u16;
    fn get_weight(&self) -> f32;
    fn get_incident_vertex(&self) -> (dyn VertexTrait, dyn VertexTrait);

    fn set_weight(&mut self, weight : f32) -> void;

}

/* Graph trait definition. */
pub struct Graph
{

    vertex_list : Vec<dyn VertexTrait>,
    edge_list   : Vec<dyn EdgeTrait>

}
