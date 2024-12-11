
// to achieve mutability, we can use shadowing .
// shadowing is creating another variable that was once declared.
// fn main() {

//     let x = 20;
//     let x = 50;

//     assert_eq!(x , 50);
// }


//by reassigning the value to another variable;
fn main() {

    let x = String::from("rust");

    let z = x + "acean";

    // assert_eq!(z , "rustacean");
    println!("{}", z);
}