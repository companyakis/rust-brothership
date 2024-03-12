fn main() {

    let x: u8 = 21;
    
    let y = 10i8; //i8
    
    //let sum = x + y;
    
    //println!("x + y = {}", sum); //throw an error!
    
    let sum = x + y as u8;
    
    println!("x + y = {}", sum); //x + y = 31

}
