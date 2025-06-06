/*
 * lib/numrs/num.rs
 * Q@khaa.pk
 */

use super::{dimensions::Dimensions, collective::Collective};

pub struct Numrs;

pub trait One {
    fn one() -> Self;
}

impl One for i32 {
    fn one() -> Self { 1 }
}

impl One for f64 {
    fn one() -> Self { 1.0 }
}
// Add more impls as needed (u32, usize, etc.)

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for i32 {
    fn zero() -> Self { 0 }
}

impl Zero for f64 {
    fn zero() -> Self { 0.0 }
}
// Add more impls as needed (u32, usize, etc.)

impl Numrs {

    /// Creates a new `Collective<E>` filled with ones, based on the shape provided by `Dimensions`.
    /// 
    /// # Arguments
    ///
    /// * `like` - A `Dimensions` object specifying the desired shape of the one-filled array.
    ///
    /// # Returns
    ///
    /// * `Option<Collective<E>)` with all elements set to `E::default() or if some other trait is defined`, or with no data if the shape is zero.
    pub fn ones<E: One + Copy>(like: Dimensions) -> Option<Collective<E>> {
        let n = like.get_n();
        if n == 0 {
            // Return an empty shape if the size is zero
            return Some(Collective { 
                data: None,
                shape: Some(Box::new(Dimensions::new(0, 0))) // Wrap in Box::new()
            });
        }
    
        let allocation: Box<[E]> = vec![E::one(); n].into_boxed_slice();
    
        Some(Collective { 
            data: Some(allocation),
            shape: Some(Box::new(like)), // Wrap in Box::new()
        })
    }

    /// Creates a new `Collective<E>` filled with ones, based on the shape provided by `Dimensions`.
    /// 
    /// # Arguments
    ///
    /// * `like` - A `Dimensions` object specifying the desired shape of the one-filled array.
    ///
    /// # Returns
    ///
    /// * `Some(Collective<E>)` with all elements set to `E::default() or if some other trait is defined`, or with no data if the shape is zero.
    /*pub fn ones_old<E: One/*Default*/ + Copy>(like: Dimensions) -> Option<Collective<E>> {
        // You can allocate a Box<[E]> of zeros based on `like` shape
        
        let n = like.get_n();
        if n == 0 {

            // Return an empty shape if the size is zero
            return Some(Collective { data: None, 
                                     shape: Some(Dimensions::new(0, 0)) });
        }
        
        // Note: E::default() is used to represent "one"; avoids need for a separate `Zero` trait.
        let allocation: Box<[E]> = vec![E::one()/*default()*/; n].into_boxed_slice();
    
        Some(Collective { data: Some(allocation),
                          shape: Some(like),
        })
    }*/

    /// Creates a new `Collective<E>` filled with zeros, based on the shape provided by `Dimensions`.
    /// 
    /// # Arguments
    ///
    /// * `like` - A `Dimensions` object specifying the desired shape of the zero-filled array.
    ///
    /// # Returns
    ///
    /// * `Some(Collective<E>)` with all elements set to `E::default()`, or with no data if the shape is zero.
    /*pub fn zeros<E: Zero/*Default*/ + Copy>(like: Dimensions) -> Option<Collective<E>> {
        // You can allocate a Box<[E]> of zeros based on `like` shape
        
        let n = like.get_n();
        if n == 0 {

            // Return an empty shape if the size is zero
            return Some(Collective { data: None, 
                                     shape: Some(Dimensions::new(0, 0)) });
        }
        
        // Note: E::default() is used to represent "zero"; avoids need for a separate `Zero` trait.
        let allocation: Box<[E]> = vec![E::zero()/*default()*/; n].into_boxed_slice();
    
        Some(Collective { data: Some(allocation),
                          shape: Some(like),
        })
    }*/

    pub fn zeros<E: Zero + Copy>(like: Dimensions) -> Option<Collective<E>> {
    let n = like.get_n();
    if n == 0 {
        // Return an empty shape if the size is zero
        return Some(Collective { 
            data: None,
            shape: Some(Box::new(Dimensions::new(0, 0))) // Wrap in Box::new()
        });
    }
    
    let allocation: Box<[E]> = vec![E::zero(); n].into_boxed_slice();
    
    Some(Collective { 
        data: Some(allocation),
        shape: Some(Box::new(like)), // Wrap in Box::new()
    })
}
}
