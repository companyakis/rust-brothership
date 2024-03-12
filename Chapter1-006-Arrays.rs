//An array is a fixed length collection of data elements all of the same type.

fn main() {
    let numbers: [i8; 5] = [-100, -50, 0, 50, 100];
    println!("{:?}", numbers);
    println!("Third element is {}", numbers[2]);
}

/*

[-100, -50, 0, 50, 100]
Third element is 0

*/
