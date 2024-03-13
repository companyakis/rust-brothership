fn main() {
    
    let year: u16 = 1991;
    
    match year {
        
        2024 => {println!("This year and I hope everything will be ok!");}
        
        2018..=2023 => {println!("Awful years!");}
        
        2025 | 2026 => {println!("Near future...");}
        
        _ => {println!("Out of sight, out of mind...");}
    }
    
}
