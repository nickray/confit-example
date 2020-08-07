pub mod lib_b {
    pub const B: usize = 2;
    pub const V: u8 = 200;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
