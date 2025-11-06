/*
 * Numrs/src/header.rs
 * Q@khaa.pk
 */

/* 
    Mental Model:
    - axis=0 -> Going down the rows (operate along rows)
    - axis=1 -> Going across the columns (operate along columns) 
    - axis=-1 -> The last axis (columns in 2D array)

    The axis along which a particular operation should be performed. 
    The axis=-1, is the last axis. 
    The axis=1, second axis (in two dimensional array it means the columns)
    The axis=0, first axis (in two dimensional array it means rows)
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /*
        In Python, using "axis=-1" implies an operation along the last axis. 
        In the context of our 2D array, this corresponds to columns.
        In Numpy, Axis::Column is equivalent to Python's "axis=-1". 
        This means that only the last axis of the two arrays can have different sizes.

        For instance, in the scenario "a[10][256] concatenate b[10][10]", 
        the result will be a new array with a shape of [10][266].
        However, attempting "a[5][256] concatenate b[10][10]" would result in an error 
        since the number of rows is different.
    */  
    Columns = 1,  // Operate along columns, horizontal operations
    
    /*
        Specifies that the operation should be done across rows.
        The resulting array will have the same number of columns as the input vectors.
     */
    Rows = 0,    // Operate along rows, vertical operations
    
    /*
        For operations that flatten the entire array to a single scalar value.
        Results in one-dimensional output regardless of input shape.
     */
    None = 2,    // Flatten all dimensions to scalar
}