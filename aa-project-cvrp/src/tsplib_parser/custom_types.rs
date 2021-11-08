/* Custom types used to model the
 * instance graph. */

pub type Node = usize;

pub type Edge = (Node, Node);

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
