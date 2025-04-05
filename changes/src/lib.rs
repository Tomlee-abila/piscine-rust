pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
	pub alias: String,
	pub brightness: u8,
}

impl Light {
	pub fn new(alias: &str) -> Self {
	    Self{
	        alias: alias.into(),
	        brightness: 0
	    }
	}
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    for light in lights.iter_mut(){
        if light.alias == alias{
            light.brightness = value;
        }
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
