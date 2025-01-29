mod primitives_literals;
mod primitives_tuples;

//  https://doc.rust-lang.org/rust-by-example/primitives.html
//  (variables are prefixed with _ because warnings clutter the screen otherwise)
fn main() {
    // variables can be type annotated
    let _logical: bool = true;

    let _a_float: f64 = 1.0; // regular annotation
    let _an_integer = 5i32; // Suffix annotation
    
    // or a default will be used
    let _default_float = 3.0; // f64
    let _default_integer = 7; // i32

    // a type can also be inferred from context
    let mut _inferred_type = 12; // i64 is inferred from the next line
    _inferred_type = 4294967296i64;

    // a mutable variable's value can be changed
    let mut _mutable = 12;
    _mutable = 21;

    // Error! type cannot be changed
    // mutable = true;

    // variables can be overwritten by shadowing
    let _mutable = true;

    // Compound types - arrays and tuples

    // Array signature consists of Type T and length as [T; length]
    let _my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // Tuple is a collection of values of different types
    // and is constructed using parentheses ().
    let _my_tuple = (5u32, 1u8, true, -5.04f32);

    primitives_literals::main();
    primitives_tuples::main();
}
