# Rust Memory Safety Lab

Annotated Rust programs demonstrating stack vs. heap allocation,
ownership-based memory safety guarantees, and `unsafe` Rust patterns.
Documents the memory error classes that `unsafe` unlocks and that underlie
common exploit primitives in systems software.

## Structure

| Module | What it shows |
|---|---|
| [`src/stack_heap.rs`](src/stack_heap.rs) | Stack-allocated `Copy` types vs. heap-backed `Vec` buffers; move semantics when ownership of heap data transfers. |
| [`src/ownership.rs`](src/ownership.rs) | How the borrow checker statically rejects use-after-free (`E0597`) and aliased mutable access (`E0502`) — bug classes that are runtime hazards in C/C++. |
| [`src/unsafe_patterns.rs`](src/unsafe_patterns.rs) | `unsafe` Rust reopening the two bug classes safe Rust closes: **use-after-free** ([CWE-416](https://cwe.mitre.org/data/definitions/416.html)) and **buffer overread** ([CWE-125](https://cwe.mitre.org/data/definitions/125.html)), with notes on how each maps to real exploit primitives (UAF reallocation/hijacking, Heartbleed-style info leaks). |

## Run it

```bash
cargo run
```

Each module prints its own section to stdout, in this order:
stack vs. heap → ownership guarantees → unsafe memory errors.

## Why this matters

Rust's pitch is "memory safety without garbage collection," enforced at
compile time by the ownership and borrowing rules in `ownership.rs`. The
`unsafe` keyword is not a loophole in that pitch — it's the explicit,
`grep`-able boundary where those guarantees are suspended and the
programmer takes over the compiler's job. This lab makes both halves of
that story concrete: what the checker prevents, and what reappears the
moment you opt out of it.

## Disclaimer

`unsafe_patterns.rs` intentionally triggers undefined behavior (UB) for
teaching purposes. UB output is illustrative, not guaranteed — behavior can
differ across platforms, allocators, and compiler versions. Do not reuse
these patterns outside a lab/teaching context.

## License

MIT
