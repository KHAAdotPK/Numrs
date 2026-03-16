/*
 * Numrs/src/Dimensions.rs
 * Q@khaa.pk
 */

use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Clone)]
// Derive Clone trait for easy cloning.
// Rust has the Clone trait, which is used to explicitly copy the data of a struct (or any value) — somewhat like a copy constructor in C++. But it’s opt-in and not automatic unless you derive it.
// This is not automatic because Rust values are moved by default, not copied
#[derive(Debug)] // Derive Debug trait for easy debugging. In Rust, if you want to print something with {:?} in println!, the type must implement the Debug trait.
                 // If you're debugging and want clearer output, you can use {:#?} instead of {:?} for a pretty-printed, indented layout

pub struct Dimensions {
    columns: f64,
    rows: f64,
    next: Option<Rc<RefCell<Dimensions>>>, // This field does not implement `Copy` trait. Copy doesn’t work for anything with heap-allocated parts like String, Vec, etc. Use Clone for those
    prev: Option<Rc<RefCell<Dimensions>>>, // This field does not implement `Copy` trait. Copy doesn’t work for anything with heap-allocated parts like String, Vec, etc. Use Clone for those
}

impl Dimensions {
    // Constructor
    pub fn new(columns: f64, rows: f64) -> Self {
        Self {
            columns,
            rows,
            next: None,
            prev: None,
        }
    }

    // Fluent setters
    pub fn with_columns(mut self, columns: f64) -> Self {
        self.columns = columns;
        self
    }

    pub fn with_rows(mut self, rows: f64) -> Self {
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
    pub fn set_columns(&mut self, columns: f64) {
        self.columns = columns;
    }

    pub fn set_rows(&mut self, rows: f64) {
        self.rows = rows;
    }

    pub fn set_next(&mut self, next: Option<Rc<RefCell<Dimensions>>>) {
        self.next = next;
    }

    pub fn set_prev(&mut self, prev: Option<Rc<RefCell<Dimensions>>>) {
        self.prev = prev;
    }

    // Getters

    pub fn get_columns(&self) -> f64 {
        self.columns()
    }

    pub fn get_rows(&self) -> f64 {
        self.rows()
    }

    pub fn get_width(&self) -> f64 {
        self.columns()
    }

    pub fn get_height(&self) -> f64 {
        self.rows()
    }

    pub fn get_next(&mut self) -> Option<&mut Rc<RefCell<Dimensions>>> {
        self.next.as_mut()
    }

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
    /// * `f64` — number of columns in the last node.
    ///
    /// # Note
    /// - If the linked list is a single node (no `next`), this returns its `columns`.
    /// - If the list is empty (hypothetically), it would return `0`.
    pub fn columns(&self) -> f64 {
        let mut current_opt = Some(Rc::new(RefCell::new(self.clone())));
        let mut n: f64 = 0.0;

        while let Some(current_rc) = current_opt {
            let current = current_rc.borrow();

            n = current.columns as f64;

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
    pub fn rows(&self) -> f64 {
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
    /// /*let total = dimensions.getN();*/
    /// /*assert!(total > 0); // if the structure is valid*/
    /// ```
    ///
    /// # Returns
    /// * `usize` - Total number of elements (or `0` if invalid).    
    pub fn get_n(&self) -> usize {
        let mut n: f64 = 1.0;

        let mut current_opt = Some(Rc::new(RefCell::new(self.clone())));
        let mut last_link_columns: f64 = 0.0;

        while let Some(current_rc) = current_opt {
            let current = current_rc.borrow();

            n *= current.rows;
            last_link_columns = current.columns;

            current_opt = current.next.clone();
        }

        n *= last_link_columns;

        return (n as usize);
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
    /// * `f64` — Total number of inner arrays.
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
    pub fn get_number_of_inner_arrays(&self) -> f64 {
        let mut n: f64 = 1.0;
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
    pub fn get_number_of_innermost_arrays(&self) -> f64 {
        let mut n: f64 = 0.0;
        let mut current_opt = Some(Rc::new(RefCell::new(self.clone())));

        while let Some(current_rc) = current_opt {
            let current = current_rc.borrow();

            if current.next.is_none() && current.columns != 0.0 {
                n = current.rows;
            }

            current_opt = current.next.clone(); // move to next node
        }

        n
    }

    /// Checks whether the linked `Dimensions` chain is valid.
    ///
    /// Validation rules:
    /// - `rows` must never be zero at any node.
    /// - If `next` exists (i.e., not the last node), then `columns` must be **zero**.
    /// - If `next` is `None` (i.e., the last node), then `columns` must be **non-zero**.
    ///
    /// Walks through the entire linked list starting from `self`, using a loop and match.
    ///
    /// Returns:
    /// - `true` if all nodes follow the rules.
    /// - `false` if any node violates the rules.
    ///
    /// # Logic Summary
    /// - If a node's `rows == 0` → Invalid
    /// - If `next == Some(...)` and `columns != 0` → Invalid
    /// - If `next == None` and `columns == 0` → Invalid
    /// - Otherwise → Valid
    ///
    /// # Example
    /// ```rust
    /// /*let dim3 = Dimensions::new(0, 0).with_columns(5).with_rows(10);
    /// let dim2 = Dimensions::new(0, 0).with_columns(0).with_rows(10).with_next(Rc::new(RefCell::new(dim3)));
    /// let dim1 = Dimensions::new(0, 0).with_columns(0).with_rows(10).with_next(Rc::new(RefCell::new(dim2)));*/
    ///
    /// /*assert!(dim1.is_valid());*/
    /// ```        
    pub fn is_valid(&self) -> bool {
        let mut current_opt = Some(Rc::new(RefCell::new(self.clone())));

        loop {
            match current_opt {
                None => break,
                Some(current_rc) => {
                    let current = current_rc.borrow();

                    if current.rows == 0.0 {
                        return false;
                    }

                    if current.next.is_none() {
                        if current.columns == 0.0 {
                            return false;
                        }
                    } else {
                        if current.columns != 0.0 {
                            return false;
                        }
                    }

                    current_opt = current.next.clone();
                }
            }
        }

        true
    }

    /*
       @param vec: Vec<f64> - Vector of dimensions
       @panic - If vector has less than 2 elements
       @panic - If vector contains 0

       @return Dimensions - Linked list of dimensions

       @example
       ```rust
       let vec = vec![1, 2, 3];
       let dimensions = Dimensions::from_vec(vec);
       assert_eq!(dimensions.get_rows(), 1);
       assert_eq!(dimensions.get_columns(), 2);
       assert_eq!(dimensions.get_next().unwrap().borrow().get_rows(), 2);
       assert_eq!(dimensions.get_next().unwrap().borrow().get_columns(), 3);
       ```
    */
    pub fn from_vec(vec: Vec<f64>) -> Dimensions {
        if vec.len() < 2 {
            panic!("Dimensions::from_vec(_: Vec<f64>) Error: Vector must have at least 2 elements");
        }

        for &val in vec.iter() {
            if val == 0.0 {
                panic!("Dimensions::from_vec(_: Vec<f64>) Error: Vector must not contain 0");
            }
        }

        // Special case: exactly two dimensions -> single node
        if vec.len() == 2 {
            return Dimensions::new(vec[1], vec[0]); // columns = vec[1], rows = vec[0]
        }

        // More than two dimensions: build a linked list
        let head = Rc::new(RefCell::new(Dimensions::new(0.0, vec[0]))); // first node: rows = vec[0], columns = 0
        let mut prev = head.clone();

        // Create intermediate nodes (indices 1 .. len-2)
        for i in 1..vec.len() - 2 {
            let curr = Rc::new(RefCell::new(Dimensions::new(0.0, vec[i]))); // columns = 0.0, rows = vec[i]
            prev.borrow_mut().set_next(Some(curr.clone()));
            curr.borrow_mut().set_prev(Some(prev.clone()));
            prev = curr;
        }

        // Create the final node: rows = vec[len-2], columns = vec[len-1]
        let last = Rc::new(RefCell::new(Dimensions::new(
            vec[vec.len() - 1],
            vec[vec.len() - 2],
        )));

        prev.borrow_mut().set_next(Some(last.clone())); // Connect the second last node to the last node
        last.borrow_mut().set_prev(Some(prev.clone())); // Connect the last node to the previous node

        // Return a clone of the head (shares ownership via Rc)
        let x = head.borrow().clone();

        x // Pass by value
    }

    /// Converts the linked list of nodes starting from `self` into a vector of floating-point values.
    ///
    /// The traversal begins at the current node and follows the `next` references until the end of the list.
    /// For each node encountered, its `rows` field is appended to the result vector. If the node's `columns`
    /// field is greater than `0.0`, it is also appended immediately after the corresponding `rows` value.
    ///
    /// This operation clones the starting node to avoid mutating the original structure; the rest of the list
    /// is accessed through shared references (via `Rc` and `RefCell`), so no deep copy of the entire list occurs.
    ///
    /// # Returns
    /// A `Vec<f64>` containing the collected values in the order they appear during traversal.
    pub fn to_vec(&self) -> Vec<f64> {
        let mut current_opt = Some(Rc::new(RefCell::new(self.clone())));

        let mut ret = Vec::new(); // Zero length vector

        while let Some(current_rc) = current_opt
        /* While the current node is not None */
        {
            let current = current_rc.borrow(); // Borrow the current node

            ret.push(current.rows); // Push the rows

            if current.columns > 0.0 {
                ret.push(current.columns); // Push the columns
            }

            current_opt = current.next.clone(); // move to next node
        }

        ret // Pass by value
    }
}

/// Formats the `Dimensions` linked list for display using the `Display` trait.
///
/// This method traverses the entire linked list of `Dimensions` nodes and creates
/// a human-readable string representation showing the dimensional structure.
///
/// # Format Rules:
/// - **Intermediate nodes** (where `next.is_some()`): Shows only the `rows` value
/// - **Final node** (where `next.is_none()` and `columns > 0`): Shows `rows × columns`
/// - **Empty or invalid structures**: Shows "Empty dimensions"
/// - **Separator**: Uses `→` (arrow) to indicate dimensional flow from outer to inner
///
/// # How it works:
/// 1. Creates a vector to collect dimension strings
/// 2. Traverses the linked list using `next` pointers
/// 3. For each node, determines the appropriate format based on its position
/// 4. Joins all collected parts with `→` arrows
/// 5. Handles edge cases for empty or malformed structures
///
/// # Output Examples:
/// - Single dimension: `"10 × 5"` → 10 rows with 5 columns each
/// - Two dimensions: `"3 → 10 × 5"` → 3 arrays, each with 10 rows of 5 columns
/// - Three dimensions: `"2 → 3 → 10 × 5"` → 2 groups of 3 arrays, each 10×5
/// - Invalid/empty: `"Empty dimensions"`
///
/// # Parameters:
/// * `&self` - Reference to the starting `Dimensions` node
/// * `f` - Mutable reference to the formatter for writing output
///
/// # Returns:
/// * `fmt::Result` - `Ok(())` on successful formatting, or formatting error
///
/// # Usage:
/// ```rust
/// let dims = Dimensions::new(10, 5);
/// println!("{}", dims); // Prints: "10 × 5"
///
/// let complex_dims = Dimensions::new(0, 3)
///     .with_next(Rc::new(RefCell::new(
///         Dimensions::new(10, 5)
///     )));
/// println!("{}", complex_dims); // Prints: "3 → 10 × 5"
/// ```
///
/// # Note:
/// This implementation assumes that the linked structure follows the validation rules
/// defined in `is_valid()`. Malformed structures may produce unexpected output.
impl fmt::Display for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        let mut current_opt = Some(Rc::new(RefCell::new(self.clone())));

        // Traverse the linked list and collect dimension information
        while let Some(current_rc) = current_opt {
            let current = current_rc.borrow();

            // If this is the last node (has columns), show rows × columns
            if current.next.is_none() && current.columns > 0.0 {
                parts.push(format!("{} × {}", current.rows, current.columns));
            }
            // If this is not the last node, just show rows
            else if current.next.is_some() && current.rows > 0.0 {
                parts.push(format!("{}", current.rows));
            }

            current_opt = current.next.clone();
        }

        // Join all parts with " → " to show the dimensional flow
        if parts.is_empty() {
            write!(f, "Empty dimensions")
        } else {
            write!(f, "{}", parts.join(" → "))
        }
    }
}

/*
 (A personal design journey — only later realized similar ideas power major frameworks.)

 ChatGPT4o, April 2025...
 Final comment:
 You did a very nice design!
 This structure is very close to how even real frameworks (like PyTorch) internally represent shapes! 💪🏼
*/
