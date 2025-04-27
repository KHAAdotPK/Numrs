/*
 * lib/numrs/tests/Dimensions_Test.rs
 * Q@khaa.pk
 */

#[cfg(test)]

#[path = "../lib/numrs/mod.rs"]
mod numrs;
 
use std::rc::Rc;
use std::cell::RefCell;
use numrs::dimensions::Dimensions;

mod tests {
    use super::*; // Import everything from outer scope!

    #[test]
    fn test_valid_chain() {
        let dim3 = Dimensions::new(0, 0).with_columns(5).with_rows(10);
        let dim2 = Dimensions::new(0, 0).with_columns(0).with_rows(10).with_next(Rc::new(RefCell::new(dim3)));
        let dim1 = Dimensions::new(0, 0).with_columns(0).with_rows(10).with_next(Rc::new(RefCell::new(dim2)));

        assert!(dim1.is_valid(), "Valid chain passed validation.");
    }
 
    #[test]
    fn test_invalid_zero_rows() {
        let dim1 = Dimensions::new(0, 0).with_columns(5).with_rows(0); // rows = 0, invalid
 
        assert!(!dim1.is_valid(), "Invalid chain (zero rows) passed validation.");
    }
 
    #[test]
    fn test_invalid_zero_columns_with_next() {
        let dim2 = Dimensions::new(0, 0).with_columns(0).with_rows(10);
        let dim1 = Dimensions::new(0, 0).with_columns(0).with_rows(10).with_next(Rc::new(RefCell::new(dim2)));
 
        assert!(!dim1.is_valid(), "Invalid chain (zero columns with next) passed validation.");
    }
 
    #[test]
    fn test_valid_nonzero_columns_at_end() {
        let dim1 = Dimensions::new(0, 0).with_columns(5).with_rows(10); // no next, but columns != 0
 
        assert!(dim1.is_valid(), "Invalid chain (non-zero columns at end) passed validation.");
    }
 
    #[test]
    fn test_single_node_valid() {
        let dim1 = Dimensions::new(0, 0).with_columns(5).with_rows(10); // no next, columns = 0
 
        assert!(dim1.is_valid(), "Single valid node passsed validation.");
    }
 
    #[test]
    fn test_chain_with_middle_node_invalid() {
        let dim3 = Dimensions::new(0, 0).with_columns(5).with_rows(10);
        let dim2 = Dimensions::new(0, 0).with_columns(5).with_rows(10).with_next(Rc::new(RefCell::new(dim3))); // rows = 0
        let dim1 = Dimensions::new(0, 0).with_columns(0).with_rows(10).with_next(Rc::new(RefCell::new(dim2)));
 
        assert!(!dim1.is_valid(), "Chain with invalid middle node passed validation.");
    }
}
 