fn main() {

    //we can use if, match...
    
    let year: u16 = 2023;
    
    let error_result = if year < 2024 {-100} else {100};
    
    println!("Error result: {}", error_result); //Error result: -100

}
