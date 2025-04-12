pub fn get_diamond(c: char) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let len: usize = index(c);

    for (i, ch) in ('A'..=c.to_ascii_uppercase()).enumerate(){
        let mut row: String;
        
        row = format!("{}{}", " ".repeat(len-1-i), ch);
        if i > 0{
            row = format!("{}{}{}", row," ".repeat(i), ch);
        }
        row = format!("{}{}", row," ".repeat(len-1-i));
        
        result.push(row);
    }

    for i in (0..len-1).rev(){
        result.push(result[i].clone());
    }
    result

}

fn index(ch: char) -> usize{
   ch.to_ascii_uppercase() as usize + 1 -'A' as usize
}