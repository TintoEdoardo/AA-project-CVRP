/*
 * Values for each keywords.
 */

#[derive(Clone)]
#[allow(non_camel_case_types, dead_code)]
pub enum TYPE
{

    TSP, /* Data for symmetric traveling salesman problem. */
    ATSP, /* Data for asymmetric traveling salesman problem. */
    SOP, /* Data for sequential ordering problem. */
    HCP, /* Hamilton cycle problem data. */
    CVRP, /* Capacitated .vehicle routing problem data. */
    TOUR /* A collection of tours. */

}

#[derive(Clone, Eq, PartialEq)]
#[allow(non_camel_case_types, dead_code)]
pub enum EDGE_WEIGHT_TYPE
{

    EXPLICIT, /* Weights are listed explicitly in the corresponding section. */
    EUC_2D, /* Weights are Eucledean distances in 2-D. */
    EUC_3D, /* Weights are Eucledean distances in 3-D. */
    MAX_2D, /* Weights are maximum distances in 2-D. */
    MAX_3D, /* Weights are maximum distances in 3-D. */
    MAN_2D, /* Weights are Manhattan distances in 2-D. */
    MAN_3D, /* Weights are Manhattan distances in 3-D. */
    CEIL_2D, /* Weights are Eucledean distances in 2-D rounded up. */
    GEO, /* Weights are geographical distances. */
    ATT, /* Special distance function for problem att48 and att532. */
    XRAY1, /* Special distance function for crystallography problems (version 1). */
    XRAY2, /* Special distance function for crystallography problems (version 2). */
    SPECIAL, /* There is a special distance function documented elsewhere. */

}

#[derive(Clone, Eq, PartialEq)]
#[allow(non_camel_case_types, dead_code)]
pub enum EDGE_WEIGHT_FORMAT {

    FUNCTION, /* Weights are given by a function. */
    FULL_MATRIX, /* Weights are given by a full matrix. */
    UPPER_ROW, /* Upper triangular matrix (row-wise without diagonal entries). */
    LOWER_ROW, /* Lower triangular matrix (row-wise without diagonal entries). */
    UPPER_DIAG_ROW, /* Upper triangular matrix (row-wise including diagonal entries). */
    LOWER_DIAG_ROW, /* Lower triangular matrix (row-wise including diagonal entries). */
    UPPER_COL, /* Upper triangular matrix (column-wise without diagonal entries). */
    LOWER_COL, /* Lower triangular matrix (column-wise without diagonal entries). */
    UPPER_DIAG_COL, /* Upper triangular matrix (column-wise including diagonal entries). */
    LOWER_DIAG_COL /* Lower triangular matrix (column-wise including diagonal entries). */

}

#[derive(Clone)]
#[allow(non_camel_case_types, dead_code)]
pub enum EDGE_DATA_FORMAT {

    EDGE_LIST, /* The graph is given by an edge list. */
    ADJ_LIST /* The graph is given by an adjacency list. */

}

#[derive(Clone)]
#[allow(non_camel_case_types, dead_code)]
pub enum NODE_COORD_TYPE {

    TWOD_COORDS, /* Nodes are specified by coordinates in 2-D. */
    THREED_COORDS, /* Nodes are specified by coordinates in 3-D. */
    NO_COORDS /* The nodes do not have associated coordinates. */

}

#[derive(Clone)]
#[allow(non_camel_case_types, dead_code)]
pub enum DISPLAY_DATE_TYPE {

    COORD_DISPLAY, /* Display is generated from the node coordinates. */
    TWOD_DISPLAY, /* Explicit coordinates in 2-D are given. */
    NO_DISPLAY /* N graphical display is possible. */

}