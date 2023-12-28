# Rust Experiments

Repo for diving in deeper into what happens in Rust beyond rustlings.

## Smart Pointers and Interior Mutability

### Cell

[std::Cell](https://doc.rust-lang.org/std/cell/struct.Cell.html)

Allow for multiple references that can be mutated even with aliasing. `Cell<T>`
with `RefCell<T>` and `OnceCell<T>` provides interior mutability, i.e. mutate
with `(&T)` instead of what other data types do with inherited mutability (`&mut T`).

The issue being solved is given an immutable container(struct,collection, etc) we
cannot mutate the value inside them unless we mark the entire container mutable
or use exclusive reference

For `Cell` there is no way to get a reference to the inner value. We can get swap,
replace and get the copy of the value. There if no one has a reference to the value
it is safe to mutate it.

`Cell` has `get_mut(&mut self) -> &mut T` but for this you need exclusive value to the
Cell to get the reference. this prevents dangling references or dirty reads.

`Cell` does not implement `Sync` therefore its reference cannot be given to another thread.

`Cell` is generally used with small copy types
