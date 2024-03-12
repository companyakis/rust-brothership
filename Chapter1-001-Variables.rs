fn main() {
    // rust infers the type of this_year
    let my_number = 2024;
    println!("{}", my_number);

    // rust can also be explicit about the type
    //we can assign to the same variable name multiple times
    //This is called variable shadowing 
    //and the type can be changed for subsequent references to that name
    let my_number: f32 = 20.24;
    println!("{}", my_number);

    // rust can also declare and initialize later, but this is rarely done
    let my_number;
    my_number = 101;
    println!("{}", my_number);
}

/*

2024
20.24
101

*/
