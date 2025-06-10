/*
 * lib/numrs/collective.rs
 * Q@khaa.pk
 */

use std::ops::{Index, IndexMut};

use super::dimensions::Dimensions; 

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
}
