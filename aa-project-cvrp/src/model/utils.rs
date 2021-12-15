/* Utility functions used for different
 * tsplib instances. */

use crate::tsplib_parser::custom_types::{Coord};
use crate::tsplib_parser::keyword_values::{EDGE_WEIGHT_FORMAT};

/* Computes savings for instance
 * where the weight of each edge
 * is explicit. */
pub(crate) fn compute_savings_explicit(
    edge_weight      : &Option< Vec< Vec<usize>>>,
    node_number      : usize,
    savings          : &mut Vec<(usize, usize, usize)>)
{

    match edge_weight {

        Some(e_weight) =>
            {

                for i in 1..node_number
                {

                    for j in (i + 1)..node_number
                    {

                        let d_0_i : usize = e_weight[0][i];
                        let d_0_j : usize = e_weight[0][j];
                        let d_i_j : usize = e_weight[i][j];

                        /* Compute the saving for the edge between i, j. */
                        let s : usize = (d_0_i + d_0_j).checked_sub(d_i_j).unwrap_or(0);

                        savings.push((i, j, s));

                    }

                }

            }

        _ => ()

    }

}

/* Computes savings for instance
 * where the weight of each edge
 * is expressed as euclidean distance. */
pub(crate) fn compute_savings_coord(
    node_coord      : &Option< Vec<Coord>>,
    node_number     : usize,
    savings         : &mut Vec<(usize, usize, usize)>)
{

    match node_coord {

        Some(n_coord) =>
            {

                let (x_0, y_0) : (f64, f64) = match n_coord[0] {
                    Coord::Coord2d((_, x, y)) => (x, y),
                    Coord::Coord3d(_) => (0.0, 0.0),
                };

                for i in 1..node_number
                {

                    let (x_i, y_i) : (f64, f64) = match n_coord[i] {
                        Coord::Coord2d((_, x, y)) => (x, y),
                        Coord::Coord3d(_) => (0.0, 0.0),
                    };

                    for j in (i + 1)..node_number
                    {

                        let (x_j, y_j) : (f64, f64) = match n_coord[j] {
                            Coord::Coord2d((_, x, y)) => (x, y),
                            Coord::Coord3d(_) => (0.0, 0.0),
                        };

                        let d_0_i : f64 =
                            ((x_0 - x_i).powf(2.0) + (y_0 - y_i).powf(2.0)).sqrt();

                        let d_0_j : f64 =
                            ((x_0 - x_j).powf(2.0) + (y_0 - y_j).powf(2.0)).sqrt();

                        let d_i_j : f64 =
                            ((x_i - x_j).powf(2.0) + (y_i - y_j).powf(2.0)).sqrt();

                        /* Compute the saving for the edge between i, j. */
                        let s : usize = (d_0_i + d_0_j - d_i_j) as usize;

                        savings.push((i, j, s));

                    }

                }

            }

        _ => ()

    }

}

/* Computes savings for instance
 * where the weight of each edge
 * is expressed as geographical distance. */
pub(crate) fn compute_savings_geo(
    node_coord      : &Option< Vec<Coord>>,
    node_number     : usize,
    savings         : &mut Vec<(usize, usize, usize)>)
{

    match node_coord {

        Some(n_coord) =>
            {

                let (x_0, y_0) : (f64, f64) = match n_coord[0] {
                    Coord::Coord2d((_, x, y)) => (x, y),
                    Coord::Coord3d(_) => (0.0, 0.0),
                };

                for i in 1..node_number
                {

                    let (x_i, y_i) : (f64, f64) = match n_coord[i] {
                        Coord::Coord2d((_, x, y)) => (x, y),
                        Coord::Coord3d(_) => (0.0, 0.0),
                    };

                    for j in (i + 1)..node_number
                    {

                        let (x_j, y_j) : (f64, f64) = match n_coord[j] {
                            Coord::Coord2d((_, x, y)) => (x, y),
                            Coord::Coord3d(_) => (0.0, 0.0),
                        };

                        let d_0_i : f64 =
                            ((x_0 - x_i).powf(2.0) + (y_0 - y_i).powf(2.0)).sqrt();

                        let d_0_j : f64 =
                            ((x_0 - x_j).powf(2.0) + (y_0 - y_j).powf(2.0)).sqrt();

                        let d_i_j : f64 =
                            ((x_i - x_j).powf(2.0) + (y_i - y_j).powf(2.0)).sqrt();

                        /* Compute the saving for the edge between i, j. */
                        let s : usize = (d_0_i + d_0_j - d_i_j) as usize;

                        savings.push((i, j, s));

                    }

                }

            }

        _ => ()

    }

}



/* Computes savings for instance
 * where the weight of each edge
 * is expressed in a full matrix. */
pub(crate) fn compute_savings_fmatrix(
    edge_weight     : &Option< Vec< Vec<usize>>>,
    node_number     : usize,
    savings         : &mut Vec<(usize, usize, usize)>)
{

    /* For now, it is sufficient to call
     * compute_savings_explicit. */
    compute_savings_explicit(
        edge_weight,
        node_number,
        savings);

}

/* Convert half matrix into full matrix. */
pub(crate) fn from_hmatrix_to_fmatrix(
    edge_weight     : &Option< Vec< Vec<usize>>>,
    node_number     : usize,
    e_weight_format : EDGE_WEIGHT_FORMAT)
    -> Option< Vec< Vec<usize>>>
{

    match edge_weight
    {
        Some(e_weight) =>
        {

            let mut result : Vec< Vec<usize>> = Vec::with_capacity(node_number);
            for i in 0..node_number
            {
                result.push(Vec::with_capacity(node_number));
                for _j in 0..node_number
                {
                    result[i].push(0);
                }
            }

            if e_weight_format == EDGE_WEIGHT_FORMAT::UPPER_ROW ||
                e_weight_format == EDGE_WEIGHT_FORMAT::UPPER_DIAG_ROW
            {
                for i in 0..node_number
                {
                    for j in 0..node_number
                    {
                        if i < j
                        {
                            result[i][j] = e_weight[i][j - i - 1].clone();
                        }
                        else if i == j
                        {
                            result[i][j] = 0;
                        }
                        else if i > j
                        {
                            result[i][j] = e_weight[j][i -j - 1].clone();
                        }
                    }
                }
            }
            else if e_weight_format == EDGE_WEIGHT_FORMAT::LOWER_ROW ||
                e_weight_format == EDGE_WEIGHT_FORMAT::LOWER_DIAG_ROW
            {
                for i in 0..node_number
                {

                    for j in 0..node_number
                    {
                        if i < j
                        {
                            result[i][j] = e_weight[j][i].clone();
                        }
                        else if i == j
                        {
                            result[i][j] = 0;
                        }
                        else
                        {
                            result[i][j] = e_weight[i][j].clone();
                        }
                    }
                }
            }
            else if e_weight_format == EDGE_WEIGHT_FORMAT::UPPER_COL ||
                e_weight_format == EDGE_WEIGHT_FORMAT::UPPER_DIAG_COL
            {
                /*
                    TODO:
                        IMPLEMENT
                */
            }
            else if e_weight_format == EDGE_WEIGHT_FORMAT::LOWER_COL ||
                e_weight_format == EDGE_WEIGHT_FORMAT::LOWER_DIAG_COL
            {
                /*
                    TODO:
                        IMPLEMENT
                */
            }
            else
            {
                result.clear();
            }

            if result.len() > 0
            {
                Some(result)
            }
            else
            {
                None
            }

        }
        _ => None
    }

}