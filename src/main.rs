fn main() {
    // ! shows that println() is a macro function. 
    println!("Hello, world!");
    println!("Rust is a secure language - Safety Guranteed");
    println!("FOUR RUST KEYWORDS ==== AS, MUT, LET, UNSAFE");
    // ===== AS
    println!(" 'As' is used to turn one primitive data type to another primitive data type.");
    // u8 (unsigned 8 bit), bool, pointer, str , array, i32, i8, fn, tupble are primitive 
    println!(" 'As' can also turn pointers into addresses or vice versa.");
    println!("FROM and INTO are also used to change types, but mostly used on types such as String or Vec"); 
    // ===== MUT
    println!("'MUT' mutable");
    println!("There are situations more data needs to be added to a variable holding data already.");
    // In other programming languages just add more values no need to declare a variable mutable first.
    println!("To increase the size or add more data, the variable will have to be mutable.");
    println!("To declare a variable as mutable see example : let mut _num = 11;");
    println!("Now if +2 needs to be added to _num, _num +=2; if mut keyword wasn't used, error.");
    // ===== LET 
    println!("To declare a variable use Let.");
    // ===== UNSAFE 
    println!("UNSAFE");
    println!("Rust has another hidden language within it - "); 
    println!("To discard rust's memory safety guards unsafe keyword is used.");
    println!("'unsafe' is dangerous to use, however sometimes computer hardware is unsafe rust won't allow unsafe code, but unsafe keyword can help.");
    println!("THANK YOU"); 


}

