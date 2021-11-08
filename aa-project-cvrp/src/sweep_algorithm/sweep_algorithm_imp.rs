use crate::sweep_algorithm::sweep_instance_trait::SweepInstanceTrait;
use crate::algorithm_interface::CVRPSolver;

/* Here the behaviour of the sweep algorithm
 * is implemented.
 * The two dependencies refer to problem instance
 * trait and the CVRPSolver trait. */
pub struct SweepSolver<'a>
{

    instance : &'a dyn SweepInstanceTrait,

}

/* Methods of SavingsSolver not
 * defined by the CVRPSolver trait. */
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

    fn solve(&self) -> Vec<Vec<usize>> {

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

        /* Initialize the index of the current routes. */
        let current_route_index : usize = 0;

        while !remaining_nodes.is_empty()
        {

            let mut remaining_capacity : i64        = capacity as i64;
            let mut current_route      : Vec<usize> = Vec::new();

            /* Iterate over the remaining nodes until
             * there is capacity available. */
            while remaining_capacity > 0
            {

                let current_node      : usize      = remaining_nodes.pop().unwrap();

                if (nodes_demand[current_node] as i64) < remaining_capacity
                {

                    current_route.push(current_node);
                    remaining_capacity -= nodes_demand[current_node] as i64;

                }
                else
                {

                    /* If the capacity is not sufficient, for the
                     * current node, we consider the route complete. */
                    remaining_capacity = -1;

                    /* Eventually, here we can execute a TSP algorithm
                     * over the nodes inside the already computed route,
                     * in order to increase the quality of the result. */

                }

            }

            routes.push(current_route);

        }

        return routes;

    }

}