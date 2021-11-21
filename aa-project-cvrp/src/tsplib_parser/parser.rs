/*

 */

use nom::combinator::opt;
use nom::branch::permutation;
use crate::tsplib_parser::keywords;
use crate::tsplib_parser::keyword_values;
use crate::tsplib_parser::problem_instance::{TSPInstance, Specification, Data};
use crate::tsplib_parser::parser_functions::{parse_key_value, parse_instance_type, parse_instance_dimension, parse_instance_edge_weight_type, parse_instance_edge_weight_format, parse_instance_edge_data_format, parse_instance_display_data_type, parse_instance_node_coord_type, parse_instance_section, parse_coord_3d, parse_depot, parse_coord_2d, parse_edge, parse_adj_vec, parse_node_demand, parse_tour, parse_edge_weight, order_node_coord, order_node_demand_section, compute_edge_weight_matrix, parse_instance_capacity};
use nom::multi::many0;
use nom::{IResult};
use crate::tsplib_parser::custom_types::{Coord, EdgeData};
use nom::error::ErrorKind;
use nom::error::Error;

pub fn parse(input : &str) -> TSPInstance
{

    /* Compute specification section. */
    let specification_result             : IResult<&str, Specification>  =
        parse_specification(input);

    let (remaining_input, specification) : (&str, Option<Specification>) =
        match specification_result
    {
        Ok((input, spec)) => (input, Some(spec)),
        _ => ("", None),
    };

    //DEBUG
    /*match specification.clone()
    {
        Some(s) =>
            {
                println!("Result of '{k}' output is: {ou}", k = "NAME", ou = s.name);
                println!("Result of '{k}' output is: {ou}", k = "DIMENSION", ou = s.dimension);
                println!("Result of '{k}' output is: {ou}", k = "EDGE_WEIGHT_TYPE", ou = (s.edge_weight_type == EUC_2D));
                println!("Result of '{k}' output is: {ou}", k = "TYPE", ou = s.data_type == CVRP);
                println!("Result of '{k}' output is: {ou}", k = "COMMENT", ou = s.comment.concat());
                println!("Result of '{k}' output is: {ou}", k = "CAPACITY", ou = s.capacity);
                println!("Result of '{k}' output is: {ou}", k = "DISPLAY", ou = s.display_data_type == NO_DISPLAY);
                println!("Result of '{k}' output is: {ou}", k = "NODE_COORD_TYPE", ou = s.node_coord_type == NO_COORDS);

            }
        None => println!("Specification is None"),
    }*/

    /* Compute data section. */
    let data_result : IResult<&str, Data> =
        parse_data(&remaining_input, &specification.as_ref().unwrap());
    let data : Option<Data> = match data_result
    {
        Ok((_, dat)) => Some(dat),
        _ => None,
    };

    return TSPInstance
    {
        specification : specification.unwrap().clone(),
        data          : data.unwrap().clone(),
    };

}

fn parse_specification(input : &str) -> IResult<&str, Specification>
{

    let specification : Specification;
    let data_input    : &str;

    let parsed_specification :
        IResult<&str,
            (&str, &str, Vec<&str>, &str, &str, &str,
             Option<&str>, Option<&str>, Option<&str>, Option<&str>)>
        =
        permutation((
            parse_key_value(&*keywords::NAME),
            parse_key_value(&*keywords::TYPE),
            many0(parse_key_value(&*keywords::COMMENT)),
            parse_key_value(&*keywords::DIMENSION),
            parse_key_value(&*keywords::CAPACITY),
            parse_key_value(&*keywords::EDGE_WEIGHT_TYPE),
            opt(parse_key_value(&*keywords::EDGE_WEIGHT_FORMAT)),
            opt(parse_key_value(&*keywords::EDGE_DATA_FORMAT)),
            opt(parse_key_value(&*keywords::DISPLAY_DATA_TYPE)),
            opt(parse_key_value(&*keywords::NODE_COORD_TYPE))))
            (input);

    return match parsed_specification
    {
        Ok((data_sections, (
            _name,
            _type,
            _comments,
            _dimension,
            _capacity,
            _edge_weight_type,
            _edge_weight_format,
            _edge_data_format,
            _display_data_type,
            _node_coord_type)))
        =>
            {
                specification = Specification
                {
                    name               : _name.clone(),
                    data_type          : parse_instance_type(_type),
                    comment            : _comments.clone(),
                    dimension          : parse_instance_dimension(_dimension),
                    capacity           : parse_instance_capacity(_capacity),
                    edge_weight_type   : parse_instance_edge_weight_type(_edge_weight_type),
                    edge_weight_format : parse_instance_edge_weight_format(_edge_weight_format),
                    edge_data_format   : parse_instance_edge_data_format(_edge_data_format),
                    display_data_type  : parse_instance_display_data_type(_display_data_type),
                    node_coord_type    : parse_instance_node_coord_type(_node_coord_type),
                };

                //DEBUG
                /*{
                    println!("Result of '{k}' output is: {ou}", k = "NAME", ou = specification.name);
                    println!("Result of '{k}' output is: {ou}", k = "DIMENSION", ou = specification.dimension);
                    println!("Result of '{k}' output is: {ou}", k = "EDGE_WEIGHT_TYPE", ou = (specification.edge_weight_type == EUC_2D));
                    println!("Result of '{k}' output is: {ou}", k = "TYPE", ou = specification.data_type == CVRP);
                    println!("Result of '{k}' output is: {ou}", k = "COMMENT", ou = specification.comment.concat());
                    println!("Result of '{k}' output is: {ou}", k = "CAPACITY", ou = specification.capacity);
                    println!("Result of '{k}' output is: {ou}", k = "DISPLAY", ou = specification.display_data_type == NO_DISPLAY);
                    println!("Result of '{k}' output is: {ou}", k = "NODE_COORD_TYPE", ou = specification.node_coord_type == NO_COORDS);

                }*/

                data_input = data_sections;

                Ok((data_input, specification))
            }
        Err(_) =>
            {
                //DEBUG
                println!("Error in specification");
                Err(nom::Err::Error(Error { input, code: ErrorKind::Permutation }))
            },
    }
}

fn parse_data<'a>(input : &'a str, specification : &'a Specification) -> IResult<&'a str, Data>
{

    let data : Data;
    let node_coord_parser : fn(Vec<f64>) -> Option<Coord> =
        match specification.node_coord_type
        {
            keyword_values::NODE_COORD_TYPE::TWOD_COORDS   => parse_coord_2d,
            keyword_values::NODE_COORD_TYPE::THREED_COORDS => parse_coord_3d,
            _ => parse_coord_2d, // By default we assume 2D coords.
        };

    let edge_data_parser : fn(Vec<f64>) -> Option<EdgeData> =
        match specification.edge_data_format
        {
            Some(keyword_values::EDGE_DATA_FORMAT::EDGE_LIST) => parse_edge,
            _  => parse_adj_vec,
        };

    let dimension        : &usize                           = &specification.dimension;

    let edge_weight_format : keyword_values::EDGE_WEIGHT_FORMAT =
        match specification.edge_weight_format
        {
            Some(keyword_values::EDGE_WEIGHT_FORMAT::LOWER_DIAG_COL) => keyword_values::EDGE_WEIGHT_FORMAT::LOWER_DIAG_COL,
            Some(keyword_values::EDGE_WEIGHT_FORMAT::LOWER_COL)      => keyword_values::EDGE_WEIGHT_FORMAT::LOWER_COL,
            Some(keyword_values::EDGE_WEIGHT_FORMAT::UPPER_COL)      => keyword_values::EDGE_WEIGHT_FORMAT::UPPER_COL,
            Some(keyword_values::EDGE_WEIGHT_FORMAT::UPPER_DIAG_COL) => keyword_values::EDGE_WEIGHT_FORMAT::UPPER_DIAG_COL,
            Some(keyword_values::EDGE_WEIGHT_FORMAT::LOWER_ROW)      => keyword_values::EDGE_WEIGHT_FORMAT::LOWER_ROW,
            Some(keyword_values::EDGE_WEIGHT_FORMAT::LOWER_DIAG_ROW) => keyword_values::EDGE_WEIGHT_FORMAT::LOWER_DIAG_ROW,
            Some(keyword_values::EDGE_WEIGHT_FORMAT::UPPER_ROW)      => keyword_values::EDGE_WEIGHT_FORMAT::UPPER_ROW,
            Some(keyword_values::EDGE_WEIGHT_FORMAT::UPPER_DIAG_ROW) => keyword_values::EDGE_WEIGHT_FORMAT::UPPER_DIAG_ROW,
            Some(keyword_values::EDGE_WEIGHT_FORMAT::FULL_MATRIX)    => keyword_values::EDGE_WEIGHT_FORMAT::FULL_MATRIX,
            _ => keyword_values::EDGE_WEIGHT_FORMAT::FUNCTION /* Temporary. */
        };

    match permutation((
        opt(parse_instance_section(keywords::NODE_COORD_SECTION, node_coord_parser)),
        opt(parse_instance_section(keywords::DEPOT_SECTION, parse_depot)),
        opt(parse_instance_section(keywords::DEMAND_SECTION, parse_node_demand)),
        opt(parse_instance_section(keywords::EDGE_DATA_SECTION, edge_data_parser)),
        opt(parse_instance_section(keywords::FIXED_EDGE_SECTION, parse_edge)),
        opt(parse_instance_section(keywords::TOUR_SECTION, parse_tour)),
        opt(parse_instance_section(keywords::EDGE_WEIGHT_SECTION, parse_edge_weight))
    ))(input)
    {
        Ok((_, (
            _node_coord,
            _depots,
            _demands,
            _edges_data,
            _fixed_edges,
            _tours,
            _edges_weight)))
        =>
            {
                data = Data
                {
                    node_coord_section  : order_node_coord(&_node_coord, dimension),
                    depot_section       : _depots.clone(),
                    demand_section      : order_node_demand_section(&_demands, dimension),
                    edge_data_section   : _edges_data.clone(),
                    fixed_edges_section : _fixed_edges.clone(),
                    tour_section        : _tours.clone(),
                    edge_weight_section : compute_edge_weight_matrix(_edges_weight, edge_weight_format, *dimension)
                };


                /*//DEBUG
                match data.node_coord_section.clone()
                {
                    Some(v) =>
                        {
                            for i in 0..v.len()
                            {
                                let (n, x, y) = match v[i] .clone()
                            {
                                Coord::Coord2d((ni, xi, yi)) => (ni, xi, yi),
                                Coord::Coord3d((ni, xi, yi, _)) => (ni, xi, yi),
                            };
                                println!("  {x1}, {x2}, {x3}", x1 = n, x2 = x, x3 = y);
                            }
                        }
                    None => println!("Error in node_coord_section")
                }*/

                Ok(("", data))

            }
        _ => Err(nom::Err::Error(Error { input, code: ErrorKind::Permutation })),
    }

}