/* Custom types used to model the
 * instance graph. */

#[allow(dead_code)]
pub type Node = usize;

#[allow(dead_code)]
pub type Edge = (Node, Node);

#[allow(dead_code)]
pub type Adj  = Vec<Node>;

#[derive(Clone)]
pub enum Coord
{

    Coord2d((usize, usize, usize)),
    Coord3d((usize, usize, usize, usize)),

}

#[derive(Clone)]
pub enum EdgeData
{

    Edge((Node, Node)),
    Adj(Vec<Node>),

}
