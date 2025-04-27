/*
 * lib/numrs/collective.rs
 * Q@khaa.pk
 */

use super::dimensions::Dimensions; 

 pub struct Collective<E = f64> {
    pub data: Option<Box<[E]>>,     // Option means maybe allocated, maybe not yet
    pub shape: Option<Dimensions>, // shape can also be deferred
}

impl<E> Collective<E> where E: Default + Copy {

    pub fn new() -> Self {
        Self { data: None, shape: None }
    }
}

