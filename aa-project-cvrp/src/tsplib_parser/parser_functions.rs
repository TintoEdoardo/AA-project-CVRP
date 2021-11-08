use nom::{IResult};
use nom::bytes::complete::tag;
use nom::character::complete::{space0, line_ending, multispace1, not_line_ending, multispace0};
use crate::tsplib_parser::keyword_values::{TYPE, EDGE_WEIGHT_TYPE, EDGE_WEIGHT_FORMAT, EDGE_DATA_FORMAT, NODE_COORD_TYPE, DISPLAY_DATE_TYPE};
use crate::tsplib_parser::custom_types::{Coord, Edge, Node, Adj, EdgeData};
use crate::tsplib_parser::custom_types::Coord::{Coord2d, Coord3d};
use nom::multi::{separated_list0};
use nom::number::complete::{double};
use nom::combinator::{map_opt, opt};
use nom::sequence::{tuple};
use nom::error::Error;
use nom::Err;
use std::slice::Iter;


pub(crate) fn parse_key_value<'a>(key : &'a str) -> impl Fn(&str) -> IResult<&str, &str> + 'a
{

    move |x|
        {

            /* The result is of type OK((input, Tuple())). */
            let tuple_result: IResult<&str, (&str, &str, &str,  &str, &str, &str)> =
                tuple((multispace0, tag(key), space0, tag(":"), space0, not_line_ending))(x);

            let result : IResult<&str, &str> = match tuple_result
            {
                Ok((input, (_, _, _, _, _, value))) => Ok((input, value)),
                _ => Err(Err::Error(Error::new("", nom::error::ErrorKind::Tag))),
            };

            return result

        }

}

/* Functions invoked by the parser to
 * analyze the specification sections of
 * the instance received as input. */
/*pub fn parse_instance_name<'a>(parsed_input : IResult<&str, &str>) -> &'a str
{

    match parsed_input
    {
        Ok((name_value, _)) => name_value,
        Err(_) => ""
    }

}*/

pub fn parse_instance_type(_type : &str) -> TYPE
{

    match _type
    {
        "TSP"  => TYPE::TSP,
        "ATSP" => TYPE::ATSP,
        "SOP"  => TYPE::SOP,
        "HCP"  => TYPE::HCP,
        "CVRP" => TYPE::CVRP,
        "TOUR" => TYPE::TOUR,
        _ => TYPE::TSP // We assume TSP by default.
    }

}

pub fn parse_instance_comment<'a>(parsed_input : IResult<&'a str, &str>) -> &'a str
{

    match parsed_input
    {
        Ok((comment_value, _)) => comment_value,
        Err(_) => ""
    }

}

pub fn parse_instance_dimension(_dimension : &str) -> usize
{

    _dimension.parse::<usize>().unwrap()

}

pub fn parse_instance_capacity(parsed_input : IResult<&str, &str>) -> usize
{

    match parsed_input
    {

        Ok((capacity_value, _)) => capacity_value.parse::<usize>().unwrap(),
        _ => 0

    }

}

pub fn parse_instance_edge_weight_type(_edge_weight_type : &str) -> EDGE_WEIGHT_TYPE
{

    match _edge_weight_type
    {

        "EXPLICIT"  => EDGE_WEIGHT_TYPE::EXPLICIT,
        "EUC_2D"    => EDGE_WEIGHT_TYPE::CEIL_2D,
        "EUC_3D"    => EDGE_WEIGHT_TYPE::EUC_3D,
        "MAX_2D"    => EDGE_WEIGHT_TYPE::MAX_2D,
        "MAX_3D"    => EDGE_WEIGHT_TYPE::MAX_3D,
        "MAN_2D"    => EDGE_WEIGHT_TYPE::MAN_2D,
        "MAN_3D"    => EDGE_WEIGHT_TYPE::MAN_3D,
        "CEIL_2D"   => EDGE_WEIGHT_TYPE::CEIL_2D,
        "GEO"       => EDGE_WEIGHT_TYPE::GEO,
        "ATT"       => EDGE_WEIGHT_TYPE::ATT,
        "XRAY1"     => EDGE_WEIGHT_TYPE::XRAY1,
        "XRAY2"     => EDGE_WEIGHT_TYPE::XRAY2,
        "SPECIAL"   => EDGE_WEIGHT_TYPE::SPECIAL,
        _ => EDGE_WEIGHT_TYPE::EXPLICIT,

    }

}

pub fn parse_instance_edge_weight_format(_edge_weight_format : Option<&str>) -> Option<EDGE_WEIGHT_FORMAT>
{

    match _edge_weight_format
    {
        Some(value) =>
            {
                match value
                {
                    "FUNCTION"       => Some(EDGE_WEIGHT_FORMAT::FUNCTION),
                    "FULLL_MATRIX"   => Some(EDGE_WEIGHT_FORMAT::FULL_MATRIX),
                    "UPPER_ROW"      => Some(EDGE_WEIGHT_FORMAT::UPPER_ROW),
                    "LOWER_ROW"      => Some(EDGE_WEIGHT_FORMAT::LOWER_ROW),
                    "UPPER_DIAG_ROW" => Some(EDGE_WEIGHT_FORMAT::UPPER_DIAG_ROW),
                    "LOWER_DIAG_ROW" => Some(EDGE_WEIGHT_FORMAT::LOWER_DIAG_ROW),
                    "UPPER_COL"      => Some(EDGE_WEIGHT_FORMAT::UPPER_COL),
                    "LOWER_COL"      => Some(EDGE_WEIGHT_FORMAT::LOWER_COL),
                    "UPPER_DIAG_COL" => Some(EDGE_WEIGHT_FORMAT::LOWER_DIAG_COL),
                    "LOWER_DIAG_COL" => Some(EDGE_WEIGHT_FORMAT::LOWER_DIAG_COL),
                    _ => None
                }
            }
        _ => None,
    }


}

pub fn parse_instance_edge_data_format(_edge_data_format : Option<&str>) -> Option<EDGE_DATA_FORMAT>
{

    match _edge_data_format
    {

        Some(edge_data_format_value) =>
            {
                match edge_data_format_value
                {
                    "EDGE_LIST" => Some(EDGE_DATA_FORMAT::EDGE_LIST),
                    "ADJ_LIST"  => Some(EDGE_DATA_FORMAT::ADJ_LIST),
                    _ => None
                }
            }

        _ => None

    }

}

pub fn parse_instance_node_coord_type(_node_coord_type : &str) -> NODE_COORD_TYPE
{

    match _node_coord_type
    {
        "TWOD_COORDS" => NODE_COORD_TYPE::TWOD_COORDS,
        "THREED_COORDS"  => NODE_COORD_TYPE::THREED_COORDS,
        _ => NODE_COORD_TYPE::NO_COORDS
    }

}

pub fn parse_instance_display_data_type(_display_data_type : Option<&str>) -> DISPLAY_DATE_TYPE
{

    match _display_data_type
    {

        Some(display_data_type_value) =>
            {
                match display_data_type_value
                {
                    "COORD_DISPLAY" => DISPLAY_DATE_TYPE::COORD_DISPLAY,
                    "TWOD_DISPLAY"  => DISPLAY_DATE_TYPE::TWOD_DISPLAY,
                    _ => DISPLAY_DATE_TYPE::NO_DISPLAY
                }
            }

        _ => DISPLAY_DATE_TYPE::NO_DISPLAY

    }

}

/* Functions for parsing the data sections
 * of the instance passed as input. */

/* These functions are use to parse a single
 * input line.
 * They are composed inside the functions for
 * parsing the instance sections. */
pub fn parse_coord_2d(coord : Vec<f64>) -> Option<Coord>
{

    match coord.len()
    {
        3 => Some(Coord2d((coord[0] as usize,
                          coord[1] as usize,
                          coord[2] as usize))),
        _ => None,
    }

}

pub fn parse_coord_3d(coord : Vec<f64>) -> Option<Coord>
{

    match coord.len()
    {
        4 => Some(Coord3d((coord[0] as usize,
                          coord[1] as usize,
                          coord[2] as usize,
                          coord[3] as usize))),
        _ => None,
    }

}

pub fn parse_depot(depot : Vec<f64>) -> Option<usize>
{

    match depot.len()
    {
        1 => Some(depot[0] as usize),
        _ => None,
    }

}

pub fn parse_node_demand(node_demand : Vec<f64>) -> Option<(usize, usize)>
{

    match node_demand.len()
    {
        2 => Some((node_demand[0] as usize,
                   node_demand[1] as usize)),
        _ => None,
    }

}

pub fn parse_edge(edge : Vec<f64>) -> Option<EdgeData>
{

    match edge.len()
    {
        2 => Some(EdgeData::Edge((edge[0] as usize,
                       edge[1] as usize))),
        _ => None,
    }

}

pub fn parse_adj_vec(node_vec : Vec<f64>) -> Option<EdgeData>
{

    match node_vec.len()
    {

        0 => None,
        1 => None,
        _ => Some(EdgeData::Adj(node_vec
            .iter()
            .map(|node| *node as usize)
            .collect::<Vec<usize>>())),

    }

}

pub fn parse_tour(tour_vec : Vec<f64>) -> Option< Vec<usize>>
{

    match tour_vec.len()
    {

        0 => None,
        _ => Some(tour_vec
            .iter()
            .map(|node| *node as usize)
            .collect::<Vec<usize>>()),
    }

}

pub fn parse_edge_weight(edge_weight : Vec<f64>) -> Option< Vec<usize>>
{

    match edge_weight.len()
    {
        0 => None,
        _ => Some(edge_weight
            .iter()
            .map(|ew| *ew as usize)
            .collect::<Vec<usize>>())
    }

}

/* Functions for parsing instance sections. */
pub fn parse_instance_section<'a, T: 'a>(
    section_name : &'a str,
    line_parser : fn(Vec<f64>) -> Option<T>)
    -> impl Fn(&'a str)
    -> IResult<&str, Vec<T>> + 'a
{

    move |section|
        {

            let key_value : IResult<&str, &str> = parse_key_value(section_name)(section);
            let value     : IResult<&str, Vec<T>> =
                key_value
                    .and_then(|(_, sec)| line_ending(sec))
                    .and_then(|(_, sec)| space0(sec))
                    .and_then(|(_, sec)|
                        separated_list0(
                            multispace1,
                            map_opt(separated_list0(space0, double), line_parser))
                            (sec));

            let (input, section) : (&str, Vec<T>) = value.unwrap_or(("", Vec::new()));

            let remaining_input : IResult<&str, &str> =
                space0(input)
                    .and_then(|(_, r_in)| opt(line_ending)(r_in))
                    .and_then(|(_, Some(r_in))| opt(tag("-1"))(r_in))
                    .and_then(|(_, Some(r_in))| Ok((r_in, "")));

            let (r_input, _) : (&str, &str) = remaining_input.unwrap_or(("", ""));

            return Ok((r_input, section));

        }

}

/* In order to optimize the access to the
 * data information, this vector should be
 * properly sorted. */
pub fn order_node_coord(
    node_coord : &Option<Vec<Coord>>,
    dimension  : &usize)
    -> Option<Vec<Coord>>
{

    let mut result            : Option<Vec<Coord>> = None;
    let mut sorted_node_coord : Vec<Coord>         = Vec::with_capacity(*dimension);

    if !node_coord.is_none()
    {

        for n_i in node_coord.unwrap()
        {
            let i : usize = match n_i
            {
                Coord2d((id, _, _)) => id,
                Coord3d((id, _, _, _)) => id,
            };
            /* n_i.0 contains the id of the node. */
            sorted_node_coord[i] = n_i;
        }

        result = Some(sorted_node_coord);

    }

    return result;

}

pub fn order_node_demand_section(
    demand_vector : &Option<Vec<(Node, usize)>>,
    dimension     : &usize)
    -> Option<Vec<(Node, usize)>>
{

    let mut result             : Option<Vec<(Node, usize)>> = None;
    let mut sorted_node_demand : Vec<(Node, usize)> = Vec::with_capacity(*dimension);

    if !demand_vector.is_none()
    {

        for d_i in demand_vector.unwrap()
        {
            /* n_i.0 contains the id of the node. */
            sorted_node_demand[d_i.0] = d_i;
        }

        result = Some(sorted_node_demand);

    }

    return result;

}

pub fn order_edge_list(
    edge_data : &Option< Vec<EdgeData>>)
    -> Option< Vec<Edge>>
{

    let mut result : Option< Vec<Edge>> = None;

    result = match edge_data
    {

        Some(edge_data_values) => {

            let mut sorted_edges : Vec<Edge>
                = edge_data_values
                .iter()
                .map(|EdgeData::Edge((n1, n2))| ((*n1, *n2)))
                .collect();

            sorted_edges.sort_by_key(|k_v| k_v.0);

            Some(sorted_edges)

        }
        _ => None,

    };

    return result;

}



pub fn order_adj_list(
    edge_data : &Option< Vec<EdgeData>>,
    dimension : usize)
    -> Option< Vec<Adj>>
{

    let mut result : Option< Vec<Adj>> = None;

    result = match edge_data
    {
        Some(edge_data_values) => {

            let mut sorted_edge_data : Vec<Adj>
                = Vec::with_capacity(dimension);

            let mut adj_vec : Vec<Adj>
                = edge_data_values
                .iter()
                .map(|EdgeData::Adj(vec)| *vec)
                .collect();

            for i in 0..dimension
            {

                let adj_vec_i  : Adj
                    = adj_vec[i].clone();
                let node_id   : Node = adj_vec_i[0];

                sorted_edge_data[node_id] = adj_vec_i;
                sorted_edge_data[node_id].sort();

            }

            Some(sorted_edge_data)

        }
        _ => None,

    };

    return result;

}

pub fn compute_edge_weight_matrix(
    edge_weight_section : Option< Vec< Vec<usize>>>,
    edge_weight_format  : EDGE_WEIGHT_FORMAT,
    dimension           : usize)
    -> Option< Vec< Vec<usize>>>
{

    let index_matrix : Vec<Vec<usize>>
        = match edge_weight_format
    {
        EDGE_WEIGHT_FORMAT::FULL_MATRIX    => (0..dimension).map(|index| (0..dimension).collect()).collect(),
        EDGE_WEIGHT_FORMAT::LOWER_ROW      => (1..dimension).map(|index| (0..(index - 1)).collect()).collect(),
        EDGE_WEIGHT_FORMAT::UPPER_COL      => (1..dimension).map(|index| (0..(index - 1)).collect()).collect(),
        EDGE_WEIGHT_FORMAT::LOWER_DIAG_ROW => (0..dimension).map(|index| (0..index).collect()).collect(),
        EDGE_WEIGHT_FORMAT::UPPER_DIAG_COL => (0..dimension).map(|index| (0..index).collect()).collect(),
        EDGE_WEIGHT_FORMAT::UPPER_DIAG_ROW => (0..dimension).map(|index| (index..dimension).collect()).collect(),
        EDGE_WEIGHT_FORMAT::LOWER_DIAG_COL => (0..dimension).map(|index| (index..dimension).collect()).collect(),
        EDGE_WEIGHT_FORMAT::UPPER_ROW      => (0..(dimension - 1))
            .map(|index| ((index + 1)..dimension).collect()).collect(),
        EDGE_WEIGHT_FORMAT::LOWER_COL      => (0..(dimension - 1))
            .map(|index| ((index + 1)..dimension).collect()).collect(),
        _ => Vec::new(),
    };

    let mut weight_matrix    : Vec< Vec<usize>> = index_matrix;
    let mut edge_weight_iter : Iter<usize>;
    let mut result           : Option< Vec< Vec<usize>>>;

    match edge_weight_section
    {
        Some(e_w_sec) =>
            {
                let e_w_sec_vec_ref : Vec<&usize> = e_w_sec.iter().flatten().collect::<Vec<&usize>>();
                let e_w_sec_vec     : Vec<usize>  = e_w_sec_vec_ref.iter().map(|u_ref| **u_ref).collect();
                edge_weight_iter = e_w_sec_vec.iter();
                for i in 0..weight_matrix.len()
                {
                    for j in 0..weight_matrix[i].len()
                    {
                        weight_matrix[i][j] = *edge_weight_iter.next().unwrap();
                    }
                }

                result = Some(weight_matrix);

            }
        _ =>
            {
                result = None;
            }
    }

    return result;
}

/*
    TODO:
        FIXED_EDGE_SECTION
        DISPLAY_DATA_SECTION
 */