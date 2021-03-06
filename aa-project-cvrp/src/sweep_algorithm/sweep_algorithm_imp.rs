use crate::sweep_algorithm::sweep_instance_trait::SweepInstanceTrait;
use crate::algorithm_interface::CVRPSolver;

/* Here the behaviour of the sweep algorithm
 * is implemented.
 * The two dependencies refer to problem instance
 * trait and the CVRPSolver trait. */
pub struct SweepSolver<'a>
{
    pub(crate) instance : &'a dyn SweepInstanceTrait,

}

/* Methods of SweepSolver not
 * defined by the CVRPSolver trait. */
#[allow(dead_code)]
impl<'a> SweepSolver<'a>
{

    pub fn add_instance(&'a mut self, i : &'a dyn SweepInstanceTrait)
    {

        self.instance = i;

    }

}

/* Implementation of the CVRPSolver trait. */
impl<'a> CVRPSolver for SweepSolver<'a>
{

    fn solve(&self) -> Vec< Vec<usize>> {

        let instance : &dyn SweepInstanceTrait = self.instance;

        /* Compute other parameters for
         * further computation. */
        let nodes_demand       : Vec<usize> = instance.get_nodes_demand();
        let capacity           : usize      = instance.get_capacity();

        /* Compute a tour of the instance graph. */
        let ordered_nodes_list  : Vec<usize> = instance.order_nodes();
        let mut remaining_nodes : Vec<usize> = ordered_nodes_list.clone();

        /* Initialize the routes vector, without
         * knowing the final size of it. */
        let mut routes: Vec< Vec<usize>> = Vec::new();

        while !remaining_nodes.is_empty()
        {

            let mut remaining_capacity : i64        = capacity as i64;
            let mut current_route      : Vec<usize> = Vec::new();

            /* Iterate over the remaining nodes until
             * there is capacity available. */
            while remaining_capacity >= 0 &&
                !remaining_nodes.is_empty()
            {

                let current_node      : usize      = remaining_nodes.pop().unwrap();

                if (nodes_demand[current_node] as i64) <= remaining_capacity
                {

                    current_route.push(current_node);
                    remaining_capacity -= nodes_demand[current_node] as i64;

                }
                else
                {

                    /* Eventually, here we can execute a TSP algorithm
                     * over the nodes inside the already computed route,
                     * in order to increase the quality of the result. */

                    /* The current node is inserted in the same position. */
                    remaining_nodes.push(current_node);

                    /* If the capacity is not sufficient for the
                     * current node, we consider the route complete. */
                    remaining_capacity = -1;

                }

            }

            routes.push(current_route);

        }

        return routes;

    }

}