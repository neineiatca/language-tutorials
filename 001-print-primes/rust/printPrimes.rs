fn aaa(max: u32) {
    let mut vec = Vec::new();
    for dividend in 1..max {
        let mut isPrime = true;
        for dividor in 2..dividend {
            let dividendAsFloat = dividend as f64;
            let dividorAsFloat = dividor as f64;
            let integerResult = dividend / dividor;
            let converted = integerResult as f64;
            let floatResult = dividendAsFloat / dividorAsFloat;
            if converted == floatResult {
                isPrime = false;
                break;
            }
        }
        if isPrime {
            vec.push(dividend);
        }
    }
    println!("{:?}", vec);
}

fn main() {
    aaa(100);
}
