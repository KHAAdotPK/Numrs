/*
    numrs/src/lib.rs
    Q@khaa.pk
 */

pub mod dimensions;
pub mod collective;
pub mod num;

// Re-export main types at crate level for easier importing
pub use dimensions::Dimensions;
//pub use collective::Collective; // Uncomment when you have this module


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let dim = Dimensions::new(5, 10);
        assert!(dim.is_valid());
    }
    
    // Add basic smoke tests for other modules when ready
    // #[test]
    // fn test_collective_basic() {
    //     // Test collective module when implemented
    // }
    
    // #[test] 
    // fn test_num_basic() {
    //     // Test num module when implemented
    // }
}

