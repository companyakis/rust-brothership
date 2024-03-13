fn main() {
    
    let mut counter = 0u16;
    
    loop {
        counter = 2 * counter + 1;
        
        if counter > 300 {
            break;
        }
        print!{"{} ", counter};
    }
    println!("Counter ended!");
}

//1 3 7 15 31 63 127 255 Counter ended!
