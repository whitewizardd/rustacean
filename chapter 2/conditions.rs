

fn main() {
    // when dealing with conditions and conditional statements, we sometimes deal with boolean algebra

    // true && true === true, true && false === false
    // true || true === true, true || false === true
    // !false === true, !true === false
    // true ^ true === false , true ^ false === true

    //examples below
    let greater_than : bool = 5 < 10;
    let not_equal_to : bool = 1 != 9;
    let equal_to : bool = 0x5ff == 0x5ff;

    let combined : bool = greater_than && not_equal_to || equal_to > true;

    println!("{} ::: {} ::: {}", equal_to, equal_to > true, equal_to > false);

    println!("result is ::: {}", combined);
}