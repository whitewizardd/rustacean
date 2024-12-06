// when it comes to rust, we have two major ways of handling errors which are 
// Result <T, E> which has Ok(T) and Err(E) where T is the actual ecpected result and E is the error
// Option<T> which has Some(T) and a general error;


fn result_enum_type_example (fail: bool) -> Result<u8, String> {
    if !fail {
        Ok(100)
    }
    else {
        Err("Error occurred".to_string())
    }
}

// // this is the main function without using the ? operator
// fn main() {
//     let result = result_enum_type_example(false);

//     match result {
//         Ok(value) => println!("Result is Ok and value is ::: {}", value),
//         Err(error) => println!("Result is Err and error is ::: {}", error),
//     }
// }

//using the ? operator and i think does unwrap as well
fn main() -> Result<(), String> {

    let result = result_enum_type_example(false)?;

    println!("Result is Ok and value is ::: {}", result);

    Ok(())
}