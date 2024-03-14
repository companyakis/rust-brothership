fn main() {
  
    // Using a static method to create an instance of String
  
    //String::from is a static method
    
    let my_name = String::from("Mustafa Büyükdereli");
    
    //.len() is an instance method
    
    // Using a method on the instance
    
    println!("My name is {} and length is: {}", my_name, my_name.len()); //My name is Mustafa Büyükdereli and length is: 21
}


//methods are functions associated with a specific data type

//static methods — methods that belong to a type itself are called using the :: operator

//instance methods — methods that belong to an instance of a type are called using the . operator

