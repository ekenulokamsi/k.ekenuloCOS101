fn main() {
    let p:f64 = 510,000.0
    let r:f64 = 5.0
    let t:f64 =3.0
    
    // compond interest
    let a = p* ( 1.0 - (r / 100)).powf(t)
    println!("Amount is {}", a);
    let ci = a-p; 
    println!("Compound Interest {}", ci)
    
}
