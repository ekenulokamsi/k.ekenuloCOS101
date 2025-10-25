// Rust progam to determine the roots of a quadratic equation


use std::io;
    
fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter a: ",);
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");
    
    println!("Enter b: ",);
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter c: ",);
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");
    
    let discriminant:f32 = (b * b) - 4.0 * a * c;
    if discriminant > 0.0 {
    let first_root = (-b + discriminant.sqrt() )/( 2.0*a);
    let second_root = (-b - discriminant.sqrt() )/( 2.0*a);
        println!(" there are two real roots");
        println!("The roots are {} and {}",first_root,second_root);
    } 

    else if discriminant==0.0{
        let root = -b/(2.0 * a);
        println!(" there is exactly one real roots");
        println!(" the root is {}",root);
    }

    else {
        println!(" there are no real root");
    }
    
}