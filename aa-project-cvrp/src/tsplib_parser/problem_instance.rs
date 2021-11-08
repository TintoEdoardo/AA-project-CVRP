/*
 * Instance for the problem derived
 * from the specification file.
 */

use crate::tsplib_parser::keyword_values;
use crate::tsplib_parser::custom_types::{Coord, Node, EdgeData};

#[derive(Clone)]
pub(crate) struct Specification
{
    pub(crate) name      : &'static str,
    pub(crate) data_type : keyword_values::TYPE,
    pub(crate) comment   : Vec<&'static str>,
    pub(crate) dimension : usize,
    pub(crate) capacity  : usize,
    pub(crate) edge_weight_type   : keyword_values::EDGE_WEIGHT_TYPE,
    pub(crate) edge_weight_format : Option< keyword_values::EDGE_WEIGHT_FORMAT>,
    pub(crate) edge_data_format   : Option< keyword_values::EDGE_DATA_FORMAT>,
    pub(crate) display_data_type  : keyword_values::DISPLAY_DATE_TYPE,
    pub(crate) node_coord_type    : keyword_values::NODE_COORD_TYPE,

}

#[derive(Clone)]
pub(crate) struct Data
{
    pub(crate) node_coord_section   : Option< Vec<Coord>>,
    pub(crate) depot_section        : Option< Vec<Node>>,
    pub(crate) demand_section       : Option< Vec<(Node, usize)>>,
    pub(crate) edge_data_section    : Option< Vec<EdgeData>>,
    pub(crate) fixed_edges_section  : Option< Vec<EdgeData>>,
    // display_data_section :
    pub(crate) tour_section         : Option< Vec< Vec<Node>>>,
    pub(crate) edge_weight_section  : Option< Vec< Vec<usize>>>,

}

pub struct TSPInstance
{

    /* Specification section. */
    pub(crate) specification : Specification,

    /* Data section. */
    pub(crate) data : Data,

}
