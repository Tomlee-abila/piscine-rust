fn main() {
    println!("{}", stars(20))
}

pub fn stars(n: u32) -> String {
    "*".repeat(2u32.pow(n) as usize).to_string()
}