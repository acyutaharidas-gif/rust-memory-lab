mod ownership;
mod stack_heap;
mod unsafe_patterns;

fn main() {
    stack_heap::run();
    println!();
    ownership::run();
    println!();
    unsafe_patterns::run();
}
