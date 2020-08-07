use confit::lib_b::{B, V};

pub const Y: [u8; B] = [V; B];

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
