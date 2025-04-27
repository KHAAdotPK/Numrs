/*
 * lib/numrs/num.rs
 * Q@khaa.pk
 */

use super::{dimensions::Dimensions, collective::Collective};

pub struct Numrs;

impl Numrs {
    pub fn zeros<E: Default + Copy>(like: Dimensions) -> Option<Collective<E>> {
        // You can allocate a Box<[E]> of zeros based on `like` shape
        // But for now, let's just leave data empty and set the shape

        Some(Collective {
            data: None,  // or Some(allocation) if you implement it
            shape: Some(like),
        })
    }
}
