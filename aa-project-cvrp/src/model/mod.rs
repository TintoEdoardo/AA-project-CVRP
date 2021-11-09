/*
 * This module encapsulates the tsplib module,
 * provides new trait definitions to hide the
 * difference between the CRVP instances received
 * as input.
 */

mod utils;
use crate::tsplib_parser::problem_instance::TSPInstance;
use crate::sweep_algorithm::sweep_instance_trait::SweepInstanceTrait;
use crate::savings_algorithm::savings_instance_trait::SavingsInstanceTrait;
use crate::model::utils::{compute_savings_coord, compute_savings_fmatrix, compute_savings_hmatrix};
use crate::tsplib_parser::keyword_values::{EDGE_WEIGHT_TYPE, EDGE_WEIGHT_FORMAT};
use crate::tsplib_parser::custom_types::{Node};
use rand::Rng;
use rand::rngs::ThreadRng;

/* The GraphInstance object encapsulates
 * the tsplib dependence, which is then
 * transparent to the other entities. */
pub struct GraphInstance<'a>
{

    /* Instance of the CVRP problem,
     * implemented in the tsplib module. */
    instance : TSPInstance<'a>,

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
                compute_savings_coord(
                    &self.instance.data.node_coord_section,
                    node_number,
                    &mut savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::FULL_MATRIX), _) =>
                compute_savings_fmatrix(
                    &self.instance.data.edge_weight_section,
                    node_number,
                    &mut savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::LOWER_COL), _) =>
                compute_savings_hmatrix(
                    edge_weight,
                    node_number,
                    false,
                    &mut savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::LOWER_DIAG_COL), _) =>
                compute_savings_hmatrix(
                    edge_weight,
                    node_number,
                    false,
                    &mut savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::LOWER_DIAG_ROW), _) =>
                compute_savings_hmatrix(
                    edge_weight,
                    node_number,
                    false,
                    &mut savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::LOWER_ROW), _) =>
                compute_savings_hmatrix(
                    edge_weight,
                    node_number,
                    false,
                    &mut savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::UPPER_COL), _) =>
                compute_savings_hmatrix(
                    edge_weight,
                    node_number,
                    true,
                    &mut savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::UPPER_DIAG_COL), _) =>
                compute_savings_hmatrix(
                    edge_weight,
                    node_number,
                    true,
                    &mut savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::UPPER_DIAG_ROW), _) =>
                compute_savings_hmatrix(
                    edge_weight,
                    node_number,
                    true,
                    &mut savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::UPPER_ROW), _) =>
                compute_savings_hmatrix(
                    edge_weight,
                    node_number,
                    true,
                    &mut savings),

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
        let mut result: Vec<Node>                  = Vec::with_capacity(dimension);

        /* Select a node randomly. */
        let mut rng            : ThreadRng         = rand::thread_rng();
        let n1                 : Node              = rng.gen_range(1..dimension);
        let n2                 : Node;
        let n3                 : Node;

        /* Check if the edge weight are expressed
         * as coord distances. */
        if  *edge_weight_type == EDGE_WEIGHT_TYPE::GEO ||
            *edge_weight_type == EDGE_WEIGHT_TYPE::EUC_2D ||
            *edge_weight_type == EDGE_WEIGHT_TYPE::EUC_3D
        {



        }
        /* Otherwise we have to handle
         * a matrix. */
        else
        {

            /* Move the edge weight information. */
            let e_weights : Vec< Vec <usize>> = match edge_weights.clone()
            {
                Some(e_vec) => e_vec,
                _ => Vec::new(),
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
            let mut dist_n1_n2_n3 : f64  = 0 as f64;
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

    let distance_n1_n2 : f64;
    if e_weights.len() >= n1 && e_weights[n1].len() >= n2
    {
        distance_n1_n2 = e_weights[n1][n2] as f64;
    }
    else
    {
        distance_n1_n2 = e_weights[n2][n1] as f64;
    }

    return distance_n1_n2;

}

fn compute_node_list_sort_by_distance(
    e_weights : &Vec< Vec<usize>>,
    n1        : Node,
    dimension : usize)
    -> Vec<(Node, f64)>
{

    let mut nodes_from_n1: Vec<(Node, f64)> = Vec::with_capacity(dimension - 1);
    for i in 1..(dimension)
    {

        let n2             : Node = i;
        let distance_n1_n2 : f64  = compute_distance_from_nodes(e_weights, n1, n2);

        nodes_from_n1[n2] = (n2, distance_n1_n2);

    }

    /* Sort the node according to their distances
     * from n1. */
    nodes_from_n1.sort_by_key(|n| n.1 as usize);

    return nodes_from_n1;

}
