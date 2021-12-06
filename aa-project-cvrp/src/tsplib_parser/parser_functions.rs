use nom::{IResult};
use nom::bytes::complete::tag;
use nom::character::complete::{space0, line_ending, multispace1, not_line_ending, multispace0, space1};
use crate::tsplib_parser::keyword_values::{TYPE, EDGE_WEIGHT_TYPE, EDGE_WEIGHT_FORMAT, EDGE_DATA_FORMAT, NODE_COORD_TYPE, DISPLAY_DATE_TYPE};
use crate::tsplib_parser::custom_types::{Coord, Node, EdgeData};
use crate::tsplib_parser::custom_types::Coord::{Coord2d, Coord3d};
use nom::multi::{separated_list0};
use nom::number::complete::{double};
use nom::combinator::{map_opt, opt};
use nom::sequence::{tuple, preceded};
use nom::error::{Error, ErrorKind};
use nom::Err;


pub(crate) fn parse_key_value<'a>(key : &'a str) -> impl Fn(&str) -> IResult<&str, &str> + 'a
{

    move |x|
        {

            /* The result is of type OK((input, Tuple())). */
            let tuple_result: IResult<&str, (&str, &str, &str,  &str, &str, &str)> =
                tuple((multispace0, tag(key), space0, tag(":"), space0, not_line_ending))(x);

            /* Trim from space at the end, if any. */
            let result : IResult<&str, &str> = match tuple_result
            {
                Ok((input, (_, _, _, _, _, value))) => Ok((input, value.trim_end_matches(" "))),
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

/*
pub fn parse_instance_comment<'a>(parsed_input : IResult<&'a str, &str>) -> &'a str
{

    match parsed_input
    {
        Ok((comment_value, _)) => comment_value,
        Err(_) => ""
    }

} */

pub fn parse_instance_dimension(_dimension : &str) -> usize
{

    _dimension.parse::<usize>().unwrap()

}

pub fn parse_instance_capacity(_capacity : &str) -> usize
{

    _capacity.parse::<usize>().unwrap()

}

pub fn parse_instance_edge_weight_type(_edge_weight_type : &str) -> EDGE_WEIGHT_TYPE
{

    match _edge_weight_type
    {

        "EXPLICIT"  => EDGE_WEIGHT_TYPE::EXPLICIT,
        "EUC_2D"    => EDGE_WEIGHT_TYPE::EUC_2D,
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

    let result : Option<EDGE_WEIGHT_FORMAT>;

    match _edge_weight_format
    {
        Some(value) =>
            {
                match value
                {
                    "FUNCTION"       => result = Some(EDGE_WEIGHT_FORMAT::FUNCTION),
                    "FULL_MATRIX"    => result = Some(EDGE_WEIGHT_FORMAT::FULL_MATRIX),
                    "UPPER_ROW"      => result = Some(EDGE_WEIGHT_FORMAT::UPPER_ROW),
                    "LOWER_ROW"      => result = Some(EDGE_WEIGHT_FORMAT::LOWER_ROW),
                    "UPPER_DIAG_ROW" => result = Some(EDGE_WEIGHT_FORMAT::UPPER_DIAG_ROW),
                    "LOWER_DIAG_ROW" => result = Some(EDGE_WEIGHT_FORMAT::LOWER_DIAG_ROW),
                    "UPPER_COL"      => result = Some(EDGE_WEIGHT_FORMAT::UPPER_COL),
                    "LOWER_COL"      => result = Some(EDGE_WEIGHT_FORMAT::LOWER_COL),
                    "UPPER_DIAG_COL" => result = Some(EDGE_WEIGHT_FORMAT::LOWER_DIAG_COL),
                    "LOWER_DIAG_COL" => result = Some(EDGE_WEIGHT_FORMAT::LOWER_DIAG_COL),
                    _ => result = None
                }
            }
        _ => result = None,
    }

    return result;

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

pub fn parse_instance_node_coord_type(_node_coord_type : Option<&str>) -> NODE_COORD_TYPE
{

    match _node_coord_type
    {
        Some("TWOD_COORDS")    => NODE_COORD_TYPE::TWOD_COORDS,
        Some("THREED_COORDS")  => NODE_COORD_TYPE::THREED_COORDS,
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

pub(crate) fn parse_section_name<'a>(key : &'a str) -> impl Fn(&str) -> IResult<&str, &str> + 'a
{

    move |x|
        {

            /* The result is of type OK((input, Tuple())). */
            let tuple_result: IResult<&str, (&str, &str, &str)> =
                tuple((multispace0, tag(key), space0))(x);

            let result : IResult<&str, &str> = match tuple_result
            {
                Ok((input, (_, value, _))) => Ok((input, value)),
                _ => Err(Err::Error(Error::new("", nom::error::ErrorKind::Tag))),
            };

            return result

        }

}

/* Functions for parsing instance sections. */
pub fn parse_instance_section<'a, T: 'a>(
    section_name : &'a str,
    line_parser  : fn(Vec<f64>) -> Option<T>)
    -> impl Fn(&'a str)
    -> IResult<&str, Vec<T>> + 'a
{

    move |section|
        {

            let mut is_err : bool                = false;
            let key_value  : IResult<&str, &str> = parse_section_name(section_name)(section);

            let section_res : IResult<&str, Vec<T>>;
            match key_value
            {
                Ok((sec, _)) =>
                    {
                        let fist_parsed_data : IResult<&str, (&str, &str)> =
                            tuple((line_ending, space0))(sec);

                        section_res = match fist_parsed_data
                        {
                            Ok((value_list, (_, _))) =>
                                separated_list0(multispace1,
                                                map_opt(separated_list0(space1, double), line_parser))(value_list),
                            _ => Err(nom::Err::Error(Error { input : "", code: ErrorKind::SeparatedList }))
                        };
                    }
                Err(_) => section_res = Err(nom::Err::Error(Error { input : "", code: ErrorKind::SeparatedList })),
            }

            let remaining_input : IResult<&str, Option<&str>>;
            let section_vec     : Vec<T>;
            match section_res
            {
                Ok((r_input, vec_t)) =>
                    {
                        remaining_input =
                            preceded(opt(space0),
                                     preceded(
                                         multispace0,
                                         opt(tag("-1"))))
                                (r_input);

                        section_vec = vec_t;
                    }
                Err(_) =>
                    {
                        remaining_input =
                            Err(nom::Err::Error(Error { input : "", code: ErrorKind::Fail }));

                        section_vec = Vec::new();
                        is_err      = true;
                    }
            }

            let r_input : &str = match remaining_input
            {
                Ok((r_in, _)) => r_in,
                Err(_) => "",
            };

            return if is_err
            {
                Err(nom::Err::Error(Error { input: "", code: ErrorKind::Fail }))
            } else {
                Ok((r_input, section_vec))
            }
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

    /* Initialize sorted_node_coord. */
    for _ in 0..sorted_node_coord.capacity()
    {
        sorted_node_coord.push(Coord::Coord2d((0, 0, 0)));
    }

    if !node_coord.is_none()
    {

        let n_coord : Vec<Coord> = match node_coord
        {
            Some(vec_cord) => vec_cord.clone(),
            _ => Vec::new()
        };

        for n_i in n_coord
        {
            let i : usize = match n_i
            {
                Coord2d((id, _, _)) => id,
                Coord3d((id, _, _, _)) => id,
            };

            /* n_i.0 contains the id of the node. */
            sorted_node_coord[i - 1] = n_i;

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

    /* Initialize sorted_node_demand. */
    for _ in 0..sorted_node_demand.capacity()
    {
        sorted_node_demand.push((0, 0));
    }

    if !demand_vector.is_none()
    {

        let dem_vector : Vec<(Node, usize)> = match demand_vector
        {
            Some(d_vec) => d_vec.clone(),
            _ => Vec::new()
        };

        for d_i in dem_vector
        {
            /* n_i.0 contains the id of the node. */
            sorted_node_demand[d_i.0 - 1] = d_i;
        }

        result = Some(sorted_node_demand);

    }

    return result;

}

/*
pub fn order_edge_list(
    edge_data : &Option< Vec<EdgeData>>)
    -> Option< Vec<Edge>>
{

    let result : Option< Vec<Edge>>;

    result = match edge_data
    {

        Some(edge_data_values) => {

            let edge_number      : usize     = edge_data_values.len();
            let mut sorted_edges : Vec<Edge> = Vec::with_capacity(edge_number);

            for i in 0..edge_number
            {

                let edge_data_i : EdgeData = edge_data_values[i].clone();
                match edge_data_i
                {
                    EdgeData::Edge((n1, n2)) => sorted_edges[i] = (n1, n2),
                    EdgeData::Adj(_) => {}
                };

            }

            sorted_edges.sort_by_key(|k_v| k_v.0);

            Some(sorted_edges)

        }
        _ => None,

    };

    return result;

}
*/


/*
pub fn order_adj_list(
    edge_data : &Option< Vec<EdgeData>>,
    dimension : usize)
    -> Option< Vec<Adj>>
{

    let result : Option< Vec<Adj>>;

    result = match edge_data
    {
        Some(edge_data_values) => {

            let mut sorted_edge_data : Vec<Adj>
                = Vec::with_capacity(dimension);

            let mut adj_vec: Vec<Adj> =
                Vec::with_capacity(dimension);
            for i in 0..dimension
            {

                let edge_data_i : EdgeData = edge_data_values[i].clone();

                match edge_data_i
                {
                    EdgeData::Adj(node_vec ) => adj_vec[i] = node_vec,
                    _ => {}
                };

            }

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
*/

pub fn compute_edge_weight_matrix(
    edge_weight_section : Option< Vec< Vec<usize>>>,
    edge_weight_format  : EDGE_WEIGHT_FORMAT,
    dimension           : usize)
    -> Option< Vec< Vec<usize>>>
{

    let index_matrix : Vec<Vec<usize>>
        = match edge_weight_format
    {
        EDGE_WEIGHT_FORMAT::FULL_MATRIX    => (0..dimension).map(|_| (0..dimension).collect()).collect(),
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
    let mut edge_weight_iter; // : Iter<usize>;
    let result               : Option< Vec< Vec<usize>>>;

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
 */