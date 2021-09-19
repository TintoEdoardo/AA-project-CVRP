/*
 * This module implements a StdVertex.
 *
 * STD_VERTEX:
 *  - Methods:
 *    GET_ID: (&self) -> u16
 */

use crate::model::VertexTrait;

struct StdVertex
{

    id : u16

}

/* StdVertex implements the Vertex trait. */
impl VertexTrait for StdVertex
{

    fn get_id(&self) -> u16
    {

        self.id

    }

}


