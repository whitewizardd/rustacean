
// to achieve mutability, we can use shadowing .
// shadowing is creating another variable that was once declared.
// fn main() {

//     let x = 20;
//     let x = 50;

//     assert_eq!(x , 50);
// }


//by reassigning the value to another variable;
// fn main() {

//     let x = String::from("rust");

//     let z = x + "acean";

//     // assert_eq!(z , "rustacean");
//     println!("{}", z);
// }

// we can also achieve mutablility by declaring the variable as mutable using the keyword mut

// fn main() {
//     let mut x = Vec::<String>::new();

//     x.push(String::from("Hello"));
//     x.push(String::from("World"));
//     x.push(String::from("rustacean"));

//     println!("result of the vector is ::: {:?}", x)

// }


// mutablility can also be achieved by using functions and for structs as well

fn join_together(target: &mut String, separator: char , list: &[&str]) {

    // this bool variable below allow us to control the dynamics of the instruction
    let mut is_first : bool = true;

    for element in list.iter() {
        // the if statement checks if the is_first variable is not true , if it isn't then it adds the separtor to the target as a demacator
        if !is_first {
            target.push(separator);
        }
        else {
            is_first = false;
        }

        target.push_str(element);
    }
}

fn main() {

    let list = ["goat", "sheep", "cow"];
    let separator : char = '-';
    let mut target = String::new();

    join_together(&mut target, separator, &list);

    println!("target values are ::: {}", target)
}