fn perimeter_area_rectangular(x: u32, y: u32) -> (u32, u32) {
    (2 * (x + y), x * y)
}

fn main() {

    println!("Perimeter: {}", perimeter_area_rectangular(12, 21).0);
    
    println!("Area: {}", perimeter_area_rectangular(12, 21).1);

}

/*

Perimeter: 66
Area: 252

*/
