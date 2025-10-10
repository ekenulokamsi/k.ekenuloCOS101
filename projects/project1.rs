fn main() {
    let p:=520,000,000
    let r:=10
    let n:=5

    // compound interest
    let a = p *(1 + (r / 100)).powf(t);
    println!("Amount is {}", a);
    let ci = a - p;
    println!("Compound interest is {}", ci);

     }