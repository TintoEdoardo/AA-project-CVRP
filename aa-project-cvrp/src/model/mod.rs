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

/* The GraphInstance object encapsulates
 * the tsplib dependence, which is then
 * transparent to the other entities. */
pub struct GraphInstance
{

    /* Instance of the CVRP problem,
     * implemented in the tsplib module. */
    instance : TSPInstance,

}

/* Implementation of the SavingsInstanceTrait
 * for GraphInstance. */
impl SavingsInstanceTrait for GraphInstance
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
        match (edge_weight_type, edge_weight_format, edge_weight.unwrap())
        {

            (EDGE_WEIGHT_TYPE::EUC_2D, _, _) =>
                compute_savings_coord(
                    &self.instance.data.node_coord_section,
                    node_number,
                    &savings),

            (EDGE_WEIGHT_TYPE::GEO, _, _) =>
                compute_savings_coord(
                    &self.instance.data.node_coord_section,
                    node_number,
                    &savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::FULL_MATRIX), _) =>
                compute_savings_fmatrix(
                    &self.instance.data.edge_weight_section,
                    node_number,
                    &savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::LOWER_COL), _) =>
                compute_savings_hmatrix(
                    edge_weight,
                    node_number,
                    false,
                    &savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::LOWER_DIAG_COL), _) =>
                compute_savings_hmatrix(
                    edge_weight,
                    node_number,
                    false,
                    &savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::LOWER_DIAG_ROW), _) =>
                compute_savings_hmatrix(
                    edge_weight,
                    node_number,
                    false,
                    &savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::LOWER_ROW), _) =>
                compute_savings_hmatrix(
                    edge_weight,
                    node_number,
                    false,
                    &savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::UPPER_COL), _) =>
                compute_savings_hmatrix(
                    edge_weight,
                    node_number,
                    true,
                    &savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::UPPER_DIAG_COL), _) =>
                compute_savings_hmatrix(
                    edge_weight,
                    node_number,
                    true,
                    &savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::UPPER_DIAG_ROW), _) =>
                compute_savings_hmatrix(
                    edge_weight,
                    node_number,
                    true,
                    &savings),

            (EDGE_WEIGHT_TYPE::EXPLICIT, Some(EDGE_WEIGHT_FORMAT::UPPER_ROW), _) =>
                compute_savings_hmatrix(
                    edge_weight,
                    node_number,
                    true,
                    &savings),

            _ => (),

        }

        return savings;

    }

    fn get_capacity(&self) -> usize {
        todo!()
    }

    /* fn get_capacity(&self) -> usize {

        return self.instance.specification.capacity;

    } */

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
        todo!()
    }
}

/* Implementation of the SweepInstanceTrait
 * for GraphInstance. */
impl SweepInstanceTrait for GraphInstance
{

    fn order_nodes(&self) -> Vec<usize>
    {



    }

}
