/*
 * Here the interface of the CVRP algorithm
 * is defined as a trait.
 */

pub trait CVRPSolver
{

    fn solve(&self) -> Vec< Vec<usize>>;

}