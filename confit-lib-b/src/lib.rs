pub mod lib_b {
    pub const B: usize = 4;
    pub const V: u8 = 250;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
