//1
//Bool
// 💡 Returns true or false
let x = true;
let y: bool = false;

//2
//Char
//💡 A single Unicode Scalar value
let x = 'a';
let y = 'b';
// No "a", single quotes only

//3
//i8, i16, i32, i64, i128 -> 8, 16, 32, 64 and 128 bit fixed sized signed(+/-) integer types
// 💡 The min and max values are based on the following equation; from -(2ⁿ⁻¹) to 2ⁿ⁻¹-1

//4
//u8, u16, u32, u64, u128 -> 8, 16, 32, 64 and 128 bit fixed sized unsigned(0/+) integer types
//💡 The min and max values are based on the following equation; from 0 to 2ⁿ-1.

//5
//isize and usize
//Pointer sized signed types && pointer sized unsigned integer types
// 💡 the sizes are equal to 32 bit on 32-bit platforms and 64 bit on 64-bit platforms.

//6
//f32, f64
//32 and 64 bit sized floating point numbers
/*
💡 Should avoid using f32, unless you need to reduce memory consumption badly or if you are doing low-level optimization, when targeted hardware does not support for double-precision or when single-precision is faster than double-precision on it.
*/

//7
//arrays
//Fixed size list of elements of the same type
let a = [1, 2, 3]; // a[0] = 1, a[1] = 2, a[2] = 3;
let mut b = [1, 2, 3];

let c: [i32; 0] = []; //[Type, No of elements] -> [] / empty array
let d: [i32; 3] = [1, 2, 3];

let e = ["my value", 3]; //["my value", "my value", "my value"]

println!(":?", a); //[1, 2, 3]
println!(":#?", a);
// [
//   1,
//   2,
//   3
// ]
//⭐️ Arrays are immutable by default and even with mut, its element count cannot be changed.


//8
//Tuples
//Fixed size ordered list of different(or same) data types.
let a = (1, 1.5, true, 'a', "hello"); 
// a.0=1, a.1=1.5, a.2=true, a.3='a', a.4="hello"

let b: (i32, f64) = (20, 0.5);

let (c, d) = b; // c=20, d=0.5
let (e, _, _, _, f) = a; //e=1, f="hello", _indicates no interest on that item

let g = (0,); //single-element tuple

let h = (b, (2, 4), 5); //((20, 0.5), (2, 4), 5)

println!("{:?}", a); // (1, 1.5, true, 'a', "hello")
/*
⭐️ Tuples are also immutable by default and even with mut, its element count cannot be changed. Also, if you want to change an element’s value, the new value should have the same data type of previous value.
*/

//9
//Slice
//Dynamically-sized reference to another data structure
let a: [i32; 4] = [1, 2, 3, 4]; //parent array
let b: &[i32] = &a; //Slicing the whole array
let c = &a[0..4]; //From 0th position to 4th(excluding)
let d = &a[..]; //Slicing the whole array

let e = &a[1..3]; // [2, 3]
let f = &a[1..]; //[2, 3, 4]
let g = &a[..3]; //[1, 2, 3]

//10
//str
// Unsized UTF-8 sequence of Unicode string slices
let a = "hello"; //a: &static str
let b: &str = "こんにちは, 世界!";
/*
⭐️ It’s an immutable/statically allocated slice holding an unknown sized sequence of UTF-8 code points stored in somewhere in memory. &str is used to borrow and assign the whole array to the given variable binding.
*/
/*
🔎 A String is a heap-allocated string. This string is growable and is also guaranteed to be UTF-8. They are commonly created by converting from a string slice using the to_string() or String::from() methods. ex: “Hello”.to_string(); String::from("Hello");
*/
//💡 In general, you should use String when you need ownership, and &str when you just need to borrow a string.

//11
//functions
// b is a function pointer, to plus_one() in the below code.
fn plus_one(a: i32) -> i32 {
    a + 1
}

let b: fn(i32) -> i32 = plus_one;
let c = b(5); //6


