

use physical_access_control:: Door;



fn main() {

    let door : Door = physical_access_control::Door::new(50,150, true);

    println!("{:?}", door);
}