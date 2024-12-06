


fn matching_the_variant(number : usize) -> String {

    match number {
        0_usize | 1_usize  => "zero or one ".to_string(), 
        2_usize .. 9_usize => "betweeen two to nine".to_string(),
        10_usize => "ten".to_string(),
        _ => "not a valid number".to_string(),
    }
}

fn main() {

    let result : String = matching_the_variant(00100);

    println!("result is ::: {}", result);
}