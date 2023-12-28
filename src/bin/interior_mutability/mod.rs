pub mod cell;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        let c = cell::Cell::new(1);
        let v = c.get();
        c.set(2);
        assert!(v == 1);
    }
}
