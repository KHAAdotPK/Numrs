/*
 * lib/Numrs/Dimensions.rs
 * Q@khaa.pk
 */

use std::rc::Rc;
//use std::cell::Ref;
use std::cell::RefCell;

#[derive(Clone)] // Derive Clone trait for easy cloning.
                 // Rust has the Clone trait, which is used to explicitly copy the data of a struct (or any value) — somewhat like a copy constructor in C++. But it’s opt-in and not automatic unless you derive it.
                 // This is not automatic because Rust values are moved by default, not copied
#[derive(Debug)] // Derive Debug trait for easy debugging. In Rust, if you want to print something with {:?} in println!, the type must implement the Debug trait.
                 // If you're debugging and want clearer output, you can use {:#?} instead of {:?} for a pretty-printed, indented layout

pub struct Dimensions {
    columns: usize,
    rows: usize,
    next: Option<Rc<RefCell<Dimensions>>>, // This field does not implement `Copy` trait. Copy doesn’t work for anything with heap-allocated parts like String, Vec, etc. Use Clone for those
    prev: Option<Rc<RefCell<Dimensions>>>, // This field does not implement `Copy` trait. Copy doesn’t work for anything with heap-allocated parts like String, Vec, etc. Use Clone for those
}
 
impl Dimensions {
    // Constructor
    pub fn new(columns: usize, rows: usize) -> Self {
        Self {
            columns,
            rows,
            next: None,
            prev: None,
        }
    }
 
    // Fluent setters
    pub fn with_columns(mut self, columns: usize) -> Self {
        self.columns = columns;
        self
    }
 
    pub fn with_rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }
 
    pub fn with_next(mut self, next: Rc<RefCell<Dimensions>>) -> Self {
        self.next = Some(next);
        self
    }
 
    pub fn with_prev(mut self, prev: Rc<RefCell<Dimensions>>) -> Self {
        self.prev = Some(prev);
        self
    }
 
    // Regular mutable setters (if needed after creation)
    pub fn set_columns(&mut self, columns: usize) {
        self.columns = columns;
    }

    pub fn set_rows(&mut self, rows: usize) {
        self.rows = rows;
    }
 
    pub fn set_next(&mut self, next: Option<Rc<RefCell<Dimensions>>>) {
        self.next = next;
    }
 
    pub fn set_prev(&mut self, prev: Option<Rc<RefCell<Dimensions>>>) {
        self.prev = prev;
    }
 
    // Getters  
    
    /// Returns the number of columns in the last link of the `Dimensions` linked list.
    ///
    /// In this linked structure:
    /// - Only the **last** link is expected to have a non-zero `columns` value.
    /// - All earlier links may have `columns == 0`.
    ///
    /// This method traverses the entire linked list,
    /// and returns the `columns` value from the last node.
    ///
    /// # How it works:
    /// - Starts from the current node.
    /// - Iteratively moves through the `next` links.
    /// - Updates `n` at each node.
    /// - After reaching the end, returns the last `columns` value found.
    ///
    /// # Returns
    /// * `usize` — number of columns in the last node.
    ///
    /// # Note
    /// - If the linked list is a single node (no `next`), this returns its `columns`.
    /// - If the list is empty (hypothetically), it would return `0`.
    pub fn columns(&self) -> usize {
               
        let mut current_opt = Some(Rc::new(RefCell::new(self.clone())));
        let mut n = 0;
    
        while let Some(current_rc) = current_opt {
            let current = current_rc.borrow();
               
            n = current.columns;
    
            current_opt = current.next.clone();
        }

        n
    }
    
    /// Calculates the total number of rows in the multidimensional array.
    ///
    /// This method combines two values:
    /// - The number of **inner arrays** (from `get_number_of_inner_arrays()`).
    /// - The number of **innermost arrays** (from `get_number_of_innermost_arrays()`).
    ///
    /// By multiplying these two values, it computes the total number of rows
    /// across all levels of the multidimensional structure.
    ///
    /// # How it works:
    /// - Calls `get_number_of_inner_arrays` to find how many groups of arrays there are.
    /// - Calls `get_number_of_innermost_arrays` to find how many arrays exist inside each group.
    /// - Multiplies the two values to find the total number of rows.
    ///
    /// # Returns
    /// * `usize` — Total number of rows in the multidimensional array.
    ///
    /// # Notes
    /// - If the structure has only one node (no `next`), this simplifies to the number of rows directly.
    /// - If the dimensions are invalid (e.g., zero rows somewhere), the result may be zero, which can act as a **sanity check**.
    ///
    /// # Example
    /// Suppose the dimensions are:
    /// ```text
    /// rows = 3 -> rows = 4 -> columns = 5
    /// ```
    /// Then:
    /// ```text
    /// get_number_of_inner_arrays() = 3
    /// get_number_of_innermost_arrays() = 4
    /// rows() = 3 * 4 = 12
    /// ```
    /// Meaning there are 12 rows total (each having 5 columns).
    pub fn rows(&self) -> usize {
        
        (self.get_number_of_inner_arrays() * self.get_number_of_innermost_arrays())
    }
 
    pub fn next(&self) -> Option<Rc<RefCell<Dimensions>>> {
        self.next.clone()
    }
 
    pub fn prev(&self) -> Option<Rc<RefCell<Dimensions>>> {
        self.prev.clone()
    }
    
    /// Returns the total number of places (indices) in the multidimensional array.
    ///
    /// Traverses the linked list of `Dimensions`, multiplying the `rows` at each link.
    /// Only the last link contributes its `columns` value; intermediate links must have `columns = 0`.
    ///
    /// If any `rows` value is zero, or the final `columns` is zero, the result will be zero,
    /// indicating an invalid structure.
    ///
    /// # Examples
    /// ```
    /// let total = dimensions.getN();
    /// assert!(total > 0); // if the structure is valid
    /// ```
    ///
    /// # Returns
    /// * `usize` - Total number of elements (or `0` if invalid).    
    pub fn get_n(&self) -> usize {
        let mut n: usize = 1;
    
        let mut current_opt = Some(Rc::new(RefCell::new(self.clone())));
        let mut last_link_columns = 0;
    
        while let Some(current_rc) = current_opt {
            let current = current_rc.borrow();
    
            n *= current.rows;
            last_link_columns = current.columns;
    
            current_opt = current.next.clone();
        }
    
        n *= last_link_columns;
    
        n
    }
    
    /// Calculates the number of "inner arrays" in a multidimensional array structure.
    ///
    /// This method interprets the linked `Dimensions` structure as a series of dimensions,
    /// where each `rows` value (except the last node) represents a level of inner arrays.
    ///
    /// # How it works:
    /// - Starts at the current node.
    /// - Traverses the linked list through the `next` pointers.
    /// - At each node **except the last one**, multiplies the running total `n` by the `rows` value.
    /// - Skips multiplying at the last link, because it represents the final scalar elements
    ///   (defined by `columns`), not further nested arrays.
    /// - Returns the final computed `n`, representing the total number of "inner arrays".
    ///
    /// # Returns
    /// * `usize` — Total number of inner arrays.
    ///
    /// # Notes
    /// - If the structure has only one node (no `next`), this will simply return `1`.
    /// - If any intermediate node has `rows = 0`, the total `n` will become zero.
    ///   This acts as a **sanity check** — a valid multidimensional array must have at least one place to store data.
    ///
    /// # Example
    /// Suppose the dimensions are:
    /// ```text
    /// rows = 3 -> rows = 4 -> columns = 5
    /// ```
    /// Then, `get_number_of_inner_arrays` will compute:
    /// ```text
    /// n = 1 * 3 = 3
    /// ```
    /// Meaning there are 3 inner arrays, each with 4 rows and 5 columns.
    pub fn get_number_of_inner_arrays(&self) -> usize {        
        let mut n = 1;
        let mut current_opt = Some(Rc::new(RefCell::new(self.clone())));
    
        while let Some(current_rc) = current_opt {
            let current = current_rc.borrow();
            
            if !current.next.is_none() {

                n *= current.rows;              
            }
                
            current_opt = current.next.clone(); // move to next node (clone the Rc)
        }
    
        n
    }
    
    /// Calculates the number of "innermost arrays" in a multidimensional array structure.
    ///
    /// This method traverses the linked `Dimensions` list and finds the number of arrays
    /// present at the **last level** — where scalar elements (`columns`) exist.
    /// 
    /// Specifically, it picks the `rows` value from the last node (where `next` is `None`
    /// and `columns` is non-zero). This represents how many separate "innermost" arrays
    /// exist before reaching individual elements.
    ///
    /// # How it works:
    /// - Starts at the current node.
    /// - Traverses the list via `next` pointers.
    /// - At the last node (where `next` is `None` and `columns != 0`), reads the `rows` value.
    /// - Returns that value as the number of innermost arrays.
    ///
    /// # Returns
    /// * `usize` — The number of innermost arrays.
    ///
    /// # Notes
    /// - If the structure has only one node, and `columns != 0`, it will return its `rows`.
    /// - If the structure is invalid (e.g., missing final `columns`), this may return `0`,
    ///   acting as a **sanity check**.
    /// - Zero `rows` at the last node means no innermost arrays exist.
    ///
    /// # Example
    /// Suppose the dimensions are:
    /// ```text
    /// rows = 2 -> rows = 3 -> columns = 5
    /// ```
    /// Then, `get_number_of_innermost_arrays` will pick the `rows = 3` from the second link:
    /// ```text
    /// n = 3
    /// ```
    /// Meaning there are 3 innermost arrays (each with 5 elements).
    pub fn get_number_of_innermost_arrays(&self) -> usize {
        let mut n = 0;
        let mut current_opt = Some(Rc::new(RefCell::new(self.clone())));
    
        while let Some(current_rc) = current_opt {
            let current = current_rc.borrow();
            
            if current.next.is_none() && current.columns != 0 {
                n = current.rows;
            }
   
            current_opt = current.next.clone(); // move to next node
        }
        
        n
    }

}
 
