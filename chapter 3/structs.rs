

// a struct is a collection or grouping of types
// with struct we can group associated data types together 

fn main() {

    // this below are attributes we need for door which are wriiten in different variables
    let door_width : u32 = 100;
    let door_height : u32 = 120;
    let is_door_open : bool = false;


    // but with struct we can group them all together as they are all associated to that of a door 
    

}

struct Door {
    width : u32,
    height : u32,
    is_open : bool
}