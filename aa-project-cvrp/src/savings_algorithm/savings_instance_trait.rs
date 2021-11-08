/* This trait define the method to
 * compute savings, available only
 * for GraphInstance when explicitly
 * implemented. */
pub trait SavingsInstanceTrait
{

    fn compute_savings(&self) -> Vec<(usize, usize, usize)>;

    fn get_capacity(&self) -> usize;

    fn get_nodes_list(&self) -> Vec<usize>;

    fn get_nodes_demand(&self) -> Vec<usize>;

}