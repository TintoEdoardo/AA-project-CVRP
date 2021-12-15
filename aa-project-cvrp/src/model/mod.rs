/*
 * This module encapsulates the dependency with tsplib_parser,
 * provides new trait definitions to hide the
 * difference between the CRVP instances received
 * as input.
 */

mod utils;
use crate::tsplib_parser::problem_instance::TSPInstance;
use crate::sweep_algorithm::sweep_instance_trait::SweepInstanceTrait;
use crate::savings_algorithm::savings_instance_trait::SavingsInstanceTrait;
use crate::model::utils::{compute_savings_coord, compute_savings_fmatrix, from_hmatrix_to_fmatrix, compute_savings_geo, compute_distance_geo, compute_distance_euc};
use crate::tsplib_parser::keyword_values::{EDGE_WEIGHT_TYPE, EDGE_WEIGHT_FORMAT};
use crate::tsplib_parser::custom_types::{Node, Coord};
use rand::Rng;
use rand::rngs::ThreadRng;

/* The GraphInstance object encapsulates
 * the tsplib dependence, which is then
 * transparent to the other entities. */
pub struct GraphInstance<'a>
{

    /* Instance of the CVRP problem,
     * implemented in the tsplib module. */
    pub instance : TSPInstance<'a>,

}

/* Implementation of the SavingsInstanceTrait
 * for GraphInstance. */
impl SavingsInstanceTrait for GraphInstance<'_>
{

    fn compute_savings(&self) -> Vec<(usize, usize, usize)> {

        let mut savings : Vec<(usize, usize, usize)> = Vec::new();

        /* Dimension is given by the number of nodes
         * plus the depot (assuming single depot). */
        let node_number : usize = self.instance.specification.dimension;

        /* Define pointers to the instance fields
         * required for further computation. */
        let edge_weight        : &Option< Vec< Vec<usize>>>
            = &self.instance.data.edge_weight_section;

        let edge_weight_type   : &EDGE_WEIGHT_TYPE
            = &self.instance.specification.edge_weight_type;

        let edge_weight_format : &Option<EDGE_WEIGHT_FORMAT>
            = &self.instance.specification.edge_weight_format;

        /* Here the different types of CVRP
         * instances should be accounted. */
        match (edge_weight_type, edge_weight_format, edge_weight.as_ref())
        {

            (EDGE_WEIGHT_TYPE::EUC_2D, _, _) =>
                compute_savings_coord(
                    &self.instance.data.node_coord_section,
                    node_number,
                    &mut savings),

            (EDGE_WEIGHT_TYPE::GEO, _, _) =>
                compute_savings_geo(
                    &self.instance.data.node_coord_section,
                    node_number,
                    &mut savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::FULL_MATRIX), _) =>
                compute_savings_fmatrix(
                    &self.instance.data.edge_weight_section,
                    node_number,
                    &mut savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::LOWER_COL), _) =>
                compute_savings_fmatrix(
                    &from_hmatrix_to_fmatrix(edge_weight, node_number, EDGE_WEIGHT_FORMAT::LOWER_COL),
                    node_number,
                    &mut savings
                ),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::LOWER_DIAG_COL), _) =>
                compute_savings_fmatrix(
                    &from_hmatrix_to_fmatrix(edge_weight, node_number, EDGE_WEIGHT_FORMAT::LOWER_DIAG_COL),
                    node_number,
                    &mut savings
                ),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::LOWER_DIAG_ROW), _) =>
                compute_savings_fmatrix(
                    &from_hmatrix_to_fmatrix(edge_weight, node_number, EDGE_WEIGHT_FORMAT::LOWER_DIAG_ROW),
                    node_number,
                    &mut savings
                ),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::LOWER_ROW), _) =>
                compute_savings_fmatrix(
                    &from_hmatrix_to_fmatrix(edge_weight, node_number, EDGE_WEIGHT_FORMAT::LOWER_ROW),
                    node_number,
                    &mut savings
                ),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::UPPER_COL), _) =>
                compute_savings_fmatrix(
                    &from_hmatrix_to_fmatrix(edge_weight, node_number, EDGE_WEIGHT_FORMAT::UPPER_COL),
                    node_number,
                    &mut savings
                ),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::UPPER_DIAG_COL), _) =>
                compute_savings_fmatrix(
                    &from_hmatrix_to_fmatrix(edge_weight, node_number, EDGE_WEIGHT_FORMAT::UPPER_DIAG_COL),
                    node_number,
                    &mut savings
                ),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::UPPER_DIAG_ROW), _) =>
                compute_savings_fmatrix(
                    &from_hmatrix_to_fmatrix(edge_weight, node_number, EDGE_WEIGHT_FORMAT::UPPER_DIAG_ROW),
                    node_number,
                    &mut savings
                ),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::UPPER_ROW), _) =>
                compute_savings_fmatrix(
                    &from_hmatrix_to_fmatrix(edge_weight, node_number, EDGE_WEIGHT_FORMAT::UPPER_ROW),
                    node_number,
                    &mut savings
                ),

            _ => (),

        }

        return savings;

    }

    fn get_capacity(&self) -> usize {

        self.instance.specification.capacity

    }

    fn get_nodes_list(&self) -> Vec<usize> {

        let node_number : usize      = self.instance.specification.dimension;
        let mut nodes   : Vec<usize> = Vec::with_capacity(node_number);

        /* Initialize nodes. */
        for _ in 0..nodes.capacity()
        {
            nodes.push(0);
        }

        for i in 0..node_number
        {

            nodes[i] = i;

        }

        return nodes;

    }

    fn get_nodes_demand(&self) -> Vec<usize> {

        let d_section : Vec<(Node, usize)> = match self.instance.data.demand_section.clone()
        {
            Some(d_s) => d_s,
            _ => Vec::new(),
        };

        let result : Vec<usize> = d_section.iter().map(|(_, d)| *d).collect();

        return result;

    }
}

/* Implementation of the SweepInstanceTrait
 * for GraphInstance. */
impl SweepInstanceTrait for GraphInstance<'_>
{

    fn order_nodes(&self) -> Vec<usize>
    {

        let dimension          : usize                      = self.instance.specification.dimension;
        let edge_weight_type   : &EDGE_WEIGHT_TYPE          = &self.instance.specification.edge_weight_type;
        let edge_weights       : &Option< Vec< Vec<usize>>> = &self.instance.data.edge_weight_section;
        let node_coord         : &Option< Vec< Coord>>      = &self.instance.data.node_coord_section;
        let result             : Vec<Node>;

        /* Select a node randomly. */
        let mut rng            : ThreadRng         = rand::thread_rng();
        let n1                 : Node              = rng.gen_range(1..dimension);
        let n2                 : Node;
        let n3                 : Node;

        /* Check if the edge weight are expressed
         * as coord distances. */
        if  *edge_weight_type == EDGE_WEIGHT_TYPE::GEO    ||
            *edge_weight_type == EDGE_WEIGHT_TYPE::EUC_2D ||
            *edge_weight_type == EDGE_WEIGHT_TYPE::EUC_3D
        {

            /* In this solution we reduce the problem to planar
            instance. Therefore, 3D coord are used as 2D coord. */

            let n_coord : Vec< Coord> = match node_coord.clone()
            {
                Some(n_c) => n_c,
                _ => Vec::new(),
            };

            /* The format is (Node, Angle, Radius). */
            let mut node_polar_coord : Vec<(Node, f64, f64)>
                = Vec::with_capacity(dimension - 1);

            let x_0 : f64 = match n_coord[0]
            {
                Coord::Coord2d((_, x, _)) => x,
                Coord::Coord3d((_, x, _, _)) => x,
            };

            let y_0 : f64 = match n_coord[0]
            {
                Coord::Coord2d((_, _, y)) => y,
                Coord::Coord3d((_, _, y, _)) => y,
            };

            /* Compute the polar coordinates. */
            for i in 1..dimension
            {

                let x_i : f64 = match n_coord[i]
                {
                    Coord::Coord2d((_, x, _)) => x,
                    Coord::Coord3d((_, x, _, _)) => x,
                };

                let y_i : f64 = match n_coord[i]
                {
                    Coord::Coord2d((_, _, y)) => y,
                    Coord::Coord3d((_, _, y, _)) => y,
                };

                let angle_i : f64
                    = ( (y_i - y_0) / (x_i - x_0) ).atan();

                let radius_i : f64
                    = ((y_i - y_0).powf(2.0) + (x_i - x_0).powf(2.0)).sqrt();

                node_polar_coord.push((i, angle_i, radius_i));

            }

            node_polar_coord.sort_by(|(_, a1, _), (_, a2, _)| a1.partial_cmp(a2).unwrap());
            node_polar_coord.sort_by(|&(_, a1, r1), &(_, a2, r2)|
                {
                    match a1 == a2
                    {
                        true  => r1.partial_cmp(&r2).unwrap(),
                        false => a1.partial_cmp(&a2).unwrap(),
                    }
                });

            result = node_polar_coord.iter().map(|(n, _, _)| n.clone()).collect();

        }
        /* Otherwise we have to handle
         * a matrix. */
        else
        {

            /* Move the edge weight information. */
            let e_weights_hmatrix : Option<Vec< Vec<usize>>> = edge_weights.clone();

            let edge_weight_format : &Option<EDGE_WEIGHT_FORMAT> = &self.instance.specification.edge_weight_format;
            let e_w_f : EDGE_WEIGHT_FORMAT = match edge_weight_format
            {
                Some(w_f) => w_f.clone(),
                _ => EDGE_WEIGHT_FORMAT::FULL_MATRIX,
            };

            let mut op_e_weights : Option< Vec< Vec<usize>>> = e_weights_hmatrix.clone();

            /* Convert edge weight into full matrix. */
            if e_w_f != EDGE_WEIGHT_FORMAT::FULL_MATRIX
            {
                op_e_weights =
                    from_hmatrix_to_fmatrix(&e_weights_hmatrix, dimension, e_w_f);
            }

            let e_weights : Vec< Vec<usize>> = match op_e_weights
            {
                Some(ew) => ew,
                _ => Vec::new()
            };

            /* Compute a list of nodes sorted by
             * distance from n1. */
            let nodes_from_n1: Vec<(Node, f64)> =
                compute_node_list_sort_by_distance(&e_weights, n1, dimension);

            n2 = match nodes_from_n1.last()
            {
                Some((n, _)) => *n,
                _ => 0,
            };

            /* Compute the node which maximise the
             * value of d_n1_n3 + d_n2_n3. */
            let mut dist_n1_n2_n3 : f64  = 0.0;
            let mut tmp_n3        : Node = 0;
            for i in 1..dimension
            {

                /* Skip if the current node is n1 or n2. */
                if i == n1 || i == n2
                {
                    continue;
                }

                let dist_n1_n3 : f64 = compute_distance_from_nodes(&e_weights, n1, i);
                let dist_n2_n3 : f64 = compute_distance_from_nodes(&e_weights, n2, i);

                if dist_n1_n3 + dist_n2_n3 > dist_n1_n2_n3
                {
                    tmp_n3        = i;
                    dist_n1_n2_n3 = dist_n1_n3 + dist_n2_n3;
                }

            }

            n3 = tmp_n3;

            /* Finally, partition the nodes. */
            let mut cluster_n1_n2: Vec<(Node, f64)> = Vec::new();
            let mut cluster_n2_n3: Vec<(Node, f64)> = Vec::new();
            let mut cluster_n3_n1: Vec<(Node, f64)> = Vec::new();

            /* Insert n1, n2, n3, one for each cluster. */
            cluster_n1_n2.push((n1, 0.0));
            cluster_n2_n3.push((n2, 0.0));
            cluster_n3_n1.push((n3, 0.0));

            for i in 1..dimension
            {

                /* Skip if i is one of the cluster centers. */
                if i == n1 || i == n2 || i == n3
                {
                    continue;
                }

                let dist_n1_i : f64 = compute_distance_from_nodes(&e_weights, n1, i);
                let dist_n2_i : f64 = compute_distance_from_nodes(&e_weights, n2, i);
                let dist_n3_i : f64 = compute_distance_from_nodes(&e_weights, n3, i);

                if dist_n1_i < dist_n3_i
                {
                    if dist_n2_i < dist_n3_i
                    {
                        cluster_n1_n2.push((i, dist_n1_i));
                    }
                    else
                    {
                        cluster_n3_n1.push((i, dist_n3_i));
                    }
                }
                else
                {
                    if dist_n1_i < dist_n2_i
                    {
                        cluster_n3_n1.push((i, dist_n3_i));
                    }
                    else
                    {
                        cluster_n2_n3.push((i, dist_n2_i));
                    }
                }

            }

            cluster_n1_n2.sort_by_key(|n| n.1 as usize);
            cluster_n2_n3.sort_by_key(|n| n.1 as usize);
            cluster_n3_n1.sort_by_key(|n| n.1 as usize);

            /* Produce a single list from
             * the three clusters. */
            cluster_n1_n2.append(cluster_n2_n3.as_mut());
            cluster_n1_n2.append(cluster_n3_n1.as_mut());

            result = cluster_n1_n2.iter().map(|&n| n.0).collect();

        }

        return result;

    }

    fn get_capacity(&self) -> usize {

        self.instance.specification.capacity

    }

    fn get_nodes_demand(&self) -> Vec<usize> {

        let d_section : Vec<(Node, usize)> =
            match self.instance.data.demand_section.clone()
        {
            Some(d_s) => d_s,
            _ => Vec::new(),
        };

        let result : Vec<usize> =
            d_section.iter().map(|(_, d)| *d).collect();

        return result;

    }
}

fn compute_distance_from_nodes(
    e_weights : &Vec< Vec<usize>>,
    n1        : Node,
    n2        : Node)
    -> f64
{

    let distance_n1_n2 : f64 = e_weights[n1][n2] as f64;

    return distance_n1_n2;

}

fn compute_node_list_sort_by_distance(
    e_weights : &Vec< Vec<usize>>,
    n1        : Node,
    dimension : usize)
    -> Vec<(Node, f64)>
{

    let mut nodes_from_n1: Vec<(Node, f64)> = Vec::with_capacity(dimension - 2);
    for i in 1..(dimension - 1)
    {

        if i == n1
        {
            continue;
        }

        let n2             : Node = i;
        let distance_n1_n2 : f64  = compute_distance_from_nodes(e_weights, n1, n2);

        nodes_from_n1.push((n2, distance_n1_n2));

    }

    /* Sort the node according to their distances
     * from n1. */
    nodes_from_n1.sort_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());

    return nodes_from_n1;

}

fn compute_distance_from_nodes_coord(
    coord : &Vec<Coord>,
    n1    : usize,
    n2    : usize)
    -> f64
{

    let (n1_x, n1_y) : (f64, f64) = match coord[n1]
    {
        Coord::Coord2d((_, x, y)) => (x, y),
        Coord::Coord3d((_, x, y, _)) => (x, y),
    };

    let (n2_x, n2_y) : (f64, f64) = match coord[n2]
    {
        Coord::Coord2d((_, x, y)) => (x, y),
        Coord::Coord3d((_, x, y, _)) => (x, y),
    };

    return ((n1_x - n2_x).powf(2.0) + (n1_y - n2_y).powf(2.0)).sqrt();

}

pub(crate) fn compute_cost_of_routes(
    instance : TSPInstance,
    routes   : Vec< Vec<usize>>)
    -> f64
{

    let node_number     : usize                      = instance.specification.dimension.clone();
    let e_weights       : Option< Vec< Vec<usize>>>  = instance.data.edge_weight_section.clone();
    let coord           : Option< Vec <Coord>>       = instance.data.node_coord_section.clone();
    let e_weight_format : Option<EDGE_WEIGHT_FORMAT> = instance.specification.edge_weight_format.clone();
    let e_weight_type   : EDGE_WEIGHT_TYPE           = instance.specification.edge_weight_type.clone();

    let mut result  : f64 = 0.0;

    if e_weights.is_some()
    {
        let e_w_hmatrix : &Vec< Vec<usize>> = e_weights.as_ref().unwrap();
        let e_w_matrix  : Vec< Vec<usize>>;

        match e_weight_format
        {
            Some(EDGE_WEIGHT_FORMAT::FULL_MATRIX) => e_w_matrix = e_w_hmatrix.clone(),
            Some(e_w_f) => e_w_matrix =
                from_hmatrix_to_fmatrix(&e_weights, node_number, e_w_f).unwrap(),
            _ => e_w_matrix = Vec::new()
        }

        for route_i in 0..routes.len()
        {

            let mut prev_index    : usize = 0;
            let mut previous_node : usize = prev_index;
            let route_i_len       : usize = routes[route_i].len();

            /* Add the cost of the first edge. */
            result = result + e_w_matrix[previous_node][routes[route_i][0]] as f64;

            println!("    Route{}: ", route_i);
            print!("    c-{} ", e_w_matrix[previous_node][routes[route_i][0]] as f64);
            '_inner: for current_index in 0..route_i_len
            {
                print!("    n-{} ", routes[route_i][current_index]);
                let current_node  : usize = routes[route_i][current_index];
                if current_index == route_i_len - 1
                {
                    print!("    c-{} ", e_w_matrix[current_node][0] as f64);
                    result = result + e_w_matrix[current_node][0] as f64;
                    break;
                }
                else
                {
                    print!("    c-{} ", e_w_matrix[previous_node][current_node] as f64);
                    result        = result + e_w_matrix[previous_node][current_node] as f64;
                    prev_index    = current_index;
                    previous_node = routes[route_i][prev_index];
                }
            }
            println!();
        }

    }
    else
    {

        /* Coord should not be None. */
        assert!(coord.is_some());

        /* Check if the Coord ar eucledean
         * or geographical. */
        let distance_function : fn(&Coord, &Coord) -> f64;
        if e_weight_type == EDGE_WEIGHT_TYPE::GEO
        {
            distance_function = compute_distance_geo;
        }
        else
        {
            distance_function = compute_distance_euc;
        }

        let c_vector : Vec< Coord> = coord.unwrap();

        for route_i in 0..routes.len()
        {

            /* Add the cost of the first edge. */
            result = result + distance_function(&c_vector[0], &c_vector[routes[route_i][0]]);

            let mut prev_index    : usize = 0;
            let mut previous_node : usize = prev_index;
            let route_i_len       : usize = routes[route_i].len();

            println!("    Route{}: ", route_i);
            print!("    c-{} ", distance_function(&c_vector[0], &c_vector[routes[route_i][0]]));
            for current_index in 0..routes[route_i].len()
            {

                print!("    n-{} ", routes[route_i][current_index]);
                let current_node  : usize = routes[route_i][current_index];
                if current_index == route_i_len - 1
                {
                    print!("    c-{} ", distance_function(&c_vector[current_node], &c_vector[0]));
                    result = result + distance_function(&c_vector[current_node], &c_vector[0]);
                    break;
                }
                else
                {
                    print!("    c-{} ", distance_function(&c_vector[previous_node], &c_vector[current_node]));
                    result        = result + distance_function(&c_vector[previous_node], &c_vector[current_node]);
                    prev_index    = current_index;
                    previous_node = routes[route_i][prev_index];
                }
            }
            println!();
        }

    }

    return result;

}
