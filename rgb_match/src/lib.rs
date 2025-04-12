#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        self.r = swap_value(self.r, first, second);
        self.g= swap_value(self.g, first, second);
        self.b= swap_value(self.b, first, second); 
        self.a= swap_value(self.a, first, second);
        self        

    }
}

fn swap_value(value: u8, first: u8, second: u8)-> u8{
    if value == first{
        second
    }else if value == second{
        first
    }else{
        value
    }
}