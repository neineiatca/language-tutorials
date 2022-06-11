fn aaa() {
    for dividend in 1..10 {
        for dividor in 1..dividend {
            let dividendAsFloat = dividend as f64;
            let dividorAsFloat = dividor as f64;
            let aaa = dividend / dividor;
            let bbb = aaa as f64;
            let aaaAsFloat = dividendAsFloat / dividorAsFloat;
            if bbb == aaaAsFloat {
                println!("no remaider {}, {}", dividend, dividor);
            }
        }
    }
}

fn main() {
    aaa();
}
