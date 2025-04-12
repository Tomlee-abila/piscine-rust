pub fn number_logic(num: u32) -> bool {    
    let mut digits = Vec::new();
    let mut copy = num;

    while copy > 0{
        digits.push(copy%10);
        copy /= 10;
    }
    let len: u32 = digits.len() as u32;
    digits.iter().map(| n| n.pow(len)).sum::<u32>() == num
}