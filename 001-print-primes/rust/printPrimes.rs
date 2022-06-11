fn aaa() {
    for dividend in 1..10 {
        for dividor in 1..dividend {
            let aaa = dividend / dividor;
            println!("{}", dividend, dividor, aaa);
        }
    }
}

fn main() {
    aaa();
}
