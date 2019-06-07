fn main() {
    println!("Hello, world!"); //hello world
    println!("{}, {}!", "hello", "world"); //hello world
    println!("{0}, {1}!", "hello", "world");//hello world
    println!("{greeting}, {name}!", greeting="hello", name="world");//hello world

    println!("{:?}", [1,2,3] ); // [1, 2, 3]
    println!("{:#?}", [1,2,3]);
    /*
        [
            1,
            2,
            3,
        ]
     */
    // The format! macro is used to store the formatted string
    let x = format!("{}, {}!", "Hello", "world");
    println!("{}", x);

    ///This module contains tests
    mod test {
        //..
    }

}