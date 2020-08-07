pub mod lib_a {
    pub const A: usize = 3;
    pub const U: u8 = 100;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
