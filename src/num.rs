/*
 * Numrs/src/num.rs
 * Q@khaa.pk
 */

use rand::Rng;
use rand::distributions::{Distribution, Standard}; 
use super::{dimensions::Dimensions, collective::Collective};

pub struct Tensor;

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

impl Tensor {

    /// Creates a new `Collective<E>` filled with ones, based on the shape provided by `Dimensions`.
    /// 
    /// # Arguments
    ///
    /// * `like` - A `Dimensions` object specifying the desired shape of the one-filled array.
    ///
    /// # Returns
    ///
    /// * `Option<Collective<E>)` with all elements set to `E::default() or if some other trait is defined`, or with no data if the shape is zero.
    /*pub fn ones<E: One + Copy>(like: Dimensions) -> Option<Collective<E>> {
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
    }*/

    pub fn ones<E: One + Copy>(like: Dimensions) -> Collective<E> {
        let n = like.get_n();
        if n == 0 {
            // Return an empty shape if the size is zero
            return Collective { 
                data: None,
                shape: Some(Box::new(Dimensions::new(0, 0))) // Wrap in Box::new()
            };
        }
            
        let allocation: Box<[E]> = vec![E::one(); n].into_boxed_slice();
    
        Collective { 
            data: Some(allocation),
            shape: Some(Box::new(like)), // Wrap in Box::new()
        }
    }

   /*
      The function randn creates a new `Collective<E>` filled with random numbers, 
      based on the shape provided by `Dimensions`.
    
      # Arguments
       * `like` - A `Dimensions` object specifying the desired shape of the random-filled array.
    
      # Returns
       * `Option<Collective<E>>` with all elements set to random values of type E, 
         or with no data if the shape is zero.
    
      # Note
       * For floating point types (f32, f64), generates uniformly distributed values in [0, 1)
       * For integer types, generates values across the full range of the type
    */
    /*pub fn randn<E>(like: Dimensions) -> Option<Collective<E>> 
    where 
        E: Copy,
        Standard: Distribution<E>
    {
        let n = like.get_n();
        if n == 0 {
            
            // Return an empty shape if the size is zero
            return Some(Collective {
                data: None,
                shape: Some(Box::new(Dimensions::new(0, 0)))
            });
        }
        
        let mut rng = rand::thread_rng();
        let allocation: Box<[E]> = (0..n)
            .map(|_| rng.gen::<E>())
            .collect::<Vec<E>>()
            .into_boxed_slice();

        Some(Collective {
            data: Some(allocation),
            shape: Some(Box::new(like)),
        })
    }*/

    pub fn randn<E>(like: Dimensions) -> Collective<E> 
    where 
        E: Copy,
        Standard: Distribution<E>
    {
        let n = like.get_n();
        if n == 0 {
            
            // Return an empty shape if the size is zero
            return Collective {
                data: None,
                shape: Some(Box::new(Dimensions::new(0, 0)))
            };
        }
        
        let mut rng = rand::thread_rng();
        let allocation: Box<[E]> = (0..n)
            .map(|_| rng.gen::<E>())
            .collect::<Vec<E>>()
            .into_boxed_slice();

        Collective {
            data: Some(allocation),
            shape: Some(Box::new(like)),
        }
    }

    
    /*
     * Generates a tensor of random integers within a specified range, shaped according to the provided dimensions.
     * 
     * This function creates a `Collective<i32>` containing random integers in the range `[low, high)` (inclusive of `low`, 
     * exclusive of `high`), with the tensor's shape defined by the `like` parameter (a `Dimensions` struct). It is designed to 
     * generate target sequences, such as `y_train` or `y_val` for the Hierarchical Reasoning Model (HRM), where the output is a 
     * 2D tensor of shape `[batch_size, seq_len]` containing class labels for a sequence classification task. The function uses 
     * the `rand` crate's thread-local random number generator (`thread_rng`) to produce uniformly distributed integers.
     * 
     * If the total size of the tensor (`n`, computed from `like.get_n()`) is zero, the function returns an empty `Collective` 
     * with a shape of `[0, 0]`. Otherwise, it allocates a boxed slice of `i32` values and wraps it in a `Collective` with the 
     * specified shape.
     * 
     * Arguments:
     * - `low`: The inclusive lower bound of the random integer range (i32).
     * - `high`: The exclusive upper bound of the random integer range (i32).
     * - `like`: A `Dimensions` struct defining the shape of the output tensor (e.g., `[batch_size, seq_len]` for HRM target 
     *           sequences). The total number of elements (`n`) is computed via `like.get_n()`.
     * 
     * Returns:
     * - `Some(Collective<i32>)`: A `Collective` containing a tensor of random integers with the specified shape. If `n == 0`, 
     *                            returns an empty `Collective` with shape `[0, 0]`.
     * 
     * Notes:
     * - The function assumes `like.get_n()` correctly computes the total number of elements based on the `Dimensions` struct.
     * - The output tensor is stored as a `Box<[i32]>` for memory efficiency, suitable for large tensors.
     * - This function is typically used to generate synthetic target sequences (e.g., class labels) for training or validating 
     *   neural networks like the HRM.
     * - Ensure `low < high` to avoid invalid range errors in `rng.gen_range`.
     */
    /*pub fn randint (low: i32, high: i32, like: Dimensions) -> Option<Collective<i32>> {

        let n = like.get_n();
        
        if n == 0 {
            
            // Return an empty shape if the size is zero
            return Some(Collective {
                data: None,
                shape: Some(Box::new(Dimensions::new(0, 0)))
            });
        }

        let mut rng = rand::thread_rng();
        let allocation: Box<[i32]> = (0..n)
            .map(|_| rng.gen_range(low..high))
            .collect::<Vec<i32>>()
            .into_boxed_slice();

        Some(Collective {
            data: Some(allocation),
            shape: Some(Box::new(like)),
        })        
    }*/

    pub fn randint (low: i32, high: i32, like: Dimensions) -> Collective<i32> {

        let n = like.get_n();
        
        if n == 0 {
            
            // Return an empty shape if the size is zero
            return Collective {
                data: None,
                shape: Some(Box::new(Dimensions::new(0, 0)))
            };
        }

        let mut rng = rand::thread_rng();
        let allocation: Box<[i32]> = (0..n)
            .map(|_| rng.gen_range(low..high))
            .collect::<Vec<i32>>()
            .into_boxed_slice();

        Collective {
            data: Some(allocation),
            shape: Some(Box::new(like)),
        }        
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

    /*
     * Creates a tensor filled with zeros, shaped according to the provided dimensions.
     * 
     * This function generates a `Collective<E>` containing a tensor of zeros with the shape specified by the `like` parameter
     * (a `Dimensions` struct). It is designed to initialize tensors, such as hidden states or biases, for neural network models
     * like the Hierarchical Reasoning Model (HRM), which requires zero-initialized tensors for components like the High-Level
     * Module (HLM) and Low-Level Module (LLM) hidden states (`h_t` and `l_t`). The tensor is stored as a `Box<[E]>` for memory
     * efficiency, with the total number of elements (`n`) computed via `like.get_n()`.
     * 
     * If the total size of the tensor (`n`) is zero, the function returns an empty `Collective` with a shape of `[0, 0]`.
     * Otherwise, it allocates a boxed slice filled with zeros using the `Zero` trait's `zero()` method for the element type `E`.
     * 
     * Arguments:
     * - `like`: A `Dimensions` struct defining the shape of the output tensor (e.g., `[batch_size, d_h]` for HLM hidden states
     *           or `[batch_size, d_l]` for LLM hidden states in HRM). The total number of elements (`n`) is computed via
     *           `like.get_n()`.
     * 
     * Type Constraints:
     * - `E: Zero + Copy`: The element type (e.g., `f32`, `f64`) must implement the `Zero` trait to provide a zero value and
     *                      `Copy` to allow efficient copying of elements.
     * 
     * Returns:
     * - `Some(Collective<E>)`: A `Collective` containing a tensor of zeros with the specified shape. If `n == 0`, returns an
     *                          empty `Collective` with shape `[0, 0]`.
     * 
     * Notes:
     * - The function assumes `like.get_n()` correctly computes the total number of elements based on the `Dimensions` struct.
     * - The output tensor is stored as a `Box<[E]>` for efficient memory management, suitable for large tensors.
     * - This function is typically used to initialize zero-filled tensors, such as initial hidden states (`h_t`, `l_t`) or
     *   biases in the HRM's HLM and LLM modules.
     * - Ensure the `Dimensions` struct is properly configured to match the expected tensor shape for the target use case.
     */
    /*pub fn zeros<E: Zero + Copy>(like: Dimensions) -> Option<Collective<E>> {
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
    }*/

    pub fn zeros<E: Zero + Copy>(like: Dimensions) -> Collective<E> {
        let n = like.get_n();
        if n == 0 {
            // Return an empty shape if the size is zero
            return Collective { 
                data: None,
                shape: Some(Box::new(Dimensions::new(0, 0))) // Wrap in Box::new()
            };
        }
    
        let allocation: Box<[E]> = vec![E::zero(); n].into_boxed_slice();
    
        Collective { 
            data: Some(allocation),
            shape: Some(Box::new(like)), // Wrap in Box::new()
        }
    }
}
