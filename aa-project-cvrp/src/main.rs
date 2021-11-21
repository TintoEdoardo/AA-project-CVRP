use std::fs;
use crate::tsplib_parser::problem_instance::TSPInstance;
use crate::tsplib_parser::parser::parse;
use crate::algorithm_interface::CVRPSolver;
use crate::savings_algorithm::savings_algorithm_imp::SavingsSolver;
use crate::model::GraphInstance;

mod savings_algorithm;
mod sweep_algorithm;
mod algorithm_interface;
mod model;
mod tsplib_parser;


fn main() {

    let contents = fs::read_to_string("./input/bayg-n29-k4.vrp")
        .expect("Something went wrong reading the file");

    /* let contents
        = "NAME : att48
TYPE : CVRP
COMMENT : (Rinaldi,Yarrow/Araque, No of trucks: 4, Optimal value: 40002)
DIMENSION : 48
CAPACITY : 15
EDGE_WEIGHT_TYPE : EUC_2D";*/

    let instance : TSPInstance = parse(&*contents);

    let graph : GraphInstance = GraphInstance {
        instance
    };

    let solver : SavingsSolver = SavingsSolver {
        instance: &graph,
    };

    let route : Vec<Vec<usize>> = solver.solve();

    println!("PRINT THE ROUTES: ");
    for i in 0..route.len()
    {
        if route[i].len() > 0
        {
            println!("Route {ri} : ", ri = i);

            for j in 0..route[i].len()
            {
                print!(" {nj}, ", nj = route[i][j]);
            }
            print!("\n");
        }
    }

}
