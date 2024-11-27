

// fn main() {
//     // a loop is where the program keeps running indefinitely until a condition is met

//     let mut i = 0;

//     loop {
//         println!("before increment , i is ::: {}", i);

//         i += 1;

//         println!("after increment , i is ::: {}", i);
//         if i == 12 {
//             break
//         }
//     }
// }

// fn main() {
//     // learning the for loop 

//     for n in 0 ..10 {
//         // the n act as each number in the range starting from 0 and stopping at 9 i.e, including the first value and excluding the last

//         println!("n is ::: {}", n)
//     }

//     println!();

//     for n in 0 ..= 10 {
//         // it starts at 0 and the n act as each number in the range starting from 0 and stopping at 10 because an = has been added to tell the compiler to include it 
//         println!("n is ::: {}", n)
//     }
// }


fn main() {
    // learning the while loop

    let mut number = 0;

    while number <= 12 {

        println!("number value is ::: {}", number);

        number += 1
    };

    println!("number result finally ::: {} ", number);
}
