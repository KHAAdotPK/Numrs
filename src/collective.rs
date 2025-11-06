/*
 * Numrs/src/collective.rs
 * Q@khaa.pk
 */

use std::ops::{Index, IndexMut};
use super::dimensions::Dimensions; 
use crate::header::Axis;

 pub struct Collective<E = f64> {
    //pub data: Option<Box<[E]>>,    // Option means maybe allocated, maybe not yet
    //pub shape: Option<Dimensions>, // shape can also be deferred

    pub data: Option<Box<[E]>>,    // Data can be allocated or not
    pub shape: Option<Box<Dimensions>>, // Shape can be deferred, meaning it can be set later
}

// Implement Index trait for Collective to read-only access elements by single index (collective[index])
impl<E> Index<usize> for Collective<E> {
    type Output = E;

    fn index(&self, index: usize) -> &Self::Output {
        match &self.data {
            Some(buffer) => &buffer[index],
            None => panic!("Collective data is not allocated"),
        }
    }
}

// Implement IndexMut trait for Collective to modify elements by single index (collective[index] = value)
impl<E> IndexMut<usize> for Collective<E> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match &mut self.data {
            Some(buffer) => &mut buffer[index],
            None => panic!("Collective data is not allocated"),
        }
    }
}

impl<E> Collective<E> where E: Default + Copy {

    /*
        Associated constructor function for creating a new Collective instance.
        This is the most flexible constructor that allows deferred initialization.
    
        Arguments:
        - data: Option<Box<[E]>> - Optional heap-allocated array of elements of type E.
                                 None means data allocation is deferred until later.
                                 Some(buffer) means data is immediately available.
        - shape: Option<Box<Dimensions>> - Optional heap-allocated shape information.
                                         None means shape definition is deferred until later.
                                         Some(dims) means shape is immediately defined.
    
        Return Value:
        - Self - Returns a new Collective<E> instance that can be in one of four states:
               * Both data and shape deferred: Collective { data: None, shape: None }
               * Only data allocated: Collective { data: Some(buffer), shape: None }
               * Only shape defined: Collective { data: None, shape: Some(dims) }
               * Fully initialized: Collective { data: Some(buffer), shape: Some(dims) }
             
        This constructor enables lazy initialization patterns common in tensor operations
        where shape or data might be determined at different stages of computation.
     */
    pub fn new(data: Option<Box<[E]>>, shape: Option<Box<Dimensions>>) -> Self {

        Self { data: data, shape: shape }
    }
                
    /*      
        Static-like functions tied to the type (called with Type::function())
        This and functions like new() are associated helper functions.
        Associated helper function to create a new Collective from a shape.
        This function allocates a new buffer based on the shape's dimensions.
        The buffer is filled with default values of type E.
        The shape is also stored in the Collective.
        A function becomes an associated function when it does not receive `self` as an argument.         
        Distinction is that associated functions are defined by what they don't have (self parameter), not by what they return.     

        Arguments:
        - shape: Box<Dimensions> - A heap-allocated Dimensions object that defines the tensor shape.
                                   The Box ensures the shape data is stored on the heap and ownership
                                   is transferred to this function.
    
        Return Value:
        - Self - Returns a new instance of Collective<E> with:
                 * data: Some(buffer) - A heap-allocated array filled with E::default() values
                 * shape: Some(shape) - The provided shape dimensions
                
          The returned Collective is fully initialized and ready for use.
     */
    pub fn from_shape(shape: Box<Dimensions>) -> Self {

        let size = shape.get_n();
        let buffer = vec![E::default(); size].into_boxed_slice();

        Self { data: Some(buffer), shape: Some(shape) }
    }

    /*pub fn slice(&self, start: usize, end: usize) -> Collective<E> {

        let size = end - start;
        let buffer: Box<[E]> = self.data.unwrap()[start..end].into_boxed_slice();

        Collective { data: Some(buffer), shape: self.shape.clone() }
    }*/

    /// Creates a new `Collective` containing a copy of a slice of the original data.
    ///
    /// This method performs the following checks and operations:
    /// 1. Handles the case where the original `Collective` has no data (`None`).
    /// 2. Validates that the `start` and `end` indices are within the bounds of the data.
    /// 3. Creates a new, owned buffer (`Box<[E]>`) by copying the specified range of data.
    /// 4. Creates a new `Dimensions` object that correctly represents the shape of the slice (a 1D vector).
    /// 5. Returns a new, fully-initialized `Collective` with the new data and shape.
    ///
    /// # Arguments
    /// * `start` - The starting index of the slice (inclusive).
    /// * `end` - The ending index of the slice (exclusive).
    /// * `shape` - The shape of the slice. This is a heap-allocated Dimensions object that defines the tensor shape.
    /// * `axis` - The axis along which to slice the data.
    ///
    /// TODO,
    /// I actually wanted to avoid the cloning (when calling the method) but as previously I was calling it by cloning it and for that I was hoping to convert it to reference in its argument list....
    /// but now cloning is happening inside the method so i think i leave it as it is for now....?
    ///
    /// # Panics
    /// This method will panic if `start > end` or if `end` is greater than the length of the data.
    ///
    /// # Returns
    /// A new `Collective<E>` instance. If the original data is `None`, it returns an
    /// empty `Collective` with `None` for both data and shape.
    pub fn get_slice(&self, start: usize, end: usize, shape: Box<Dimensions>, axis: Axis) -> Box<Collective<E>> {

        if axis == Axis::None {
                    
            // First, check if there is any data to slice.
            // Using `if let` is a safe way to handle the Option without unwrapping.
            if let Some(data) = &self.data {
                // Check for valid slice range to prevent panics.
                if start > end || end > data.len() {
                    panic!(
                        "Slice indices out of bounds. start: {}, end: {}, len: {}",
                        start,
                        end,
                        data.len()
                    );
                }

                // Create a new buffer by copying the data from the slice.
                // .to_vec() creates an owned Vec<E>, which is then converted into a Box<[E]>.
                let new_buffer = data[start..end].to_vec().into_boxed_slice();
                let new_size = new_buffer.len();

                // The shape of the slice is a new 1D dimension.
                // We create a new Dimensions object to reflect this.
                /*let new_shape = Some(Box::new(Dimensions::new(vec![new_size])));*/

                // Return the new Collective with the copied data and new shape.
                Box::new(Collective {
                    data: Some(new_buffer),
                    shape: Some(shape),
                })
            } else {
                // If the original Collective has no data, the slice should also be empty.
                Box::new(Collective {
                    data: None,
                    shape: None,
                })
            }
        } else if axis == Axis::Columns {

            unimplemented!("get_slice(), Axis::Columns: This feature is still in development");
        } else if axis == Axis::Rows {

            unimplemented!("get_slice(), Axis::Rows: This feature is still in development");
        }
        else {

            panic!("get_slice(): Panicked!");
        }
    }
}
