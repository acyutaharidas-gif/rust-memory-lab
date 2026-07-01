//! Demonstrates how Rust's ownership + borrowing rules prevent whole classes
//! of memory errors *at compile time*, with the errors that would occur
//! shown (but not compiled) for reference.

struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    fn new(size: usize) -> Self {
        Buffer { data: vec![0u8; size] }
    }

    fn write_byte(&mut self, idx: usize, val: u8) {
        self.data[idx] = val; // bounds-checked; panics safely instead of corrupting memory
    }
}

/// Borrow checker prevents use-after-free by construction: a reference can
/// never outlive the value it points to. The classic C bug --
///   char *p = malloc(...); free(p); *p = 'x'; // use-after-free
/// has no Rust equivalent in safe code, because `p` would be *consumed*
/// (moved) or the borrow would be rejected before you could dangle it.
fn ownership_prevents_use_after_free() {
    let owner_scope_result = {
        let buf = Buffer::new(16);
        // `reference` cannot escape this block outliving `buf`.
        let reference = &buf.data;
        reference.len()
    }; // `buf` is dropped (freed) exactly here.
    println!(
        "buffer created, read, and freed safely; length was {}",
        owner_scope_result
    );

    // The following would NOT compile if uncommented -- this is the point:
    // let dangling: &Vec<u8>;
    // {
    //     let buf = Buffer::new(16);
    //     dangling = &buf.data; // error[E0597]: `buf` does not live long enough
    // }
    // println!("{:?}", dangling); // would-be use-after-free, caught by rustc
    println!("(a dangling reference like the above fails to compile: E0597)");
}

/// The borrow checker also prevents data races / iterator invalidation bugs
/// that in C/C++ often manifest as use-after-free or double-free once you
/// add concurrency or resizing into the mix.
fn ownership_prevents_aliased_mutation() {
    let mut buf = Buffer::new(4);
    buf.write_byte(0, 0xFF);

    // let r1 = &buf.data;
    // buf.write_byte(1, 0xAA); // error[E0502]: cannot borrow `buf` as mutable
    //                          // because it is also borrowed as immutable
    // println!("{:?}", r1);
    println!("(mutating while an immutable borrow is live fails to compile: E0502)");
    println!("final buffer state: {:?}", buf.data);
}

pub fn run() {
    println!("--- Ownership-Based Memory Safety ---");
    ownership_prevents_use_after_free();
    ownership_prevents_aliased_mutation();
}
