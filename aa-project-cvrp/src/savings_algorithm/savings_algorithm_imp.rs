use crate::savings_algorithm::savings_instance_trait::SavingsInstanceTrait;
use crate::algorithm_interface::CVRPSolver;

/* Here the behaviour of the savings algorithm
 * is implemented.
 * The two dependencies refer to problem instance
 * trait and the CVRPSolver trait. */
pub struct SavingsSolver<'a>
{

    instance : &'a dyn SavingsInstanceTrait,

}

/* Methods of SavingsSolver not
 * defined by the CVRPSolver trait. */
impl<'a> SavingsSolver<'a>
{

    pub fn add_instance(&'a mut self, i : &'a dyn SavingsInstanceTrait)
    {

        self.instance = i;

    }

}

/* Implementation of the CVRPSolver trait. */
impl<'a> CVRPSolver for SavingsSolver<'a>
{

    fn solve(&self) -> Vec< Vec<usize>>
    {

        let instance : &dyn SavingsInstanceTrait = self.instance;

        /* Compute other parameters for
         * further computation. */
        let nodes_list         : Vec<usize> = instance.get_nodes_list();
        let nodes_demand       : Vec<usize> = instance.get_nodes_demand();
        let node_number        : usize      = nodes_list.len();
        let mut node_to_routes : Vec<usize> = Vec::with_capacity(node_number);
        let mut routes_weight  : Vec<usize> = Vec::with_capacity(node_number);

        /* We assume a single depot. */
        // let depot_index : usize = 0;
        // let depot_id    : usize = nodes_list[depot_index];

        /* Compute savings, regardless of
         * instance type.
         * Each element of savings is a tuple
         * (i, j, w(i,g)) where i and j are two incident
         * nodes for an edge e in E, and w(i,g) is
         * the weight of the edge e. */
        let mut savings : Vec<(usize, usize, usize)> = instance.compute_savings();

        /* Sort in increasing order.
         * Therefore the last element is always
         * the maximum. */
        savings.sort_by_key(|s : &(usize, usize, usize)| s.2);

        /* Compute the initial set of routes and
         * the array node_to_route, where the i-th
         * element is the id of the route to which
         * the i-th node belong.
         * Finally computes the routes_weight vector,
         * where the i-th element contains the weight
         * of the i-th routes. */
        let mut routes : Vec< Vec<usize>> = Vec::new();
        for i in 1..node_number
        {

            /* At first we define as many routes as
             * nodes in te graph. */
            routes[i].push(nodes_list[i]);

            /* At this point, each node is associated
             * to one distinct route. */
            node_to_routes[i] = i;

            /* Initially the weight of the i-th
             * route is given by the demand of
             * the only nodes in it (which is the
             * i-th node). */
            routes_weight[i] = nodes_demand[i];

        }

        /* Generate a vector where each i-th value
         * is true if the i-th node is terminal,
         * false otherwise. */
        let mut is_node_terminal : Vec<bool> = vec![];
        for i in 1..node_number
        {

            is_node_terminal[i] = true;

        }

        /* Merge the routes according to
         * Clark and Wright's algorithm. */
        while !savings.is_empty()
        {

            /* Take the maximum element
             * inside savings out of the vector. */
            let s : (usize, usize, usize) = savings.pop().unwrap();

            let i : usize = s.0;
            let j : usize = s.1;

            /* Check if it is possible to
             * merge the two routes. */
            if node_to_routes[i] != node_to_routes[j] &&
                is_node_terminal[i] &&
                is_node_terminal[j] &&
                routes_weight[i] + routes_weight[j] < instance.get_capacity()
            {

                let route_of_i : usize = node_to_routes[i];
                let route_of_j : usize = node_to_routes[j];

                routes[route_of_i].append(&mut routes[route_of_j]);
                routes[route_of_j].clear();

                routes_weight[route_of_i] += routes_weight[route_of_j];
                node_to_routes[j] = route_of_i;

                /* Check if i and j are still terminal.
                 * If a node is terminal, then it must be
                 * the first or the last in the routes. */
                let first_node_in_route : usize = *routes[route_of_i].first().unwrap();
                let last_node_in_route  : usize = *routes[route_of_i].last().unwrap();

                if first_node_in_route == i || last_node_in_route == i
                {
                    is_node_terminal[i] = true;
                }
                else
                {
                    is_node_terminal[i] = false;
                }

                if first_node_in_route == j || last_node_in_route == j
                {
                    is_node_terminal[j] = true;
                }
                else
                {
                    is_node_terminal[j] = false;
                }

            }

            /* If is not possible to merge
             * the two routes, we simply discard
             * the savings value.
             * Therefore no operation is required. */

        }

        return routes;

    }

}