pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn mean(list: &[i32]) -> f64 {
	let sum: f64 = list.iter().map(|&x| x as f64).sum();
	let len = list.len();
	if len == 0{
		0.0
	}else{
		sum/len as f64
	}
	
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted_arr = list.to_vec();
    sorted_arr.sort();
    let n = sorted_arr.len();
    let num: i32 ;

    if n % 2 == 0 {
        num = (sorted_arr[n/2] + sorted_arr[n/2 - 1]) / 2;
    } else {
        num = sorted_arr[(n - 1) / 2]
    }
    num
}

pub fn mode(list: &[i32]) -> i32 {
    let mut num :i32 = 0;
    let mut hld = 0;
    let mut hmap = HashMap::new();
    for num in list {
        *hmap.entry(num).or_insert(0) += 1;
    }

    for (key, value) in hmap.iter() {
        if *value > hld {
            hld = *value;
            num = **key;
        }
    }

    num
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
