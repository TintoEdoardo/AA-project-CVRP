/* Utility functions used for different
 * tsplib type instances. */

use std::ops::Range;
use crate::tsplib_parser::custom_types::{Coord};

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
                        let s : usize = d_0_i + d_0_j - d_i_j;

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

                let (x_0, y_0) : (f32, f32) = match n_coord[0] {
                    Coord::Coord2d((_, x, y)) => (x as f32, y as f32),
                    Coord::Coord3d(_) => (0 as f32, 0 as f32),
                };

                for i in 1..node_number
                {

                    let (x_i, y_i) : (f32, f32) = match n_coord[i] {
                        Coord::Coord2d((_, x, y)) => (x as f32, y as f32),
                        Coord::Coord3d(_) => (0 as f32, 0 as f32),
                    };

                    for j in (i + 1)..node_number
                    {

                        let (x_j, y_j) : (f32, f32) = match n_coord[j] {
                            Coord::Coord2d((_, x, y)) => (x as f32, y as f32),
                            Coord::Coord3d(_) => (0 as f32, 0 as f32),
                        };

                        let d_0_i : f32 =
                            ((x_0 - x_i).powf(2 as f32) + (y_0 - y_i).powf(2 as f32)).sqrt();

                        let d_0_j : f32 =
                            ((x_0 - x_j).powf(2 as f32) + (y_0 - y_j).powf(2 as f32)).sqrt();

                        let d_i_j : f32 =
                            ((x_i - x_j).powf(2 as f32) + (y_i - y_j).powf(2 as f32)).sqrt();

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

    compute_savings_explicit(
        edge_weight,
        node_number,
        savings);

}

/* Computes savings for instance
 * where the weight of each edge
 * is expressed in semi-full matrix. */
pub(crate) fn compute_savings_hmatrix(
    edge_weight     : &Option< Vec< Vec<usize>>>,
    node_number     : usize,
    is_upper        : bool,
    savings         : &mut Vec<(usize, usize, usize)>)
{

    match edge_weight {

        Some(e_weight) =>
            {

                for i in 1..node_number
                {

                    let range : Range<usize>;

                    if is_upper
                    {

                        range = i..node_number;

                    }
                    else
                    {

                        range = 1..i;

                    }

                    for j in range
                    {

                        let weight_i_j : usize = e_weight[i][j];

                        /* The edge is considered only
                         * if the weight is not 0. */
                        if weight_i_j != 0
                        {

                            let d_0_i : usize = e_weight[0][i];
                            let d_0_j : usize = e_weight[0][j];
                            let d_i_j : usize = e_weight[i][j];

                            /* Compute the saving for the edge between i, j. */
                            let s : usize = d_0_i + d_0_j - d_i_j;

                            savings.push((i, j, s));

                        }

                    }

                }

            }

        _ => (),
    }

}
