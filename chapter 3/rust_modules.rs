

mod physical_access_control {

    #[derive(Debug)]
    pub struct Door {
        pub width: u8,
        height: u8,
        is_open: bool
    }

    impl Door {
        pub fn new (width: u8, height: u8, is_open: bool ) -> Self {
            Door {
                width,
                height,
                is_open
            }
        }
    }
}


use physical_access_control::Door;
fn main () {
    let door : Door = physical_access_control::Door::new(20, 50, true);

    println!("here is the door that was resturned ::: {}", door.width);
}