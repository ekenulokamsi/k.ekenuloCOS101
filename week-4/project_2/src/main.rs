use std::io;

fn main() {


println!("Enter experience:");
let mut experience = String::new();
io::stdin().read_line(&mut experience).expect("Not a valid string");
let mut experience:String = experience.trim().to_string();

println!("Enter age:");
let mut age = String::new();
io::stdin().read_line(&mut age ).expect("Not a valid string");
let  age:u32 = age.trim().parse().expect("Not a valid number");

if experience == "yes" && age >=40{
    println!("The incentive of the employee is 1,560,000.00");

}
else if experience == "yes" && age>=30 && age <40{
    println!("The incentive of the employee is 1,480,000.00");

}
else if experience == "yes" && age<=28{
    println!("The incentive of the employee is 1,300,000.000");

}
else {
    println!("The incentive of the employee is 100,000.00");

   }
}
