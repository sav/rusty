/// main.rs, Examples from the book "Rust Programming Language"
/// Copyright (C) 2022, Savio Machado <sav@loophole.cc>

#[cfg(not(debug_assertions))]
use rand::prelude::*;

#[cfg(not(debug_assertions))]
pub(crate) use std::cmp::Ordering;

#[cfg(not(debug_assertions))]
use std::io;

pub mod module;
use crate::module::submodule;
use crate::module::{submodule as sub, *};

fn main() {
    ex_const_1();

    ex_tuple_1();

    ex_vector_1();

    ex_iflet_1();

    ex_while_1();

    ex_for_1();

    ex_for_2();

    ex_for_3();

    ex_ownership_scope_1();

    ex_ownership_move_1();

    ex_ownership_clone_1();

    ex_ownership_copy_1();

    ex_ownership_fcall_1();

    ex_ownership_ret_1();

    ex_ownership_ret_multiple_vals();

    ex_ownership_ref_1();

    ex_ownership_mut_ref_1();

    ex_slice_1();

    ex_slice_2();

    ex_slice_3();

    ex_slice_4();

    ex_slice_5();

    ex_slice_6();

    ex_slice_7();

    ex_struct_1();

    ex_enum_1();

    ex_enum_2();

    ex_enum_3();

    ex_enum_4_options();

    ex_enum_5_match();

    ex_enum_6_value_bind();

    ex_enum_7_option_match();

    ex_enum_8();

    ex_enum_9_if_let();

    ex_enum_10_if_let();

    ex_module_1();

    ex_module_2_privacy();

    ex_collections_vector_1();

    ex_collections_multival_enum_1();

    ex_collections_string_1();

    ex_collections_string_2();

    ex_collections_string_3();

    ex_collections_hashmap_1();

    ex_collections_hashmap_2();

    ex_ownership_inventory_2();

    ex_generic_1();

    ex_generic_2();

    ex_generic_3();

    ex_generic_4();

    ex_generic_5();

    ex_trait_1();

    ex_trait_2();

    ex_trait_3();

    ex_trait_4();

    ex_lifetime_1();

    func(); // module::func

    subfunc(); // module::pfunc (since it does `pub use submodule::*`.

    submodule::subfunc();

    sub::subfunc();

    ex_guess_game();
}

#[cfg(debug_assertions)]
fn ex_guess_game() {
    println!("\nGuess game disabled in debug mode.");
}

/* Listing 9-9 */

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

/* Listing 2-6 */

#[cfg(not(debug_assertions))]
fn ex_guess_game() {
    let secret: u32 = thread_rng().gen_range(1..10);

    loop {
        let mut guess_str: String = String::new();

        println!("Your guess?");

        io::stdin()
            .read_line(&mut guess_str)
            .expect("Failed to read line");

        let guess: Guess = Guess::new(guess_str.trim().parse().unwrap());

        match guess.value().cmp(&secret) {
            Ordering::Less => println!("Bigger."),
            Ordering::Greater => println!("Smaller."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn ex_const_1() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);
}

fn ex_tuple_1() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred: i32 = x.0;
    let six_point_four: f64 = x.1;
    let one: u8 = x.2;

    println!("{}, {}, {}", five_hundred, six_point_four, one);

    let (x, y, z) = x;

    println!("{}, {}, {}", x, y, z);
}

fn ex_vector_1() -> (i32, i32) {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first: i32 = a[0];
    let second: i32 = a[1];

    println!("first = {}, second = {}", first, second);

    (first, second)
}

fn _ex_iflet_do(cond: bool) -> i8 {
    let number: i8 = if cond { 5 } else { 6 };
    number
}

fn ex_iflet_1() {
    println!("iflet(true) = {}", _ex_iflet_do(true));
    println!("iflet(false) = {}", _ex_iflet_do(false));
}

/* Listing 3-3 */

fn ex_while_1() {
    let mut n: i32 = 3;

    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }
}

/* Listing 3-4 */

fn ex_for_1() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;

    while index < 5 {
        println!("value = {}", a[index]);
        index += 1;
    }
}

/* Listing 3-5 */

fn ex_for_2() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("value = {}", element);
    }
}

/* Listing 3-6 */

fn ex_for_3() {
    for n in (1..10).rev() {
        println!("{}!", n);
    }
}

fn ex_ownership_scope_1() {
    let mut s: String = String::from("hello");

    s.push_str(", world!");
    println!("{}", s);

    /* expliciting only to exemplify.
     * this is called automatically when the scope is closed.
     */
    drop(s);
}

fn ex_ownership_move_1() {
    let s1: String = String::from("hello, world!");
    let s2: String = s1;

    /* line below is invalid since s1 was moved into s2. */
    // println!("s1 = {}", s1);
    println!("s2 = {}", s2);
}

fn ex_ownership_clone_1() {
    let s1: String = String::from("hello, world!");
    let s2: String = s1.clone();

    /* line below is valid since clone made a deep copy. */
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
}

fn ex_ownership_copy_1() {
    let x: i32 = 5;
    /* since integers have known size at compile time they
     * are stored entirely on stack. copies of the actual
     * values are quick to make so the line below actually
     * uses `copy` instead of `move`.
     */
    let y: i32 = x;

    println!("x = {}, y = {}", x, y);
}

fn _ex_ownership_take(s: String) {
    println!("s = {}", s);
} // s goes out of scope and drop(s) is called

fn _ex_ownership_mkcopy(i: i32) {
    println!("i = {}", i);
}

/* Listing 4-3 */

fn ex_ownership_fcall_1() {
    let s: String = String::from("hello, world");

    _ex_ownership_take(s);
    /* s is no longer valid here. */

    let i: i32 = 5;

    _ex_ownership_mkcopy(i);
    /* i32 is Copy so it's okay to still use i afterwards. */
}

fn _ex_ownership_give() -> String {
    let s: String = String::from("hello, world!");
    s
}

fn _ex_ownership_take_and_give_back(s: String) -> String {
    s
}

/* Listing 4-4 */

fn ex_ownership_ret_1() {
    /* transferring ownership of return values. */
    let s1: String = _ex_ownership_give();

    let s2: String = String::from("hello, world!");

    let s3: String = _ex_ownership_take_and_give_back(s2);

    println!("s1 = {}, s3 = {}", s1, s3);
}

fn _ex_ownership_calc_len(s: String) -> (String, usize) {
    let len = s.len();

    /* we *have* to return s here otherwise the ownership
     * would be lost!
     */
    (s, len)
}

/* Listing 4-5 */

fn ex_ownership_ret_multiple_vals() {
    let s1 = String::from("hello");

    /* returns the ownership of argument s1 */
    let (s2, len) = _ex_ownership_calc_len(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn _ex_ownership_calc_len_by_ref(s: &String) -> usize {
    /* The line below would fail as WE'RE NOT ALLOWED TO MODIFY
     * SOMETHING WE HAVE A (SIMPLE) REFERENCE TO.
     * s.push_str(", world!");
     *
     * For more information see "mutable reference" example below.
     */
    s.len()
}

/* References and Borrowing */

fn ex_ownership_ref_1() {
    let s1 = String::from("hello");

    let len = _ex_ownership_calc_len_by_ref(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn _ex_ownership_mut_ref_change(s: &mut String) {
    s.push_str(", world!");
}

/* Mutable References */

fn ex_ownership_mut_ref_1() {
    let mut s = String::from("hello");

    _ex_ownership_mut_ref_change(&mut s);

    println!("The length of '{}' is {}.", s, s.len());

    /* Keep in mind that you can HAVE ONLY ONE MUTABLE REFERENCE TO A
     * PARTICULAR PIECE OF DATA IN A PARTICULAR SCOPE. The lines below
     * would fail:
     *
     * let s1 = &mut s;
     * let s2 = &mut s;
     *
     * error[E0499]: cannot borrow `s` as mutable more than once at time.
     *
     * This Rust constraint prevent "data races".
     *
     * As always, you can use curly brackets to create a new scope, allowing
     * for multiple references, just not simultaneous ones:
     *
     * let mut s = String::from("hello");
     * {
     *     let r1 = &mut s;
     * }
     * let r2 = &mut s;
     *
     * A similar rule exists for combining mutable and immutable references.
     * You CANNOT HAVE A MUTABLE REFERENCE WHILE HAVING AN IMMUTABLE ONE:
     *
     * let r1 = &s;     // no problem
     * let r2 = &s;     // no problem
     * let t3 = &mut s; // BIG PROBLEM
     *
     * Users of an immutable reference do not expect the values to suddenly
     * change out from under them.
     */
}

/* Dangling References
 *
 * With the above properties Rust is able to prevent dangling pointers
 * in compile time. If, for example, we try to create a dangling pointer:
 *
 * fn main() {
 *     let ref = dangle();
 * }
 *
 * fn dangle() -> &String {
 *     let s = String::from("hello");
 *     &s // do not pass the ownership back
 * }
 *
 * We'd get:
 * error[E0106]: missing lifetime specifier
 */

/* Listing 4-7 */

fn ex_slice_1() {
    let s = String::from("hello, world!");
    let n = first_word(&s);

    println!("The length of the first word in '{}' is {}.", s, n);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    /* returns the index of the end of the first word. */
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

/* Listing 4-8 */

fn ex_slice_2() {
    let mut s = String::from("hello, world!");
    let n = first_word(&s);

    s.clear();

    /* n still has value 6 here but there's no more string that
     * we could meaningfully use the value 5 with. n is now totally
     * invalid.
     */
    println!("The length of the first word in '{}' is not {}", s, n);
}

/* String Slices */
fn ex_slice_3() {
    /* To solve the problem stated in Listing 4-8 (having indexes
     * separate from the state of a string) Rust provides a feature
     * called "string slices".
     */
    let s = String::from("hello, world!");

    let hello = &s[0..5];
    let world = &s[7..12];

    println!("{} {}!", hello, world);

    let _slice = &s[0..2];
    let _slice = &s[..2]; // same as above

    let len = s.len();

    let _slice = &s[0..len];
    let _slice = &s[..]; // same as above
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// XXX review this example. this is not exactly what's in the book.
//     according to the borrowing rule if we have an immutable
//     reference we cannot also take a mutable reference. the code
//     below creates a slice while keeping a mutable reference, how?

fn ex_slice_4() {
    let mut s1 = String::from("hello, world!");

    let s2 = first_word_slice(&s1);

    // s.clear(); // error[E0502]: cannot borrow `s` as mutable
    //            // because it is also borrowed as immutable

    println!("The first word of '{}' is '{}'", s1, s2);

    s1.clear(); // ok to call it after use.
}

fn ex_slice_5() {
    let s1: &str = "Hellow, world!"; // String literals are slices!

    /* This is why string literals are immutable; &str is an immutable
     * reference.
     */
    println!("{}", s1);
}

/* Function implementation does not change. only signature. */
fn first_word_advanced(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}

/* Listing 4-9 */

fn ex_slice_6() {
    let s = String::from("hello, world!");

    let word = first_word_advanced(&s[..]);

    println!("First word of '{}' is '{}'.", s, word);

    let l = "hello, world!";

    let word = first_word_advanced(l);

    println!("First word of '{}' is '{}'.", l, word);
}

/* Other Slices */

fn ex_slice_7() {
    let a1 = [1, 2, 3, 4, 5];

    let a2 = &a1[1..3];

    println!("a1 = [1, 2, 3, 4, 5]; a2 = a1[1..3]; a2[0] = {};", a2[0]);
}

/* Structs */

#[derive(Debug)]
struct Rectangle1 {
    width: u32,
    height: u32,
}

impl Rectangle1 {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle1 {
    fn can_hold(&self, other: &Rectangle1) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle1 {
    fn new(width: u32, height: u32) -> Rectangle1 {
        Rectangle1 { width, height }
    }

    fn square(size: u32) -> Rectangle1 {
        Rectangle1 {
            width: size,
            height: size,
        }
    }
}

fn ex_struct_1() {
    let rect1 = Rectangle1 {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle1::new(10, 20);
    let rect3 = Rectangle1::square(100);

    println!(">> rect1 = {:?}", rect1);
    println!("The area of the rect1 is {}", rect1.area());
    println!("rect1 can hold rect2 -> {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3 -> {}", rect1.can_hold(&rect3));

    println!(">> rect2 = {:?}", rect2);
    println!("The area of the rect2 is {}", rect2.area());
    println!("rect2 can hold rect1 -> {}", rect2.can_hold(&rect1));
    println!("rect2 can hold rect3 -> {}", rect2.can_hold(&rect3));

    println!(">> rect3 = {:?}", rect3);
    println!("The area of the rect3 is {}", rect3.area());
    println!("rect3 can hold rect1 -> {}", rect3.can_hold(&rect1));
    println!("rect3 can hold rect2 -> {}", rect3.can_hold(&rect2));
}

/* Enums */

#[derive(Debug)]
enum IpAddr1 {
    V4,
    V6,
}

fn route(_vers: IpAddr1) {}

struct IpAddr1Struct {
    version: IpAddr1,
    address: String,
}

fn ex_enum_1() {
    let v4 = IpAddr1::V4;
    let v6 = IpAddr1::V6;

    route(v4);
    route(v6);

    route(IpAddr1::V4);
    route(IpAddr1::V6);

    let home = IpAddr1Struct {
        version: IpAddr1::V4,
        address: String::from("127.0.0.1"),
    };

    println!(">> {:?}, {}", home.version, home.address);

    let loopback = IpAddr1Struct {
        version: IpAddr1::V6,
        address: String::from("::1"),
    };

    println!(">> {:?}, {}", loopback.version, loopback.address);
}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn ex_enum_2() {
    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));

    println!(">> home = {:?}", home);
    println!(">> loopback = {:?}", loopback);
}

#[derive(Debug)]
enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    _ChangeColor(i32, i32, i32),
    Write(String),
}

impl Message {
    fn call(&self) {
        println!("message: {:?}", self);
    }
}

fn ex_enum_3() {
    let m = Message::Write(String::from("hello, world!"));
    m.call();
}

/* The Option Enum */
fn ex_enum_4_options() {
    /* We can use Option, Some and None directly as they are
     * always implicity loaded in the prelude.
     */
    let _n1 = Some(5);
    let _s1 = Some("string");
    let _z1: Option<i32> = None;

    /* The type Option<T> is defined in the standard library:
     *
     * enum Option<T> {
     *   Some(T),
     *   None,
     * }
     *
     * Why is having Option<T> any better than having null?
     * In short, because Option<T> and T are different types.
     *
     * For example, the code below won't compile because it's
     * trying to add an i8 to an Option<i8>:
     *
     * let x: i8 = 5;
     * let y: Option<i8> = Some(5);
     * let sum = x + y; // no implementation for `i8 + Option<i8>`
     *
     * In other words, you have to convert an Option<T> to a T
     * before you can perform T operations with it. That helps
     * catch one of the most common issues with null: assuming
     * that something isn't null when it actually is.
     */
}

/* The match Control Flow Operator */
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

/* Listing 6-3 */
fn in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn ex_enum_5_match() {
    println!("p: {}", in_cents(Coin::Penny));
    println!("n: {}", in_cents(Coin::Nickel));
    println!("d: {}", in_cents(Coin::Dime));
    println!("q: {}", in_cents(Coin::Quarter));
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    /* --snip-- */
}

/* Listing 6-4 */

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // now holds a UsState value
}

fn in_cents2(coin: Coin2) -> u32 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State Quarter from {:?}.", state);
            25
        }
    }
}

fn ex_enum_6_value_bind() {
    println!("p: {}", in_cents2(Coin2::Penny));
    println!("n: {}", in_cents2(Coin2::Nickel));
    println!("d: {}", in_cents2(Coin2::Dime));
    println!("q(alaba~): {}", in_cents2(Coin2::Quarter(UsState::Alabama)));
    println!("q(alaska): {}", in_cents2(Coin2::Quarter(UsState::Alaska)));
}

/* Matching with Option<T> */

/* Listing 6-5 */

fn plus_one(x: Option<i32>) -> Option<i32> {
    x.map(|i| i + 1)
}

fn ex_enum_7_option_match() {
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}

/* The _ Placeholder */

fn ex_enum_8() {
    let some_value = 0u8;
    match some_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        _ => (), // will match any value
    }
}

/* Concise Control Flow with if let */

fn ex_enum_9_if_let() {
    let some_value = Some(3);

    /* bad: */
    match some_value {
        Some(3) => println!("three"),
        Some(5) => println!("five"),
        _ => (),
    }

    /* better: */
    if let Some(3) = some_value {
        println!("three");
    } // eventually: // else { (); }
}

fn ex_enum_10_if_let() {
    let mut unknown = 0;
    let coin = Coin2::Quarter(UsState::Alaska);

    if let Coin2::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        unknown += 1;
    }

    if unknown > 0 {
        println!("unknown count = {}", unknown);
    }
}

mod module_1 {
    pub fn f1() {
        println!("module_1::f1() => hello, world!");
    }
}

pub mod network;

fn ex_module_1() {
    module_1::f1();
    network::connect();
}

/* Privacy Rules
 *
 * 1. If an item is public, it can be accessed through any of its
 *    parent modules.
 *
 * 2. If an item is private, it can be accessed only by its immediate
 *    parent module and any of the parent's child modules.
 */

/* Listing 7-6 (Fixed) */

pub mod outermost {
    pub fn middle_function() {
        middle_secret_function();
    }

    fn middle_secret_function() {
        inside::inner_function();
    }

    mod inside {
        pub fn inner_function() {
            secret_function();
        }

        fn secret_function() {
            println!("outermost::inside::secret_function()");
        }
    }
}

fn ex_module_2_privacy() {
    use outermost::middle_function;
    middle_function();
}

fn ex_collections_vector_1() {
    let mut v1: Vec<i32> = vec![1, 2, 3, 4];
    println!("v1 = {:?}", v1);

    let mut v2 = vec![1, 2, 3];
    v2.push(4);
    println!("v2 = {:?}", v2);

    let v1_1: &i32 = &v1[0];
    let v2_1: &i32 = &v2[0];

    assert_eq!(v1_1, v2_1);

    /* &v1[100]; <- panics! */
    assert_eq!(v1.get(100), None); // doesn't panic!

    for i in &v1 {
        println!("+ {}", i);
    }

    for i in &mut v1 {
        *i += 50;
        println!("+ {}", *i);
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn ex_collections_multival_enum_1() {
    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}

/* Storing UTF-8 Encoded Text with Strings
 *
 * String can be tricky for a few reasons:
 *   1. Rust propensity for exposing possible errors.
 *   2. String being a more complicated date structure
 *      than most programmers give them credit for.
 *   3. Finally, because UTF-8 can cause confusion.
 */

fn ex_collections_string_1() {
    let mut _s: String = String::new();

    let data = "initial contents";
    let _s = data.to_string();

    let _s = "initial contents".to_string();

    let mut s = String::from("य एनं वेत्ति हन्तारं यश्चैनं मन्यते हतम्।\n");
    s += "उभौ तौ न विजानीतो नायं हन्ति न हन्यते॥ २-१९\n";
    println!("\n{}", s);

    s.clear();

    s.push_str("न जायते म्रियते वा कदाचिन्\n");
    s.push_str("नायं भूत्वा भविता वा न भूयः।\n");
    s.push_str("अजो नित्यः शाश्वतोऽयं पुराणो\n");
    s.push_str("न हन्यते हन्यमाने शरीरे॥ २-२०\n");
    println!("{}", s);

    /* Listing 8-16 */
    let mut s1 = String::from("foo");
    let s2 = "bar";

    s1.push_str(s2);
    println!("s2 is {}", s2);

    /* Listing 8-17 */
    let mut s = String::from("lo");
    s.push('l'); // single character
    println!("s is {}", s);

    /* Listing 8-18 */
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    /* Given that: fn add(self, s: &str) -> String {...},
     * s1 is moved and can no longer be used after the
     * line below. Also, &s2 is a &String coerced into &str.
     */
    let s3 = s1 + &s2;

    println!("s3 is {}", s3);
}

fn ex_collections_string_2() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);
}

use unicode_segmentation::UnicodeSegmentation;

fn ex_collections_string_3() {
    /* A String is a wrapper over Vec<u8> */
    let len = String::from("Hola").len();
    assert_eq!(len, 4);

    let s = "नजायतेम्रियतेवाकदाचिन्";
    println!("len('{}') = {}", s, s.len());

    /* Invalid code: let a = &s[0]; */

    let s1 = &s[0..6]; /* You can slice it...
                        * but besides being hard to tell what's the outcome,
                        * the program can also crash in runtime if the slice
                        * can't be made, for example, if '0..n' does not
                        * comprehend a whole char, as 0..5 here doesn't.
                        */
    println!("{}", s1);

    /* There are three ways to look at strings from Rust's perspective:
     *   1. Bytes
     *   2. Scalar Values
     *   3. Grapheme Clusters (the closest thing we would call "letters")
     */
    println!(">> chars({})", s);
    for c in s.chars() {
        println!(">>\t{}", c);
    }

    println!(">> bytes({})", s);
    for b in s.bytes() {
        println!(">>\t{}", b);
    }

    println!(">> graphemes({})", s);
    for g in UnicodeSegmentation::graphemes(s, true) {
        println!(">> \t{}", g);
    }
}

use std::collections::HashMap;

fn ex_collections_hashmap_1() {
    /* Listing 8-20 */
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    /* Listing 8-21 */

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let _scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    /* Listing 8-22 */

    /* For types that implement the Copy trait, such as i32, the values
     * are copied into the hash map. For owned values such as String the
     * values will be moved and the hash map will be the owner of those
     * values, as demonstrated below.
     */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    /* field_name and field_value are invalid at this point. */

    /* Listing 8-23 */

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score: Option<&u32> = scores.get(&team_name);

    assert_eq!(score, Some(&10));

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    /* Listing 8-24 */

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    /* Listing 8-25 */

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    /* entry() adds iif the value is not present in the mapping */
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    /* Listing 8-26 */

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count: &mut u32 = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn ex_collections_hashmap_2() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score}");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // overwriting a value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // adding a key and value only if a key isn't present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn ex_ownership_inventory_2() {
    fn remove_zeroes_inplace(v: &mut Vec<i32>) {
        for i in (0..v.len()).rev() {
            if v[i] == 0 {
                v.remove(i);
                v.shrink_to_fit();
            }
        }
    }
    let mut v1 = vec![0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0];
    remove_zeroes_inplace(&mut v1);
    println!("{v1:?}");

    let v1 = vec![1; 100];
    println!("{v1:?}");

    let mut v1 = vec![5, 5, 0];
    println!("{v1:?}");

    fn lennfst(v: &mut Vec<i32>) {
        let n = v.len();
        let t = &mut v[0];
        println!("{n} {t}"); // can't add {v:?} here.
        println!("{v:?}");
    }

    lennfst(&mut v1);
}

/* Listing 10-4 and Listing 10-15 */

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn ex_generic_1() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest number is {}", result);
}

struct Point1<T> {
    x: T,
    y: T,
}

fn ex_generic_2() {
    let integer = Point1 { x: 5, y: 10 };
    let float = Point1 { x: 1.0, y: 4.0 };

    println!(
        "integer = ({}, {}) , float = ({}, {})",
        integer.x, integer.y, float.x, float.y
    );
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn ex_generic_3() {
    let int_and_float = Point2 { x: 1, y: 1.5 };

    println!("int_and_float = ({}, {})", int_and_float.x, int_and_float.y);
}

/* Listing 10-9 */

struct Point3<T> {
    x: T,
    y: T,
}

impl<T> Point3<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

/* Listing 10-10 */

impl Point3<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn ex_generic_4() {
    let p = Point3 { x: 5, y: 10 };

    println!("p = ({}, {})", p.x(), p.y());

    let p = Point3 { x: 1.0, y: 1.0 };

    println!(
        "p = ({}, {}), distance_from_origin = {}",
        p.x(),
        p.y(),
        p.distance_from_origin()
    );
}

/* Listing 10-11 */

struct Point4<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point4<T, U> {
    fn mixup<V, W>(self, other: Point4<V, W>) -> Point4<T, W> {
        Point4 {
            x: self.x,
            y: other.y,
        }
    }
}

fn ex_generic_5() {
    let p1 = Point4 { x: 5, y: 10.4 };
    let p2 = Point4 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3 = ({}, {})", p3.x, p3.y)
}

/* Listing 10-12 */

pub trait Summary {
    fn summarize(&self) -> String;
}

/* Listing 10-13 */

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn ex_trait_1() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

/* Listing 10-14 */

pub trait DefaultSummary {
    fn default_summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl DefaultSummary for NewsArticle {}

fn ex_trait_2() {
    let article = NewsArticle {
        headline: String::from("Penguis win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
hockey team in the NHL",
        ),
    };

    println!("New article available! {}", article.default_summarize());
}

/*
 * fn some_function<T: Display + Clone, U: Cllone + Debug>(t: T, u: U) -> i32 {
 *
 * can be written as:
 *
 * fn some_function<T, U>(t: T, u: U) -> i32
 *   where T: Display + Clone,
 *         U: Clone + Debug
 * {
 */

pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

fn ex_trait_3() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    notify(tweet);
}

/* Listing 10-16: Conditionally implement methods on a generic type. */

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else if self.x < self.y {
            println!("The largest member is y = {}", self.y);
        } else {
            println!("The members x and y are equal = {}", self.x);
        }
    }
}

fn ex_trait_4() {
    let p = Pair::new(1, 2);
    p.cmp_display();

    let p = Pair::new(2, 1);
    p.cmp_display();

    let p = Pair::new(2, 2);
    p.cmp_display();
}

fn ex_lifetime_1() {
    let s1: &str = "xpto__";
    let s2: &str = "xpto___";

    println!("xpto_a = {}", s1.len());
    println!("xpto_b = {}", s2.len());
}
