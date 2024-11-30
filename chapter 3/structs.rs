

// a struct is a collection or grouping of types
// with struct we can group associated data types together 

// fn main() {

//     // this below are attributes we need for door which are wriiten in different variables
//     let door_width : u32 = 100;
//     let door_height : u32 = 120;
//     let is_door_open : bool = false;


//     // but with struct we can group them all together as they are all associated to that of a door 

//     let living_room_door : Door = Door {
//         width: 100,
//         height: 120,
//         is_open: false
//     };

// }


#[derive(Debug)]
struct Door {
    width : u32,
    height : u32,
    is_open : bool
}

impl Door {
    fn create_new_door(width: u32 , height: u32 , is_open: bool ) -> Self {
        Door {
            width,
            height, 
            is_open
        }
    }

    fn open_door(&mut self) {
        self.is_open = true
    }

    fn close_door(&mut self) {
        self.is_open = false
    }
}

fn main() {
    let mut room_door : Door = Door::create_new_door(20, 30, false);

    println!("here is the room door {:?}", room_door);

    room_door.open_door();
    assert!(room_door.is_open)
}