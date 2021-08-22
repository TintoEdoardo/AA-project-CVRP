//
//  This module define the graph structure
//

mod vertex_body;
mod edge_body;
mod graph_body;

// Vertex definition
struct Vertex {
    longitude : f32,
    latitude  : f32
}

// Edge definition
struct Edge {
    vertex_couple : (Vertex, Vertex),
    weight        : f32
}

struct Graph {
    vertex_list : Vec<Vertex>,
    edge_list   : Vec<Edge>
}
