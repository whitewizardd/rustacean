

// fn main() {

//     let owner_a = String::from("i am the real owner , a");

//     // the value of owner_a is moved to owner_b, hence owner_a is out of scope
//     let owner_b = owner_a;

//     // println!("this is result to an error ::: {} and it existed the main function", owner_a);

//     println!("owner_b is now the new owner of owner_a {}", owner_b);

//     {
//         let mut owner_c = String::from("here is owner c and i am just within this scope");

//         println!("owner_c is just within this scope and nothing else , {}", owner_c);

//         // owner_b is moved here 
//         owner_c = owner_b; 
//     }

//     // an error will occur here as owner_c is encapsulated within the child scope
//     // let owner_d = owner_c ;
// }


// to p=roperly allocate ownership to a variable which needs a child scope 

fn main() {
    let owner_a = String::from(" i am owner a ");

    let owner_b = owner_a;

    let owner_d = {
        let owner_c = String::from("this is owner c and it would be assigned to owner_d immediately the child scope exit");

        println!("here is owner_c ::: {}", owner_c);
    };

    println!("owner_b is now owner of owner_a ::: {}", owner_b);
    println!("owner_d is now owner of owner_c ::: {:?}", owner_d);
}