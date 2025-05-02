/*
 * lib/numrs/tests/Dimensions_Test.rs
 * Q@khaa.pk
 */

/*
    cargo test -- --nocapture
    
    cargo test test_ones -- --nocapture

    cargo test test_zeros -- --nocapture
 */ 

#[cfg(test)]
#[path = "../lib/numrs/mod.rs"]
mod numrs;
 
use std::rc::Rc;
use std::cell::RefCell;
use numrs::dimensions::Dimensions;
use numrs::num::Numrs;
use numrs::collective::Collective;

mod tests {
    use super::*; // Import everything from outer scope!

    #[test]
    fn test_zeros() {
        let dim3 = Dimensions::new(0, 0).with_columns(5).with_rows(10);
        let dim2 = Dimensions::new(0, 0).with_columns(0).with_rows(10).with_next(Rc::new(RefCell::new(dim3)));
        let dim1 = Dimensions::new(0, 0).with_columns(0).with_rows(10).with_next(Rc::new(RefCell::new(dim2)));

        let mut result: Option<Collective<f64>> = Numrs::zeros::<f64>(dim1);
        assert!(result.is_some(), "Numrs::zeros should return Some for valid dimensions"); 

        if let Some(collective) = result {
            println!("Data: {:?}", collective.data);
            println!("Shape: {:?}", collective.shape);
        }
    }
    
    #[test]
    fn test_ones() {
        let dim3 = Dimensions::new(0, 0).with_columns(5).with_rows(10);
        let dim2 = Dimensions::new(0, 0).with_columns(0).with_rows(10).with_next(Rc::new(RefCell::new(dim3)));
        let dim1 = Dimensions::new(0, 0).with_columns(0).with_rows(10).with_next(Rc::new(RefCell::new(dim2)));

        let mut result: Option<Collective<f64>> = Numrs::ones::<f64>(dim1);
        assert!(result.is_some(), "Numrs::ones should return Some for valid dimensions"); 

        if let Some(collective) = result {
            println!("Data: {:?}", collective.data);
            println!("Shape: {:?}", collective.shape);
        }
    }          
}
 