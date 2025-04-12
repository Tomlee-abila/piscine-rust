pub fn get_diamond(c: char) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let len: usize = index(c);

    for (i, ch) in ('A'..=c.to_ascii_uppercase()).enumerate(){
        let mut row: String;
        let size = (len*2)-1;
        let r = len-1-i;
        
        row = format!("{}{}", " ".repeat(r), ch);
        if i > 0{
            row = format!("{}{}{}", row," ".repeat(size - (r*2)-2), ch);
        }
        row = format!("{}{}", row," ".repeat(r));

        println!("i:{} len:{} row:[{}]",i, row.len(), row);
        
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