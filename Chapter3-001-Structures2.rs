struct TeamMembers {
    id: u8,
    name: String,
    title: String,
}

fn main() {
    // SeaCreature's data is on stack
    let founder = TeamMembers {
        id: 101,
        name: String::from("Mustafa Büyükdereli"),
        title: String::from("Founder"),

    };

    let cfo = TeamMembers {
        id: 102,
        name: String::from("Aygün"),
        title: String::from("Chief Financial Officer"),
    };
    
    
    println!("Name: {} - id: {} - title: {}", founder.name, founder.id, founder.title); // Name: Mustafa Büyükdereli - id: 101 - title: Founder
    
    
}

