/*
 * lib/numrs/collective.rs
 * Q@khaa.pk
 */

use super::dimensions::Dimensions; 

 pub struct Collective<E = f64> {
    //pub data: Option<Box<[E]>>,    // Option means maybe allocated, maybe not yet
    //pub shape: Option<Dimensions>, // shape can also be deferred

    pub data: Option<Box<[E]>>,    // Data can be allocated or not
    pub shape: Option<Box<Dimensions>>, // Shape can be deferred, meaning it can be set later
}

impl<E> Collective<E> where E: Default + Copy {

    pub fn new(data: Option<Box<[E]>>, shape: Option<Box<Dimensions>>) -> Self {
        Self { data: data, shape: shape }
    }
}
