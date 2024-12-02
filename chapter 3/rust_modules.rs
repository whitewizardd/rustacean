

mod PhysicalAccessControl {

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