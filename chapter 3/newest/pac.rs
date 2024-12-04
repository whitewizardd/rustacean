


pub mod physical_access_control {

    #[derive(Debug)]
    pub struct Door {
        width: u8,
        height: u8,
        is_open : bool
    }

    impl Door {

        pub fn new ( width: u8, height: u8 , is_open : bool ) -> Self {
            Door {
                width,
                height,
                is_open
            }
        }
    }
}