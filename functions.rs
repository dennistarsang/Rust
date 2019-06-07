//hello world
fn main() {
    println!("Hello world");
}

//Passing arguments
fn print_sum(a: i8, b: i8) {
    println!("sum is: {}", a + b);
}


//Returning values

//1. Without the return keyword. Only last expression returns
fn plus_one(a: i32) -> {
    a + 1
    // There is no ending ; in the above line.
    // It means this is an expression which equals to 'return a+1;'
}
//2. With the return keyword.
fn plus_two(b: i32) -> {
    return a + 2; //Returns a+2. This is a bad practice!
    //Should use only on conditional returns, except in the last expression
}


// Functin pointers, usage as a data type

//1. Without type declarations
let b = plus_one;
let c = b(5); //6

//2. With type declarations
let b: fn(i32) -> i32 = plus_one;
let c = b(5); //6


//Closures
//Aka anonymous functions or lamda functions
//The data types of arguments and returns are optional

//Example with a named function before closures
fn main() {
    let x = 2;
    println!("{}", get_squared_value(x));   
}
fn get_squared_value(x: i32) ->i32 {
    x * x
}

//1. With optional type declarations of input and return types
fn main() {
    let x = 2;
    let square = |x: i32| -> i32 {//input parameters are passed inside || and expression body is wrapped within {}
        x * x
    };
    println!("{}", square(x));
}
//2. Without type declarations of input and return types
fn main() {
    let x = 2;
    let square = |x| x *x; // {} are optional for single-lined closures
    println!("{}", square(x));

}