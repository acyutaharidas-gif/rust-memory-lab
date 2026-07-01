//! Demonstrates the difference between stack and heap allocation in Rust,
//! and how Rust's ownership model decides which one you get.

/// A `Point` with two `i32` fields is `Copy` and small, so instances live
/// entirely on the stack. No allocator call, no pointer indirection.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

/// A `Matrix` wraps a `Vec<f64>`. The `Vec` header (ptr, len, cap) sits on
/// the stack, but the actual buffer of floats is heap-allocated so it can
/// grow, be moved cheaply, and outlive the function that created it.
struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn zeros(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![0.0; rows * cols],
            rows,
            cols,
        }
    }
}

pub fn run() {
    println!("--- Stack vs Heap ---");

    // Stack allocation: `p` is a fixed-size value copied by value on every
    // assignment. No heap traffic, no lifetime bookkeeping needed.
    let p = Point { x: 3, y: 4 };
    let p2 = p; // Copy, not move -- both `p` and `p2` remain valid.
    println!("stack Point: p={:?} p2={:?} (both valid, Copy semantics)", p, p2);
    println!(
        "  size_of::<Point>() = {} bytes, lives entirely on the stack frame",
        std::mem::size_of::<Point>()
    );

    // Heap allocation: the Vec's backing buffer is allocated with the
    // global allocator. `mat` on the stack is just a fat pointer + metadata.
    let mat = Matrix::zeros(3, 3);
    println!(
        "heap Matrix: {}x{} buffer allocated on the heap, stack only holds ptr/len/cap",
        mat.rows, mat.cols
    );
    println!(
        "  size_of::<Vec<f64>>() header = {} bytes regardless of buffer size",
        std::mem::size_of::<Vec<f64>>()
    );

    // Ownership + move semantics: heap-backed types move instead of copy.
    let mat2 = mat; // `mat` is moved into `mat2`; `mat` is no longer usable.
    println!(
        "moved Matrix now owned by mat2 ({} elements); original binding `mat` is invalid",
        mat2.data.len()
    );

    // Uncommenting the next line is a compile error, not a runtime bug:
    // println!("{}", mat.rows); // error[E0382]: borrow of moved value: `mat`
    println!("(attempting to use `mat` again here would fail to *compile*, not crash at runtime)");
}
