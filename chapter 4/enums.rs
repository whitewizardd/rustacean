

enum Weekdays {
    Monday(String), 
    Tuesday(String),
    Wednesday(String),
    Thursday(String),
    Friday {display_name : String},
    Saturday {display_name : String},   
}

impl Weekdays {

    fn get_day_of_the_week(&self) -> String {
        
        match self {
            Weekdays::Monday(name)  
            | Weekdays::Tuesday(name) 
            | Weekdays::Wednesday(name)
            | Weekdays::Thursday(name) => name.to_string(),
            Weekdays::Friday { display_name } | 
            Weekdays::Saturday { display_name } => display_name.to_string(),
        }
    }
}


fn main() {
    let monday : Weekdays = Weekdays::Monday("Monday".to_string());

    let monday_enum_result : String = monday.get_day_of_the_week().to_string();

    println!("here is the result of the {:?}", monday_enum_result);

    let friday : Weekdays = Weekdays::Friday { display_name : "Friday".to_string()};

    let friday_enum_result : String = friday.get_day_of_the_week();

    println!("here is the friday result {:?}", friday_enum_result);
}