/* This trait define the method to
 * order the nodes according to the
 * sweep algorithm, available only
 * for GraphInstance when explicitly
 * implemented. */
pub trait SweepInstanceTrait
{

    fn order_nodes(&self) -> Vec<usize>;

    fn get_capacity(&self) -> usize;

    fn get_nodes_demand(&self) -> Vec<usize>;

}