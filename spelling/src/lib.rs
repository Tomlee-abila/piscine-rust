pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let under_20 = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
        "seventeen", "eighteen", "nineteen",
    ];

    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    fn helper(n: u64, under_20: &[&str], tens: &[&str]) -> String {
        if n < 20 {
            under_20[n as usize].to_string()
        } else if n < 100 {
            let ten = n / 10;
            let rest = n % 10;
            if rest == 0 {
                tens[ten as usize].to_string()
            } else {
                format!("{}-{}", tens[ten as usize], under_20[rest as usize])
            }
        } else if n < 1000 {
            let hundred = n / 100;
            let rest = n % 100;
            if rest == 0 {
                format!("{} hundred", under_20[hundred as usize])
            } else {
                format!("{} hundred {}", under_20[hundred as usize], helper(rest, under_20, tens))
            }
        } else if n < 1_000_000 {
            let thousand = n / 1000;
            let rest = n % 1000;
            if rest == 0 {
                format!("{} thousand", helper(thousand, under_20, tens))
            } else {
                format!("{} thousand {}", helper(thousand, under_20, tens), helper(rest, under_20, tens))
            }
        } else {
            "one million".to_string()
        }
    }

    helper(n, &under_20, &tens)
}
