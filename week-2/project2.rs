fn main() {
let tf:f64 = 450000.0 * 2.0;
let mf:f64 = 1500000.0 * 1.0
let hf:f64 = 750000.0 * 3.0
let df:f64 = 2850000.0 * 3.0
let af:f64 = 250000.0 * 1.0

let sum:f64 = tf + mf + hf + df + af;
let average:f64 = sum / 10.0;

println!("sum is {}, average is {}", sum, average) 

}