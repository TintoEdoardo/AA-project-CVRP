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

/* Compute the euclidean distance
 * between two points. */
pub(crate) fn compute_distance_euc(
    node_1_coord : &Coord,
    node_2_coord : &Coord)
    -> f64
{

    /* Extract the coord. */
    let (x_1, y_1) = match node_1_coord
    {
        Coord::Coord2d((_, _x, _y)) => (_x, _y),
        Coord::Coord3d((_, _x, _y, _)) => (_x, _y)
    };
    let (x_2, y_2) = match node_2_coord
    {
        Coord::Coord2d((_, _x, _y)) => (_x, _y),
        Coord::Coord3d((_, _x, _y, _)) => (_x, _y)
    };

    /* Compute distance. */
    let dist_1_2 : f64 =
        ((x_1 - x_2).powf(2.0) + (y_1 - y_2).powf(2.0)).sqrt().ceil();

    return dist_1_2;

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

                for i in 1..node_number
                {

                    for j in (i + 1)..node_number
                    {

                        let d_0_i : f64 =
                            compute_distance_euc(&n_coord[0], &n_coord[i]);

                        let d_0_j : f64 =
                            compute_distance_euc(&n_coord[0], &n_coord[j]);

                        let d_i_j : f64 =
                            compute_distance_euc(&n_coord[i], &n_coord[j]);

                        /* Compute the saving for the edge between i, j. */
                        let s : usize = (d_0_i + d_0_j - d_i_j) as usize;

                        savings.push((i, j, s));

                    }

                }

            }

        _ => ()

    }

}

/* Compute the geo distance
 * between two points. */
pub(crate) fn compute_distance_geo(
    node_1_coord : &Coord,
    node_2_coord : &Coord)
    -> f64
{

    /* Extract the coord. */
    let (x_1, y_1) = match node_1_coord
    {
        Coord::Coord2d((_, _x, _y)) => (_x, _y),
        Coord::Coord3d((_, _x, _y, _)) => (_x, _y)
    };
    let (x_2, y_2) = match node_2_coord
    {
        Coord::Coord2d((_, _x, _y)) => (_x, _y),
        Coord::Coord3d((_, _x, _y, _)) => (_x, _y)
    };

    /* Compute the latitude of node 1. */
    let node_1_lat_deg : f64 = x_1.trunc();
    let node_1_lat_min : f64 = x_1.fract(); // - node_1_lat_deg;
    let latitude_1     : f64 =
        std::f64::consts::PI * (node_1_lat_deg + 5.0 * node_1_lat_min / 3.0) / 180.0;

    /* Compute the latitude of node 2. */
    let node_2_lat_deg : f64 = x_2.trunc();
    let node_2_lat_min : f64 = x_2.fract(); // - node_2_lat_deg;
    let latitude_2     : f64 =
        std::f64::consts::PI * (node_2_lat_deg + 5.0 * node_2_lat_min / 3.0) / 180.0;

    /* Compute the longitude of node 1. */
    let node_1_lon_deg : f64 = y_1.trunc();
    let node_1_lon_min : f64 = y_1.fract(); // - node_1_lon_deg;
    let longitude_1     : f64 =
        std::f64::consts::PI * (node_1_lon_deg + 5.0 * node_1_lon_min / 3.0) / 180.0;

    /* Compute the longitude of node 2. */
    let node_2_lon_deg : f64 = y_2.trunc();
    let node_2_lon_min : f64 = y_2.fract(); // - node_2_lon_deg;
    let longitude_2     : f64 =
        std::f64::consts::PI * (node_2_lon_deg + 5.0 * node_2_lon_min / 3.0) / 180.0;

    let RRR : f64 = 6378.388;
    let q1  : f64 = (longitude_1 - longitude_2).cos();
    let q2  : f64 = (latitude_1 - latitude_2).cos();
    let q3  : f64 = (latitude_1 + latitude_2).cos();

    /* Compute distance. */
    let dist_1_2 : f64 =
        (RRR * (0.5 * ((1.0 + q1) * q2 - (1.0 - q1) * q3)).acos() + 1.0).ceil();

    return dist_1_2;

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

                for i in 1..node_number
                {

                    for j in (i + 1)..node_number
                    {

                        let d_0_i : f64 =
                            compute_distance_geo(&n_coord[0], &n_coord[i]);

                        let d_0_j : f64 =
                            compute_distance_geo(&n_coord[0], &n_coord[j]);

                        let d_i_j : f64 =
                            compute_distance_geo(&n_coord[i], &n_coord[j]);

                        /* Compute the saving for the edge between i, j. */
                        let s : usize = (d_0_i + d_0_j - d_i_j).ceil() as usize;

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