/*
   Numrs/src/lib.rs
   Q@khaa.pk
*/

#![feature(f16)] // The type `f16` is unstable. To enable this feature, we need to enable the f16 feature at the root level of the crate and install nightely build of rustc

pub mod collective;
pub mod dimensions;
pub mod header;
pub mod num;

// Re-export main types at crate level for easier importing
pub use dimensions::Dimensions;
pub use header::Axis;

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
