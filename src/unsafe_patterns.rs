//! Demonstrates the memory error classes that `unsafe` Rust reopens once you
//! step outside the borrow checker's guarantees: use-after-free and buffer
//! overreads. These are the same primitive bug classes (CWE-416 and
//! CWE-125) that underlie a large fraction of real-world memory-corruption
//! exploits in C/C++ systems software.
//!
//! IMPORTANT: every function here is *intentionally* undefined behavior
//! (UB) for demonstration purposes only. UB means the compiler is allowed
//! to assume it never happens -- output here is illustrative, not a
//! guarantee, and should never be imitated outside a teaching context.

use std::alloc::{alloc, dealloc, Layout};

/// Classic use-after-free: allocate memory, free it, then dereference the
/// now-dangling pointer. In safe Rust this is impossible because ownership
/// tracking forces the compiler to reject it (see ownership.rs). `unsafe`
/// lets us bypass that check entirely -- which is exactly why `unsafe`
/// blocks are the audit boundary in real Rust codebases.
pub fn use_after_free_demo() {
    println!("[unsafe] use-after-free (CWE-416):");

    let layout = Layout::new::<i32>();
    unsafe {
        let ptr = alloc(layout) as *mut i32;
        if ptr.is_null() {
            println!("  allocation failed");
            return;
        }
        *ptr = 42;
        println!("  wrote 42 through raw pointer {:p}", ptr);

        dealloc(ptr as *mut u8, layout); // memory returned to the allocator
        println!("  freed the allocation -- `ptr` is now dangling");

        // UB: reading through a dangling pointer. The allocator may have
        // already handed this address to someone else, reused it for
        // internal bookkeeping, or left the bytes untouched -- the value
        // read below is not guaranteed to be 42, or even stable across runs.
        let stale_value = *ptr;
        println!(
            "  read {} through freed pointer -- looks fine, but is UB and unreliable",
            stale_value
        );
        println!("  (this is the exact primitive attackers exploit for UAF-based RCE:");
        println!("   reallocate the freed slot with attacker-controlled data, then");
        println!("   trigger the stale read/write to hijack a vtable or pointer.)");
    }
}

/// Buffer overread: construct a slice whose length lies about how many
/// valid elements exist, then read past the true end of the allocation.
/// Safe Rust's `Index` impls are bounds-checked and panic instead; `unsafe`
/// slice construction skips that check entirely.
pub fn buffer_overread_demo() {
    println!("[unsafe] buffer overread (CWE-125):");

    let valid_data: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF];
    let true_len = valid_data.len();
    let overread_len = true_len + 8; // lie about the length

    println!(
        "  allocated {} valid bytes: {:02X?}",
        true_len, valid_data
    );

    unsafe {
        // std::slice::from_raw_parts trusts the caller completely -- it does
        // no bounds checking against the real allocation size.
        let overread_slice =
            std::slice::from_raw_parts(valid_data.as_ptr(), overread_len);
        println!(
            "  read {} bytes claiming length {}: {:02X?}",
            overread_len, overread_len, overread_slice
        );
        println!(
            "  bytes past index {} are adjacent heap memory, not part of our buffer --",
            true_len - 1
        );
        println!("  this is how heap overreads leak adjacent secrets (keys, pointers,");
        println!("  canaries) in bugs like Heartbleed-style information disclosure.");
    }
}

pub fn run() {
    println!("--- Unsafe Rust: Memory Error Classes ---");
    use_after_free_demo();
    println!();
    buffer_overread_demo();
    println!();
    println!("Note: both demos above compile only inside `unsafe` blocks.");
    println!("Rust does not eliminate these bug classes -- it fences them behind");
    println!("`unsafe`, making every possible UAF/overread grep-able and auditable,");
    println!("instead of reachable from ordinary code anywhere in the program.");
}
