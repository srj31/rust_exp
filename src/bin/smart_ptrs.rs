mod interior_mutability;

use interior_mutability::cell::Cell;

fn main() {
    println!("Hello, world!");
    let c = Cell::new(1);
}
