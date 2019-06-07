//Arithmetic operators
fn main() {
    //Arithmetic operators
    let a = 5;
    let b = a + 1; //6
    let c = a - 1; //4
    let d = a * 2; //10
    let e = a / 2; //2 not 2.5 because it's an integer unless specified otherwise
    let f = a % 2; //1
    let g = 5.0/2.0; //2.5

    println!("{}, {}, {}, {}, {}, {}, {}", a, b, c, d, e, f, g);

    // Comparison operators
    let a = 1;
    let b = 2;

    let c = a == b; //false
    let d = a != b; //true
    let e = a < b; //true
    let f = a > b; //false
    let g = a <= a; //true

    let i = true > false; //true
    let j = 'a' > 'A'; //true

    println!("{}, {}, {}, {}, {}, {}, {}, {}, {}", a, b, c, d, e, f, g, i, j);

    //Logical operators
    let a = true;
    let b = false;

    let c = !a; //false
    let d = a && b; //false
    let e = a || b; //true
    //🔎 On integer types,! inverts the individual bits in the two’s complement representation of the value.
    let a = !-2; //1
    let b = !-1; //0
    let c = !0; //-1
    let d = !1; //-2

    //Bitwise operators
    let a = 1;
    let b = 2;

    let c = a & b; //0  (01 && 10 -> 00)
    let d = a | b; //3  (01 || 10 -> 11)
    let e = a ^ b; //3  (01 != 10 -> 11)
    let f = a << b; //4  (Add b number of 0s to the end of a -> '01'+'00' -> 100)
    let g = a >> b; //0  (Remove b number of bits from the end of a -> o̶1̶ -> 0)

    //Assignment and Compound assignment operators
    let mut a = 2;

    a += 5; //2 +5 = 7
    a -= 2; //7 - 1 = 5
    a *= 5; //5 * 5 = 25
    a /= 2; //25 / 2 = 12
    a % 5 = 2; // 12 % 5 = 2

    a &= 2; // 10 && 10 -> 10 -> 2
    a |= 5; // 010 || 101 -> 111 -> 7
    a ^= 2; // 111 != 010 -> 101 -> 5
    a <<= 1; // 101 add '0' bit to the right -> 1010 -> 10
    a >>= 2; // 1010 remove 2 bits from the right -> 10 -> 2

    //Type casting operator
    let a = 15;
    let b = (a as f64) / 2.0; //7.5

    //Borrowing and deference operators
    //The & or &mut operators are used for borrowing and * operator for Dereferencing.

    }
