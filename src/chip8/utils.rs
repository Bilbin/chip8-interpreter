pub struct Utils {}

impl Utils {
    pub fn resolve_hex(nibbles: &[char]) -> u16 {
        let mut num = String::new();
        for i in nibbles {
            num.push(*i);
        }
        u16::from_str_radix(&num, 16).expect("Failed to convert hex number")
    }
}
