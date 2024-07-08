fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn move_semantics5() {
        let mut x = 100;
        let y = &mut x;
        *y += 100;
        let z = &mut x;
        *z += 1000;
        assert_eq!(x, 1200);
    }
}
