mod interior_mutability;

use interior_mutability::cell::Cell;

fn main() {
    let c = Cell::new(1);
}
