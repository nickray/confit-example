use confit::lib_a::{A, U};

pub const X: [u8; A] = [U; A];

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
