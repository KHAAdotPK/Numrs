/*
 * numrs/tests/num_tests.rs
 * Comprehensive tests for num.rs module
 * Q@khaa.pk
 */

/*
    cargo test -- --nocapture
    
    cargo test test_ones -- --nocapture
    cargo test test_zeros -- --nocapture
    cargo test num_tests -- --nocapture
*/

use std::rc::Rc;
use std::cell::RefCell;
use numrs::dimensions::Dimensions;
use numrs::collective::Collective;

// Import the traits and struct from num module
use numrs::num::{Numrs, One, Zero};

#[test]
fn test_one_trait_i32() {
    let one_i32 = i32::one();
    assert_eq!(one_i32, 1, "i32::one() should return 1");
}

#[test]
fn test_one_trait_f64() {
    let one_f64 = f64::one();
    assert_eq!(one_f64, 1.0, "f64::one() should return 1.0");
}

#[test]
fn test_zero_trait_i32() {
    let zero_i32 = i32::zero();
    assert_eq!(zero_i32, 0, "i32::zero() should return 0");
}

#[test]
fn test_zero_trait_f64() {
    let zero_f64 = f64::zero();
    assert_eq!(zero_f64, 0.0, "f64::zero() should return 0.0");
}

#[test]
fn test_zeros_single_dimension() {
    let dim = Dimensions::new(5, 10); // 5 columns, 10 rows = 50 elements
    
    let result: Option<Collective<f64>> = Numrs::zeros::<f64>(dim);
    assert!(result.is_some(), "Numrs::zeros should return Some for valid dimensions");
    
    if let Some(collective) = result {
        assert!(collective.data.is_some(), "Data should be allocated");
        assert!(collective.shape.is_some(), "Shape should be set");
        
        if let Some(data) = &collective.data {
            assert_eq!(data.len(), 50, "Data should have 50 elements");
            // Check that all elements are zero
            for &value in data.iter() {
                assert_eq!(value, 0.0, "All elements should be zero");
            }
        }
        
        if let Some(shape) = &collective.shape {
            assert_eq!(shape.get_n(), 50, "Shape should indicate 50 total elements");
            assert_eq!(shape.columns(), 5, "Shape should have 5 columns");
        }
    }
}

#[test]
fn test_ones_single_dimension() {
    let dim = Dimensions::new(3, 4); // 3 columns, 4 rows = 12 elements
    
    let result: Option<Collective<f64>> = Numrs::ones::<f64>(dim);
    assert!(result.is_some(), "Numrs::ones should return Some for valid dimensions");
    
    if let Some(collective) = result {
        assert!(collective.data.is_some(), "Data should be allocated");
        assert!(collective.shape.is_some(), "Shape should be set");
        
        if let Some(data) = &collective.data {
            assert_eq!(data.len(), 12, "Data should have 12 elements");
            // Check that all elements are one
            for &value in data.iter() {
                assert_eq!(value, 1.0, "All elements should be one");
            }
        }
        
        if let Some(shape) = &collective.shape {
            assert_eq!(shape.get_n(), 12, "Shape should indicate 12 total elements");
            assert_eq!(shape.columns(), 3, "Shape should have 3 columns");
        }
    }
}

#[test]
fn test_zeros_multidimensional() {
    // Create a 3D structure: 2 -> 3 -> 5 (columns) = 2 * 3 * 5 = 30 elements
    let dim3 = Dimensions::new(0, 3).with_columns(5);
    let dim2 = Dimensions::new(0, 2).with_next(Rc::new(RefCell::new(dim3)));
    let dim1 = Dimensions::new(0, 2).with_next(Rc::new(RefCell::new(dim2)));

    let result: Option<Collective<f64>> = Numrs::zeros::<f64>(dim1);
    assert!(result.is_some(), "Numrs::zeros should return Some for valid multidimensional structure");

    if let Some(collective) = result {
        assert!(collective.data.is_some(), "Data should be allocated");
        assert!(collective.shape.is_some(), "Shape should be set");
        
        if let Some(data) = &collective.data {
            let expected_size = 2 * 2 * 3 * 5; // Based on your dimension chain
            assert_eq!(data.len(), expected_size, "Data should have {} elements", expected_size);
            
            // Check that all elements are zero
            for &value in data.iter() {
                assert_eq!(value, 0.0, "All elements should be zero");
            }
        }
        
        if let Some(shape) = &collective.shape {
            assert!(shape.is_valid(), "Shape should be valid");
            assert_eq!(shape.columns(), 5, "Shape should have 5 columns");
        }
    }
}

#[test]
fn test_ones_multidimensional() {
    // Create a 3D structure: 2 -> 3 -> 4 (columns) = 2 * 3 * 4 = 24 elements
    let dim3 = Dimensions::new(0, 3).with_columns(4);
    let dim2 = Dimensions::new(0, 2).with_next(Rc::new(RefCell::new(dim3)));
    let dim1 = Dimensions::new(0, 2).with_next(Rc::new(RefCell::new(dim2)));

    let result: Option<Collective<f64>> = Numrs::ones::<f64>(dim1);
    assert!(result.is_some(), "Numrs::ones should return Some for valid multidimensional structure");

    if let Some(collective) = result {
        assert!(collective.data.is_some(), "Data should be allocated");
        assert!(collective.shape.is_some(), "Shape should be set");
        
        if let Some(data) = &collective.data {
            let expected_size = 2 * 2 * 3 * 4; // Based on your dimension chain
            assert_eq!(data.len(), expected_size, "Data should have {} elements", expected_size);
            
            // Check that all elements are one
            for &value in data.iter() {
                assert_eq!(value, 1.0, "All elements should be one");
            }
        }
    }
}

#[test]
fn test_zeros_with_integer_type() {
    let dim = Dimensions::new(2, 3); // 2 columns, 3 rows = 6 elements
    
    let result: Option<Collective<i32>> = Numrs::zeros::<i32>(dim);
    assert!(result.is_some(), "Numrs::zeros should work with i32");
    
    if let Some(collective) = result {
        if let Some(data) = &collective.data {
            assert_eq!(data.len(), 6, "Data should have 6 elements");
            for &value in data.iter() {
                assert_eq!(value, 0, "All i32 elements should be zero");
            }
        }
    }
}

#[test]
fn test_ones_with_integer_type() {
    let dim = Dimensions::new(3, 2); // 3 columns, 2 rows = 6 elements
    
    let result: Option<Collective<i32>> = Numrs::ones::<i32>(dim);
    assert!(result.is_some(), "Numrs::ones should work with i32");
    
    if let Some(collective) = result {
        if let Some(data) = &collective.data {
            assert_eq!(data.len(), 6, "Data should have 6 elements");
            for &value in data.iter() {
                assert_eq!(value, 1, "All i32 elements should be one");
            }
        }
    }
}

#[test]
fn test_zeros_empty_dimensions() {
    let dim = Dimensions::new(0, 0); // Invalid dimensions with 0 size
    
    let result: Option<Collective<f64>> = Numrs::zeros::<f64>(dim);
    assert!(result.is_some(), "Numrs::zeros should return Some even for empty dimensions");
    
    if let Some(collective) = result {
        assert!(collective.data.is_none(), "Data should be None for empty dimensions");
        assert!(collective.shape.is_some(), "Shape should still be set");
        
        if let Some(shape) = &collective.shape {
            assert_eq!(shape.get_n(), 0, "Empty shape should have 0 total elements");
        }
    }
}

#[test]
fn test_ones_empty_dimensions() {
    let dim = Dimensions::new(0, 0); // Invalid dimensions with 0 size
    
    let result: Option<Collective<f64>> = Numrs::ones::<f64>(dim);
    assert!(result.is_some(), "Numrs::ones should return Some even for empty dimensions");
    
    if let Some(collective) = result {
        assert!(collective.data.is_none(), "Data should be None for empty dimensions");
        assert!(collective.shape.is_some(), "Shape should still be set");
        
        if let Some(shape) = &collective.shape {
            assert_eq!(shape.get_n(), 0, "Empty shape should have 0 total elements");
        }
    }
}

#[test]
fn test_zeros_large_dimensions() {
    let dim = Dimensions::new(100, 50); // 100 columns, 50 rows = 5000 elements
    
    let result: Option<Collective<f64>> = Numrs::zeros::<f64>(dim);
    assert!(result.is_some(), "Numrs::zeros should work with large dimensions");
    
    if let Some(collective) = result {
        if let Some(data) = &collective.data {
            assert_eq!(data.len(), 5000, "Large array should have 5000 elements");
            // Spot check some elements
            assert_eq!(data[0], 0.0, "First element should be zero");
            assert_eq!(data[2500], 0.0, "Middle element should be zero");
            assert_eq!(data[4999], 0.0, "Last element should be zero");
        }
    }
}

#[test]
fn test_ones_large_dimensions() {
    let dim = Dimensions::new(80, 25); // 80 columns, 25 rows = 2000 elements
    
    let result: Option<Collective<f64>> = Numrs::ones::<f64>(dim);
    assert!(result.is_some(), "Numrs::ones should work with large dimensions");
    
    if let Some(collective) = result {
        if let Some(data) = &collective.data {
            assert_eq!(data.len(), 2000, "Large array should have 2000 elements");
            // Spot check some elements
            assert_eq!(data[0], 1.0, "First element should be one");
            assert_eq!(data[1000], 1.0, "Middle element should be one");
            assert_eq!(data[1999], 1.0, "Last element should be one");
        }
    }
}

#[test]
fn test_consistency_between_zeros_and_ones() {
    let dim = Dimensions::new(4, 5); // Same dimensions for both
    
    let zeros_result: Option<Collective<f64>> = Numrs::zeros::<f64>(dim.clone());
    let ones_result: Option<Collective<f64>> = Numrs::ones::<f64>(dim);
    
    assert!(zeros_result.is_some() && ones_result.is_some(), "Both should succeed");
    
    if let (Some(zeros_collective), Some(ones_collective)) = (zeros_result, ones_result) {
        // Both should have the same structure
        assert_eq!(
            zeros_collective.data.as_ref().map(|d| d.len()),
            ones_collective.data.as_ref().map(|d| d.len()),
            "Both should have same data length"
        );
        
        if let (Some(zeros_shape), Some(ones_shape)) = (&zeros_collective.shape, &ones_collective.shape) {
            assert_eq!(zeros_shape.get_n(), ones_shape.get_n(), "Both should have same total elements");
            assert_eq!(zeros_shape.columns(), ones_shape.columns(), "Both should have same columns");
        }
    }
}

// Performance and stress tests
#[test]
fn test_memory_efficiency() {
    // Test that we're not wasting memory
    let dim = Dimensions::new(10, 10); // 100 elements
    
    let result: Option<Collective<f64>> = Numrs::zeros::<f64>(dim);
    
    if let Some(collective) = result {
        if let Some(data) = &collective.data {
            // Ensure we allocated exactly the right amount
            assert_eq!(data.len(), 100, "Should allocate exactly 100 elements");
            
            // Box<[T]> should be properly sized
            let box_size = std::mem::size_of_val(&**data);
            let expected_size = 100 * std::mem::size_of::<f64>();
            assert_eq!(box_size, expected_size, "Memory allocation should be efficient");
        }
    }
}

#[test]
fn test_randn() {
    let dim = Dimensions::new(10, 10); // 100 elements
    
    let result: Option<Collective<f64>> = Numrs::randn::<f64>(dim);
    
    if let Some(collective) = result {
        if let Some(data) = &collective.data {
            // Ensure we allocated exactly the right amount
            assert_eq!(data.len(), 100, "Should allocate exactly 100 elements");
            
            // Box<[T]> should be properly sized
            let box_size = std::mem::size_of_val(&**data);
            let expected_size = 100 * std::mem::size_of::<f64>();
            assert_eq!(box_size, expected_size, "Memory allocation should be efficient");
        }
    }
}
