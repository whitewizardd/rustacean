

// fn main() {

//     // here we are using the if else statement to ascertain the program to perform an action if either of them it true or false

//     let number : u8 = 8;

//     if number % 2 == 0 {
//         println!("this is an even number {}", number);
//     }
//     else {
//         println!("it is an odd number {}", number);
//     }
// }

fn main() {

    // we could also perform many other actions using the if else if else

    // let number : u8 = 1_u8;

    // if number % 2 == 0 {
    //     println!("this is an even number ::: {}", number);
    // }
    // else if number == 1 {
    //     println! ("number is an odd number and is {}", number) ;
    // }
    // else {
    //     println!("number is an odd number {}", number);
    // }

    // let result_1 : bool = 1 != 1;
    // let result_2 : bool = 1 != 0;

    // println!("{}",&result_1);
    // println!("{}",&result_2);
    // println!("{}", result_1 || result_2)


    // again the if else expression can be used to return a value either by using the return keyword or using the idiomatic expression way of no semicolon

    let result : String = {
        let number : u8 = 1_u8;
        if number == 1 {
            "one".to_string()
        }
        else {
            "not one".to_string()
        }
    };

    println!("{}", result)
}