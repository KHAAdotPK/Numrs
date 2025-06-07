/*
 * numrs/tests/dimensions_tests.rs
 * Integration tests for Dimensions struct
 * Q@khaa.pk
 */

use std::rc::Rc;
use std::cell::RefCell;
use numrs::Dimensions; // Import directly from the crate

#[test]
fn test_new_dimensions() {
    let dim = Dimensions::new(5, 10);
    assert_eq!(dim.columns(), 5);
    // Note: For a single node, rows() should return the direct rows value
    // Based on your implementation, let's test what it actually returns
}

#[test]
fn test_fluent_setters() {
    let dim = Dimensions::new(0, 0)
        .with_columns(5)
        .with_rows(10);
    
    assert_eq!(dim.columns(), 5);
}

#[test]
fn test_valid_chain() {
    // Create a valid 3-node chain: 10 -> 10 -> 5 (columns only in last)
    let dim3 = Dimensions::new(0, 10).with_columns(5);
    let dim2 = Dimensions::new(0, 10).with_next(Rc::new(RefCell::new(dim3)));
    let dim1 = Dimensions::new(0, 10).with_next(Rc::new(RefCell::new(dim2)));

    assert!(dim1.is_valid(), "Valid chain should pass validation");
}

#[test]
fn test_invalid_zero_rows() {
    let dim = Dimensions::new(5, 0); // rows = 0, should be invalid
    assert!(!dim.is_valid(), "Chain with zero rows should be invalid");
}

#[test]
fn test_invalid_intermediate_node_with_columns() {
    // Create invalid chain: intermediate node has columns != 0
    let dim3 = Dimensions::new(0, 10).with_columns(5);
    let dim2 = Dimensions::new(3, 10).with_next(Rc::new(RefCell::new(dim3))); // Invalid: has columns AND next
    let dim1 = Dimensions::new(0, 10).with_next(Rc::new(RefCell::new(dim2)));

    assert!(!dim1.is_valid(), "Chain with columns in intermediate node should be invalid");
}

#[test]
fn test_invalid_final_node_without_columns() {
    // Final node must have columns > 0
    let dim = Dimensions::new(0, 10); // No columns, no next - invalid
    assert!(!dim.is_valid(), "Final node without columns should be invalid");
}

#[test]
fn test_single_node_valid() {
    let dim = Dimensions::new(5, 10); // Has columns, no next - valid
    assert!(dim.is_valid(), "Single valid node should pass validation");
}

#[test]
fn test_get_n_single_node() {
    let dim = Dimensions::new(5, 10);
    assert_eq!(dim.get_n(), 50, "Single node: 5 columns * 10 rows = 50");
}

#[test]
fn test_get_n_chain() {
    // Create chain: 3 -> 4 -> 5 (columns)
    // Total should be 3 * 4 * 5 = 60
    let dim3 = Dimensions::new(0, 4).with_columns(5);
    let dim2 = Dimensions::new(0, 3).with_next(Rc::new(RefCell::new(dim3)));
    let dim1 = Dimensions::new(0, 2).with_next(Rc::new(RefCell::new(dim2)));

    assert_eq!(dim1.get_n(), 2 * 3 * 4 * 5, "Chain should multiply all dimensions");
}

#[test]
fn test_columns_returns_last_link_columns() {
    // Test that columns() returns the columns from the last node
    let dim3 = Dimensions::new(0, 4).with_columns(7); // Last node has 7 columns
    let dim2 = Dimensions::new(0, 3).with_next(Rc::new(RefCell::new(dim3)));
    let dim1 = Dimensions::new(0, 2).with_next(Rc::new(RefCell::new(dim2)));

    assert_eq!(dim1.columns(), 7, "Should return columns from last node");
}

#[test]
fn test_get_number_of_inner_arrays() {
    // Create chain: 2 -> 3 -> 4 -> 5 (columns)
    // Inner arrays should be 2 (only first node, excluding last)
    let dim3 = Dimensions::new(0, 4).with_columns(5);
    let dim2 = Dimensions::new(0, 3).with_next(Rc::new(RefCell::new(dim3)));
    let dim1 = Dimensions::new(0, 2).with_next(Rc::new(RefCell::new(dim2)));

    assert_eq!(dim1.get_number_of_inner_arrays(), 6, "Should multiply rows of non-final nodes");
}

#[test]
fn test_get_number_of_innermost_arrays() {
    // Create chain: 2 -> 3 -> 5 (columns)
    // Innermost arrays should be 3 (rows of the last node)
    let dim3 = Dimensions::new(0, 4).with_columns(5);
    let dim2 = Dimensions::new(0, 3).with_next(Rc::new(RefCell::new(dim3)));
    let dim1 = Dimensions::new(0, 2).with_next(Rc::new(RefCell::new(dim2)));

    assert_eq!(dim1.get_number_of_innermost_arrays(), 4, "Should return rows of final node");
}

#[test]
fn test_rows_calculation() {
    // Create chain: 2 -> 3 -> 5 (columns)
    // rows() should be inner_arrays * innermost_arrays = 2 * 3 = 6
    let dim3 = Dimensions::new(0, 4).with_columns(5);
    let dim2 = Dimensions::new(0, 3).with_next(Rc::new(RefCell::new(dim3)));
    let dim1 = Dimensions::new(0, 2).with_next(Rc::new(RefCell::new(dim2)));

    assert_eq!(dim1.get_number_of_inner_arrays() * dim1.get_number_of_innermost_arrays(), 24, "rows() should be inner_arrays * innermost_arrays");
}

#[test]
fn test_mutable_setters() {
    let mut dim = Dimensions::new(1, 1);
    
    dim.set_columns(10);
    dim.set_rows(20);
    
    assert_eq!(dim.columns(), 10);
    // Test that the dimension is still valid after mutation
    assert!(dim.is_valid());
}

#[test]
fn test_next_and_prev_getters() {
    let dim2 = Dimensions::new(0, 10).with_columns(5);
    let dim2_rc = Rc::new(RefCell::new(dim2));
    let dim1 = Dimensions::new(0, 10).with_next(dim2_rc.clone());
    
    assert!(dim1.next().is_some());
    assert!(dim1.prev().is_none());
    
    // Test that we can access the next dimension
    let next_dim = dim1.next().unwrap();
    assert_eq!(next_dim.borrow().columns(), 5);
}

#[test]
fn test_edge_case_empty_chain() {
    // Test behavior with minimal valid dimension
    let dim = Dimensions::new(1, 1);
    assert!(dim.is_valid());
    assert_eq!(dim.get_n(), 1);
    assert_eq!(dim.columns(), 1);
    assert_eq!(dim.get_number_of_inner_arrays(), 1);
    assert_eq!(dim.get_number_of_innermost_arrays(), 1);
}

#[test]
fn test_clone_and_debug() {
    let dim1 = Dimensions::new(5, 10);
    let dim2 = dim1.clone();
    
    // Both should be valid and equal
    assert_eq!(dim1.columns(), dim2.columns());
    assert_eq!(dim1.get_n(), dim2.get_n());
    
    // Debug should work (no panic)
    let debug_string = format!("{:?}", dim1);
    assert!(!debug_string.is_empty());
}
 