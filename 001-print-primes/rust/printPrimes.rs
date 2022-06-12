fn findPrimes(max: i32) -> Vec<i32> {
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
    vec
}

fn main() {
    let mut aaa = Vec::new();
    aaa = findPrimes(100);
    println!("{:?}", aaa);
}
