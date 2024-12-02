

mod physical_access_control {

    #[derive(Debug)]
    struct Door {
        width: u8,
        height: u8,
        is_open: bool
    }

    impl Door {
        fn new (width: u8, height: u8, is_open: bool ) -> Self {
            Door {
                width,
                height,
                is_open
            }
        }
    }
}


fn main () {
    let door : Door = physical_access_control::Door::new(20, 50, true);

}