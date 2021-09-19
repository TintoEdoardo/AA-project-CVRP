/*
 * Instance for the problem derived
 * from the specification file.
 */

use crate::parser::keyword_values;
use crate::model::Graph;


struct TspProblem
{

    /* Specification. */
    name      : String,
    data_type : keyword_values::TYPE,
    comment   : String,
    dimension : u8,
    edge_weight_type   : keyword_values::EDGE_WEIGHT_TYPE,
    edge_weight_format : keyword_values::EDGE_WEIGHT_FORMAT,
    edge_data_format   : keyword_values::EDGE_DATA_FORMAT,
    display_data_type  : keyword_values::DISPLAY_DATE_TYPE,
    node_coord_type    : keyword_values::NODE_COORD_TYPE,

    /* Data. */
    graph_instance : Graph

}


impl TspProblem
{

    /* Setter functions. */
    pub fn set_name (&mut self, _name : String) -> void
    {

        self.name = _name

    }

    pub fn set_data_type (&mut self, _type : keyword_values::TYPE) -> void
    {

        self.data_type = _type

    }

    pub fn set_comment (&mut self, _comment : String) -> void
    {

        self.comment = _comment

    }

    pub fn set_dimension (&mut self, _dim : u8) -> void
    {

        self.dimension = _dim

    }

    pub fn set_edge_weight_type (&mut self, _edge_w_t : keyword_values::EDGE_WEIGHT_TYPE) -> void
    {

        self.edge_weight_type = _edge_w_t

    }

    pub fn set_edge_weight_format (&mut self, _edge_w_f : keyword_values::EDGE_WEIGHT_FORMAT) -> void
    {

        self.edge_weight_format = _edge_w_f

    }

    pub fn set_edge_data_format (&mut self, _edge_d_f : keyword_values::EDGE_DATA_FORMAT) -> void
    {

        self.edge_data_format = _edge_d_f

    }

    pub fn set_display_data_type (&mut self, _display_d_t : keyword_values::DISPLAY_DATE_TYPE) -> void
    {

        self.display_data_type = _display_d_t

    }

    pub fn set_node_coord_type (&mut self, _node_c_t : keyword_values::NODE_COORD_TYPE) -> void
    {

        self.node_coord_type = _node_c_t

    }

    pub fn set_graph (&mut self, &graph : Graph) -> void
    {
        self.graph_instance = graph
    }


    /* Getter functions. */
    pub fn get_name (&self) -> &str
    {

        &self.name

    }

    pub fn get_data_type (&self) -> &keyword_values::TYPE
    {

        &self.data_type

    }

    pub fn get_comment (&self) -> &str
    {

        &self.comment

    }

    pub fn get_dimension (&self) -> u8
    {

        self.dimension

    }

    pub fn get_edge_weight_type(&self) -> &keyword_values::EDGE_WEIGHT_TYPE
    {

        &self.edge_weight_type

    }

    pub fn get_edge_weight_format(&self) -> &keyword_values::EDGE_WEIGHT_FORMAT
    {

        &self.edge_weight_format

    }

    pub fn get_edge_data_format(&self) -> &keyword_values::EDGE_DATA_FORMAT
    {

        &self.edge_data_format

    }

    pub fn get_display_data_type (&self) -> &keyword_values::DISPLAY_DATE_TYPE
    {

        &self.display_data_type

    }

    pub fn get_node_coord_type(&self) -> &keyword_values::NODE_COORD_TYPE
    {

        &self.node_coord_type

    }

    pub fn get_graph(&self) -> &Graph
    {
        &self.graph_instance
    }

}