fn main() {
    
    let mut a = 5;
    
    let message = loop {
    
        a = 3 * a - 1; //a values 5-14-41
        
        if a > 20 {
            break "Whe have to break the loop";
        }

    };
    
    println!("Loop message: {} and the last a value: {}", message, a); //Loop message: Whe have to break the loop and the last a value: 41
}
