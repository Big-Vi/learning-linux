use core::{char, fmt};

fn main() {
    let mut my_var: u8 = 50;
    println!("Hello, world! {}", my_var);
    my_var = 100;
    println!("Int {}", my_var);

    // Flaot
    let my_float: f32 = 3.140053433400943;
    println!("Float {}", my_float);

    // Cast int to float
    let my_var : f32 = my_var as f32;
    println!("Casted int {}", my_var);

    // Print format
    println!("Float with 2 decimal: {:.2}", my_float);
    // pad with 8 spaces
    println!("Float with 8 spaces: {:8.2}", my_float);
    // Pad with number 0 with 8 spaces
    println!("Float with 8 spaces 0 padded: {:08.2}", my_float);
    // Print with two variables
    println!("Float with two variables:\n my_float is {:08.2}\n and my_var is {}", my_float, my_var);
    // Print with not end with new line
    print!("Float with two variables:\n my_float is {:08.2}\n and my_var is {}\n", my_float, my_var);
    // Print with position parameter
    println!("Float with position parameter: {1} and {0:.2}", my_float, my_var);

    // Check the position of bitwise operator
    let a = 1 << 5;
    println!("Bitwise operator: {}", a);
    // Provide example how the shift works
    let b = 0b00000001 << 5;
    println!("Bitwise operator shifted bits of 1: {:08b}", b);

    // or operator
    let c = 0b10100000 | 0b10100100;
    println!("Or operator: {:08b}", c);

    // xor operator
    let d = 0b10100010 ^ 0b10100000;
    println!("Xor operator: {:08b}", d);

    // and operator
    let e = 0b10100010 & 0b10100000;
    println!("And operator: {:08b}", e);

    // not operator
    let f = !0b10100010;
    println!("Not operator: {:08b}", f);

    // Right shift operator
    let g = 0b10100010 >> 2;
    println!("Right shift operator: {:08b}", g);

    // Boolean
    let h = true;
    let i = false;
    println!("Boolean: {} {}", h, i);
    // !, &, |, ^ - print
    println!("Boolean: {} {} {} {}", !h, h & i, h | i, h ^ i);

    // Short circuit logical operator
    let j = true && false;
    println!("Short circuit logical operator: {}", j);
    let k = true || false;
    println!("Short circuit logical operator: {}", k);

    // Char
    let l = 'a';
    println!("Char: {}", l);
    // Unicode
    let m = '\u{1F600}';
    println!("Unicode: {}", m);

    // Tuple - Debug with ? and pretty print - Can have mixed data type
    // Fixed length, Contiguous memory
    // // Tuple holds related data
    let n: (u8, u16, f32, char, &str) = (1, 256, 30.5, 'a', "Hello");

    println!("Tuple: {n:?}");
    println!("Tuple: {}", n.0);

    // Tuple destructuring
    let (my_int8, my_int16, my_float32, my_char, my_str) = n;
    println!("Tuple destructuring: {} {} {} {} {}", my_int8, my_int16, my_float32, my_char, my_str);

    // Array
    let o = [1, 2, 3];
    println!("Array: {o:?}");

    // Two dimensional array
    let p = [[1, 2], [3, 4]];
    println!("Two dimensional array: {p:?}");

    // Array with type and length
    let q: [i32; 3];
    q = [1; 3]; // Repeat 1 three times
    println!("Array with type and length: {q:?}");

    // Slice
    let r = &o[1..3];
    println!("Slice: {r:?}");

    // Array index data type - usize
    let s:usize = o.len();
    println!("Array index data type: {}", s);

    // Function
    my_function("Hello, world!");

    // Statement
    let _t = 5;

    // Expression
    let _u = {
        let t = 3;
        t + 1 // No semicolon
    };

    // Function with return
    let v = my_function_with_return(5);
    println!("Function with return: {v:?}");

    // Unit data type - No value
    // Used to return nothing
    let w: () = ();
    println!("Unit data type: {:?}", w);

    // If else
    let x = 5;
    if x < 5 {
        println!("If else: x is less than 5");
    } else if x == 5 {
        println!("If else: x is equal to 5");
    } else {
        println!("If else: x is greater than 5");
    }

    // Rust compiler checks every branch of if else
    // If else is an expression
    let y = if x < 5 {
        "y is less than 5"
    } else if x == 5 {
        "y is equal to 5"
    } else {
        "y is greater than 5"
    };
    println!("If else expression: {}", y);

    // Loop
    let mut z = 0;
    loop {
        z += 1;
        if z == 5 {
            break;
        }
    }

    // While loop
    let mut count = 0;
    while count < 5 {
        count += 1;
    }

    // For loop
    for i in 0..5 {
        println!("For loop: {}", i);
    }

    // For loop with range and reverse
    for i in (0..5).rev() {
        println!("For loop with range: {}", i);
    }

    // for loop with array and enumerate
    let mut my_array = [10, 20, 30, 40, 50];
    for (index, value) in my_array.iter().enumerate() {
        println!("For loop with array and enumerate: {} {}", index, value);
    }

    // Use iter mut to modify the array
    for value in my_array.iter_mut() {
        *value += 1;
    }
    println!("Muted my_array: {:?}", my_array);

    // Nested loop
    for i in 0..5 {
        for j in 0..5 {
            println!("Nested loop: {} {}", i, j);
        }
    }

    // Match
    let number = 3;
    match number {
        1 => println!("Match: One"),
        2 => println!("Match: Two"),
        3 => println!("Match: Three"),
        _ => println!("Match: Other"),
    }

    // Shadowing
    let my_var = 5;
    let my_var = my_var + 1;
    let my_var = my_var * 2;
    println!("Shadowing: {}", my_var);
    // Scope & Shadowing
    let my_var = "Hello";
    {
        let my_var = 5;
        println!("Scope & Shadowing: {}", my_var);
    }
    println!("Scope & Shadowing: {}", my_var);

    // Constants
    const MY_CONST: i32 = 10;
    println!("Constants: {}", MY_CONST);

    // Static
    static MY_STATIC: i32 = 10;
    println!("Static: {}", MY_STATIC);

    // Reference
    let my_string = String::from("Hello, world!");
    let my_string_ref = &my_string;
    println!("Reference: {}", my_string_ref);

    // Mutable reference
    let mut my_string = String::from("Hello, world!");
    let my_string_ref = &mut my_string;
    my_string_ref.push_str("!");

    // Stack and Heap memory - Pointer
    // Stack - Fixed size, LIFO, Fast, Small - Box
    // Heap - Dynamic size, Slower, Larger - Warehouse
    // String - Heap
    // &str - Stack

    // String literal - Hard coded in the executable
    // Immutable & cannot be modified at runtime
    let my_string = "Hello, world!";
    println!("String literal: {}", my_string);

    // String type - Heap
    // Mutable & can be modified at runtime
    // Stack stores the pointer to the heap and the length and capacity
    // Capacity will be doubled when the string is full
    // Capacity will be reduced when the string is half full
    // If string grows out of size, it will be moved to a new location
    let my_string = String::from("Hello, world!");
    println!("String type: {}", my_string);

    // String concatenation
    let my_string = "Hello, ".to_string() + "world!";
    println!("String concatenation: {}", my_string);

    // Ownership
    // Rust has no garbage collector
    // Ownership is a set of rules that the compiler checks at compile time
    // Stack - Copy
    // Heap - Move
    // Ownership is moved when the variable is assigned to another variable
    // Variables are responsible for freeing the memory when they go out of scope
    let my_string = String::from("Hello, world!");
    let _my_string2 = my_string.clone();
    println!("Ownership: {}", my_string);

    // Transfering heap to another variable is called move
    // Stack is copied
    // Heap is moved
    // Old variable/pointer is invalidated
    // It's called shallow copy
    // Deep copy is when the heap is copied. It's expensive. Rust does not do deep copy
    // This is where we use clone
    // Int is stored in stack and it's copied

    // Transfer ownership
    let my_string = String::from("Hello, world!");
    let my_string2 = my_string;
    // println!("Transfer ownership: {}", my_string); // Error
    println!("Transfer ownership: {}", my_string2);

    // Pass back the ownership
    let my_string = String::from("Hello, world!");
    let my_string = my_function_with_ownership(my_string);
    println!("Pass back the ownership: {}", my_string);

    // Borrowing
    // Borrowing is when we pass the reference to the function
    // We can pass the reference to the function and return the reference
    // We can pass the reference to the function and return the value
    let my_string = String::from("Hello, world!");
    my_function(&my_string);

    // mutable reference
    let mut my_string = String::from("Hello, world!");
    my_function_mut(&mut my_string);

    // Restriction of borrowing
    // We cannot have mutable reference and immutable reference at the same time
    // We cannot have multiple mutable reference at the same time
    // Prevents data races

    // Danling reference
    // Rust prevents dangling reference
    // Rust prevents the reference to the memory that has been freed, moved & invalidated

    // Slice
    // Slice is a reference to the part of the string, array, & vector
    let my_string = String::from("Hello, world!");
    let my_string_slice = &my_string[0..5];
    println!("Slice: {}", my_string_slice);
    println!("Slice: {}", &my_string[7..]);

    // Slice String length is in bytes
    // Strings are UTF-8 encoded - So some characters are more than 1 byte
    // So we cannot use index to get the character
    // We can use char_indices to get the character
    // Range indices must occur at the UTF-8 character boundary
    for (index, value) in my_string.char_indices() {
        println!("Slice: {} {}", index, value);
    }

    // Slice as function parameter
    // &String != &str
    // &String is a reference to the String
    // &str is a reference to the slice of the String
    // Deref coercion - Rust automatically converts &String to &str
    // &String can be converted to &str
    // &str cannot be converted to &String - Becasue &str lacks the capacity and length
    // Rule of thumb - Use &str as function parameter and return type - It's more flexible
    let my_string = String::from("Hello, world!");
    let my_string_slice = &my_string[0..5];
    get_first_word(my_string_slice);

    // Standard library
    // Rust has a standard library
    // Rust has a package manager called cargo
    // Use Use statements to import the library
    // Prelude - Commonly used types and functions are imported by default
    // use::prelude::*;
    // Use std::io to get the input from the user - Example
    use std::io;
    let mut input = String::new();
    println!("Enter message: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("Your message is: {}", input);

    // Parse string
    let input: i32 = input.trim().parse().expect("Please type a number");
    println!("Parsed string: {}", input + 1);

    // Turbofish operator
    let mut input2 = String::new();
    println!("Enter number: ");
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let input2: i32 = input2.trim().parse::<i32>().expect("Please type a number");
    println!("Turbofish operator: {}", input2 + 1);

    // Crates
    // A collection of Rust source code files
    // Binary crate - Executable
    // Library crate - Reusable code
    // Example - rand crate from crates.io
    // Cargo.toml
    // [dependencies]
    // rand = "0.8.3"

    // Command line arguments
    // std::env::args
    // std::env::args_os
    // std::env::current_dir
    // std::env::current_exe
    use std::env;
    use std::io::prelude::*;
    if env::args().len() < 2 {
        println!("Please provide the command line arguments");
        std::process::exit(1);
    }
    for (index, argument) in env::args().enumerate() {
        println!("Command line arguments {} : {}", index, argument);
    }
    let argone = env::args().nth(1).unwrap();
    println!("Command line 1st arguments: {}", argone);

    // Reading file
    // std::fs::File
    // std::fs::OpenOptions
    // std::fs::create
    // std::fs::remove
    // std::fs::rename
    // std::fs::metadata
    // std::fs::read
    // std::fs::write
    // std::fs::read_to_string
    use std::fs;
    let file = fs::read_to_string("food.txt").unwrap();
    println!("Reading file: \n{}", file);
    file.lines().for_each(|line| println!("{}", line));
    // If file has other than text, use read
    let file = fs::read("food.txt").unwrap();
    println!("Reading file: {:?}", file);
    // Writing file
    fs::write("bread.txt", "Garlic bread").unwrap();
    let file = fs::read_to_string("bread.txt").unwrap();
    println!("Writing file: \n{}", file);
    // Append to file
    let mut file = fs::OpenOptions::new().append(true).open("bread.txt").unwrap();
    file.write(b"\nwith cheese").unwrap();

    // Struct
    // Struct is a custom data type
    // Struct is a way to group related data
    // Struct stores the data in the stack
    // If it contains string, it stores the pointer to the heap
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    let mut user1 = User {
        username: String::from("user1"),
        email: String::from("test@test.com"),
        sign_in_count: 1,
        active: true,
    };
    user1.username = String::from("user1_updated");
    println!("Struct: {}, {}, {}, {}", user1.username, user1.email ,user1.sign_in_count, user1.active);

    // Update struct
    let user2 = User {
        username: String::from("user2"),
        ..user1
    };
    println!("Update struct: {:?}", user2);
    // This will error because user1 is partially moved to user2. user name has unique ownership. but email not which is string type stored in heap
    // println!("Update struct: {:?}", user1);

    // Struct methods
    impl User {
        fn new(username: &str, email: &str) -> User {
            User {
                username: String::from(username),
                email: String::from(email),
                sign_in_count: 1,
                active: true,
            }
        }
        fn get_username(&self) -> &str {
            &self.username
        }
        fn get_email(&self) -> &str {
            &self.email
        }
        fn set_username(&mut self, username: &str) {
            self.username = String::from(username);
        }
    }
    let mut user3 = User::new("user3", "test3@test.com");
    println!("Struct methods: {:?}", user3);
    println!("Struct methods: {}", user3.get_username());
    println!("Struct methods: {}", user3.get_email());
    user3.set_username("user3_updated");
    println!("Struct methods: {:?}", user3);

    // Associated function
    // Associated function is a function that does not take self as a parameter
    // It's like a static method in other languages
    // It's used to create a new instance of the struct

    // Tuple Structs
    // Tuple structs are a way to create a new type
    // Tuple structs are a way to give a name to the tuple
    // Tuple structs are a way to give a name to the tuple and make it a different type
    #[derive(Debug)]
    struct Color(i32, i32, i32); // RGB
    let blue = Color(0, 0, 255);
    println!("Tuple Structs: {:?}", blue);
    println!("Last value {}", blue.2);

    // Generic Structs
    // Generic structs are a way to create a struct that can hold any type
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("Generic Structs: {}, {}", integer.x, integer.y);
    println!("Generic Structs: {:?}", float);

    // Struct with multiple types
    #[derive(Debug)]
    struct Point2<T, U> {
        x: T,
        y: U,
    }
    let both = Point2 { x: 5, y: 4.0 };
    println!("Struct with multiple types: {}, {}", both.x, both.y);

    // Runtime performance - Generics
    // Zero cost abstraction
    // What is zero cost abstraction - It means that the abstraction does not cost anything at runtime
    // Writing code easier without introducing runtime overhead
    // Rust uses monomorphization to create a new type for each type
    // Monomorphic code is faster than polymorphic code
    // Compiler replaces the generic type with the actual/concrete type
    struct Point3<T, U> {
        x: T,
        y: U,
    }
    let monomorphization = Point3 { x: 5, y: 10.00 };
    println!("Runtime performance: {}, {}", monomorphization.x, monomorphization.y);
    // What gets compiled
    // struct Point3_i32_f64 {
    //     x: i32,
    //     y: f64,
    // }
    // let monomorphization = Point3_i32_f64 { x: 5, y: 10.00 };

    // Generic method definition
    impl<T, U> Point3<T, U> {
        fn mixup<V, W>(self, other: Point3<V, W>) -> Point3<T, W> {
            Point3 {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("Generic method definition: {}, {}", p3.x, p3.y);

    // Concrete method implementation
    impl Point3<i32, f64> {
        fn mixup_concrete(self, other: Point3<&str, char>) -> Point3<i32, char> {
            Point3 {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };
    let p3 = p1.mixup_concrete(p2);
    println!("Concrete method implementation: {}, {}", p3.x, p3.y);

    // Generic function definition
    // PartialOrd is a trait
    // Trait is a way to define a set of methods that a type must implement
    // PartialOrd is a trait that defines the comparison methods
    // Rust has a set of traits that are implemented for the standard library types
    fn largest<T: PartialOrd>(a: T, b: T) -> T {
        if a > b {
            a
        } else {
            b
        }
    }
    println!("Generic function definition: {}", largest(5, 10));

    // Box data type
    // Box is a smart pointer, because it's a pointer that has additional metadata
    // Box is a way to store the data in the heap and get the pointer to the data
    let my_box = Box::new(5); // Now the data is stored in the heap
    println!("Box data type: {}", my_box);
    // Why use Box
    // Store the type that has unknown size at compile time
    // ex - Recursive data type
    // Store the type that has large size
    // ex - Large struct
    // Store the type that has a lifetime that is shorter than the reference
    // ex - Return the reference to the local variable
    // Transfer the ownership of data rather than copy it on the stack

    // Traits
    // Traits are a way to define a set of methods that a type must implement
    // Data types that implement the trait are called the implementors
    // Generic use traits to define the set of methods that the type must implement
    // It's similar to interface in other languages
    struct NewsArticle {
        headline: String,
        location: String,
    }
    // Trait with default implementation
    trait Summary {
        fn summarize(&self) -> String {
            String::from("Read more...")
        }
    }
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, {}", self.headline, self.location)
        }
    }
    let news = NewsArticle {
        headline: String::from("News"),
        location: String::from("Location"),
    };
    println!("Traits: {}", news.summarize());

    // Derive traits
    // Rust has a set of traits that are implemented for the standard library types
    // Provides the default implementation for the common traits
    // Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash
    // #[derive(Debug)]
    // #[derive(Clone, Copy)]
    // Compare the data type and use derive
    #[derive(PartialEq, PartialOrd)]
    struct Point4 {
        x: i32,
        y: i32,
    }
    let p1 = Point4 { x: 5, y: 10 };
    let p2 = Point4 { x: 5, y: 10 };
    println!("Derive traits: {}", p1 == p2);
    println!("Derive traits: {}", p1 > p2);

    // Trait bounds
    // Trait bounds are a way to define the set of methods that a type must implement
    fn print_type<T: fmt::Debug>(t: T) {
        println!("Trait bounds: {:?}", t);
    }
    print_type(5);
    print_type("Hello");
    print_type([9, 8, 7]);
}

fn my_function(my_string: &str) {
    println!("barrowing, {}", my_string);
}

fn my_function_mut(my_string: &mut String) {
    my_string.push_str("!");
    println!("mut ref, {}", my_string);
}

fn my_function_with_return(x: i32) -> (i32, i32) {
    (x, x + 1) // Expression
}

fn my_function_with_ownership(my_string2: String) -> String {
    println!("ownership {}", my_string2);
    my_string2
}

fn get_first_word(my_string: &str) -> &str {
    println!("Slice: {}", my_string);
    my_string
}
