pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn fibonacci(n: u32) -> u32 {
    let mut first: u32 = 0;
    let mut second: u32 = 1;
    let mut index: u32 = 0;

    loop{
        if n == index{
            return first;
        }
        let temp = first;
        first = second;
        second += temp;
        index += 1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
