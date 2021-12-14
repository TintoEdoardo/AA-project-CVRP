use std::fs;
use std::time::{Instant, Duration};

use crate::tsplib_parser::problem_instance::TSPInstance;
use crate::tsplib_parser::parser::parse;
use crate::algorithm_interface::CVRPSolver;
use crate::savings_algorithm::savings_algorithm_imp::SavingsSolver;
use crate::sweep_algorithm::sweep_algorithm_imp::SweepSolver;

mod savings_algorithm;
mod sweep_algorithm;
mod algorithm_interface;
mod model;
mod tsplib_parser;


fn main() {

    let mut contents: Vec<&str> = Vec::new();
    contents.push("./input/att-n48-k4.vrp");
    contents.push("./input/bayg-n29-k4.vrp");
    contents.push("./input/bays-n29-k5.vrp");
    contents.push("./input/dantzig-n42-k4.vrp");
    contents.push("./input/F-n45-k4.vrp");
    contents.push("./input/F-n72-k4.vrp");
    contents.push("./input/F-n135-k7.vrp");
    contents.push("./input/fri-n26-k3.vrp");
    contents.push("./input/gr-n17-k3.vrp");
    contents.push("./input/gr-n21-k3.vrp");
    contents.push("./input/gr-n24-k4.vrp");
    contents.push("./input/gr-n48-k3.vrp");
    contents.push("./input/swiss-n42-k5.vrp");
    contents.push("./input/ulysses-n16-k3.vrp");
    contents.push("./input/ulysses-n22-k4.vrp");

    for i in 0..contents.len()
    {

        /* Acquire input data. */
        let content : String = fs::read_to_string(contents[i])
            .expect("Something went wrong reading the file");

        /* Generate the instance graph. */
        let instance : TSPInstance = parse(&*content);
        let graph : model::GraphInstance = model::GraphInstance {
            instance
        };

        /* Initialize the two solvers and compute the routes. */
        let saving_solver : SavingsSolver = SavingsSolver {
            instance: &graph,
        };
        let savings_alg_start        : Instant         = Instant::now();
        let saving_routes            : Vec<Vec<usize>> = saving_solver.solve();
        let savings_alg_elapsed_time : Duration        = savings_alg_start.elapsed();

        let sweep_solver : SweepSolver = SweepSolver {
            instance: &graph,
        };
        let sweep_alg_start        : Instant         = Instant::now();
        let sweep_routes           : Vec<Vec<usize>> = sweep_solver.solve();
        let sweep_alg_elapsed_time : Duration        = sweep_alg_start.elapsed();

        /* Compute the cost of the routes. */
        let savings_routes_cost : f64 =
            model::compute_cost_of_routes(graph.instance.clone(), saving_routes);

        let sweep_routes_cost   : f64 =
            model::compute_cost_of_routes(graph.instance.clone(), sweep_routes);

        /* Print the result. */
        println!(" - - - - - - - - - - - - - - - ");
        println!("Instance: {}", graph.instance.specification.name.clone());
        println!("Savings Algorithm results: ");
        println!("Routes cost = {cost}, Time required = {time}",
                 cost = savings_routes_cost,
                 time = savings_alg_elapsed_time.as_nanos());
        println!("Sweep Algorithm results: ");
        println!("Routes cost = {cost}, Time required = {time}",
                 cost = sweep_routes_cost,
                 time = sweep_alg_elapsed_time.as_nanos());
        println!();

    }

}
