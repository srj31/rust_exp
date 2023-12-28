use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        Cell {
            value: value.into(),
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }

    // Copy is required else it forces a move from the raw pointer
    pub fn get(&self) -> T {
        unsafe { *self.value.get() }
    }
}
