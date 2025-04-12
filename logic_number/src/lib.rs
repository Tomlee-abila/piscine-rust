pub fn number_logic(num: u32) -> bool {
    let mut sum: u32 = 0;

    let number = num.to_string();
   
    for (i, ch) in number.chars().enumerate(){        
        sum += (ch as u32 - '0' as u32).pow(i as u32+1);
        println!("i:{}, sum:{}, num:{}, ch:{}",i,sum, num, ch);
    }
    
    num == sum
}