// misc1.rs

// further examples form `the rust programming language`, but from this new
// interactive version with quizzes: https://rust-book.cs.brown.edu

use core::slice::*;
use std::any::type_name;
use std::fmt;
use std::ops::*;

fn type_of<T>(_: &T) -> String {
    type_name::<T>().to_string()
}

fn tuple() {
    let t = ([1; 2], [3; 4]);
    println!("{}", t.0[0]);

    let x = [1, 5];
    println!("{}", x.len());

    let y = [1; 5];
    println!("{}", y.len());
}

fn fmt(x: i8) {
    println!("{x}");
}

fn cond(f: bool) {
    let x = if f { 0 } else { 1 };
    println!("{}", x);
}

fn lop() {
    let mut x = 0;
    'a: loop {
        x += 1;
        'b: loop {
            if x > 10 {
                continue 'a;
            } else {
                break 'b;
            }
        }
        break;
    }
    println!("{x}");
}

fn iter() {
    let a = [5; 10];
    let mut sum = 0;

    for x in a {
        sum += x;
    }

    println!("{sum}");
    println!("");

    let b = [5, 10];
    let mut sum = 0;

    for x in b {
        sum += x;
    }

    println!("{sum}");
    println!("");

    for n in (1..4).rev() {
        println!("{n}")
    }
}

fn arr() {
    let a = [0; 1_000_000];
    let mut b = a;
    b[0] = 1;
    println!("{} {}", b.len(), a[0]);
}

fn b0x() {
    let a = Box::new([0; 1_000_000]);
    let b = a;
    println!("{}", b.len());
}

fn suf(mut s: String) -> String {
    s.push_str(" 1");
    s
}

fn refptr() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x; // *x reads the heap value, so a = 1
    *x += 1; // *x on the left-side modifies the heap value, so x points to the value 2

    let r1: &Box<i32> = &x; // r1 points to x on the stack
    let b: i32 = **r1; // two dereferences get us to the heap value

    let r2: &i32 = &*x; // r2 points to the heap value directly
    let c: i32 = *r2; // so only one dereference is needed to read it

    println!("{}, {}, {}, {}, {}, {}", x, a, r1, b, r2, c);

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs(); // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs(); // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len(); // implicit reference
    assert_eq!(s_len1, s_len2);
}

fn pfmt() {
    let x: Box<u32> = Box::new(u32::MAX);
    assert_eq!(u32::MAX, ((1u64 << 32) - 1) as u32);
    println!("{:p} <{}> -> 0x{:x}", x, type_of(&x), *x);
    let y = &*x;
    println!("{:p} <{}> -> 0x{:x}", y, type_of(&y), *y);
}

fn ptr1() {
    let v = vec![1, 2, 3];
    let v2 = &v;

    println!(
        "&v = {:p},  &v[0] = {:p} ({}),  v[0] = {}",
        &v,
        &v[0],
        type_of(&(&v[0])),
        v[0]
    );

    println!(
        "v2 = {:p}, &v2[0] = {:p} ({}), v2[0] = {}",
        v2,
        &v2[0],
        type_of(&(&v[0])),
        v2[0]
    );
}

// The core idea behind the borrow checker is that variables have three kinds of
// permissions on their data:
//
//  - Read (R): data can be copied to another location.
//  - Write (W): data can be mutated in place.
//  - Own (O): data can be moved or dropped.
//
// These permissions don't exist at runtime, only within the compiler.
//
// By default variables have RO permissions on its data. Whenever a variable is
// annotated with `let mut`, then it also has the W permission.
//
// The key idea is that references can temporarily remove these permissions.
//
//  + Shared reference = immutable.
//  + Unique reference = mutable.
//
// Unique references allows mutation but prevents aliasing. Whenever a variable
// is referenced with mut it loses ALL its permission.
//
// A reference changes permissions while it is "in use".
// Permissions are returned at the end of a reference's lifetime.
//
// As a part of the Pointer Safety Principle, the borrow checker enforces that
// data must *outlive* any references to it. Rust enforces this property in two ways.
//
//  + Within the scope of a single function (RWO permissions)
//  + When references are used as either input or output to functions (+F permission).
//
// Unlike the RWO permissions, the F permissions does not change throughout the body
// of a function.
//
// References provide the ability to read and write data without consuming ownership of it.
// References are created with borrows (& and &mut) and used with dereferences (*),
// often implicitly.
//
// However, references can be easily misused. Rust's borrow checker enforces a system of
// permissions that ensures references are used safely:
//
//  + All variables can read, own, and (optionally) write their data.
//  + Creating a reference will transfer permissions from the borrowed path to the reference.
//  + Permissions are returned once the reference's lifetime has ended.
//  + Data must outlive all references that point to it.

fn own1() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    // v[0] = 1; // can't do.
    println!("{}", num);
    v[0] = 1; // can do.
}

fn own2() {
    let x = 0;
    let mut x_ref = &x; // x_ref has +W while *x_ref does not.

    // that means, we can assign x_ref to a different reference, but we cannot
    // mutate the pointed data (eg: *x_ref += 1).

    println!("{}", x_ref);

    // More generally, permissions are defined on paths and not just variables.
    // A path is anything you can put on the left-hand side of an assignment.
    // Paths include:
    //
    // - Variables, like a.
    // - Dereferences of paths, like *a.
    // - Array accesses of paths, like a[0].
    // - Fields of paths, like a.0 for tuples or a.field for structs (discussed next chapter).
    // - Any combination of the above, like *((*a)[0].1).

    let x2 = 1;
    x_ref = &x2;

    println!("{}", x_ref);
}

fn own3() {
    let mut x = 1;
    let p: &mut i32 = &mut x;
    // let q = &mut x; // cannot borrow `x` as mutable more than once at a time.
    println!("{}", p);

    let y = 1;
    let p: &i32 = &y;
    let q: &i32 = &y; // ok to alias as reference is immutable.
    println!("{:X} {:X}", p, q);
}

fn own4() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &/*R*/v[2]; // borrow. requires v to be readable (R).
                                // v./*W*/push(4); // requires v to be readable (R) and writable (W),
                                //                  // but v is not writable anymore.
    println!("{}", *num);
    v.push(4); // v here gets its W permission back after num is unused.
}

fn own5() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    // when creating a mutable reference, ALL PERSMISSIONS are moved, thus
    // `v` becomes unreadable also at this point.
    let num: &mut i32 = &mut v[2];

    *num += 1;

    println!("third element is {}", *num);
    println!("vector is now {v:?}");
}

fn own6() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    let num2: &i32 = &*num; // this borrow removes the W permission from *num, but not the R.
    println!("{} {}", *num, *num2); // so println!() works on both.
}

fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        v[0] = up;
    } else {
        println!("{v:?}");
    }
}

fn own7() {
    let mut v: Vec<char> = vec!['a', 'b', 'c'];
    ascii_capitalize(&mut v);
    println!("{v:?}");
}

fn own8() {
    let s = String::from("hello");
    let s_ref = &s;

    // cannot move out of `s` because it's borrowed.
    // drop(s);

    println!("{}", s_ref);
}

fn first(strings: &Vec<String>) -> &String {
    let s_ref = &strings[0];
    /* RF permissions */
    s_ref
}

fn own9() {
    let mut v = vec![];
    v.push(String::from("A"));
    v.push(String::from("B"));
    v.push(String::from("C"));
    let p = first(&v);
    println!("{p:?}");
}

fn move1() {
    let orig_str = String::from("hello");
    let new_str = orig_str;

    // compilation error:
    // println!("orig_str = {}", orig_str);

    println!("new_str = {} ({})", new_str, type_of(&new_str));

    let orig_vec = vec![1, 2, 3];
    let new_vec = orig_vec;

    println!("new_str = {:?} ({})", new_vec, type_of(&new_vec));
}

fn move2() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{:p} <{}> {}", s2.as_ptr(), type_of(&s2), s2);
}

// It is very rare for Rust functions to take ownership of heap-owning
// data structures like Vec and String.
//
// The function below takes the ownership of the variable 'name'.
// This action renders `name' unusable after the call, which can be
// annoying for the caller.

fn stringify_name_with_title_1(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

fn move3() {
    let name = vec![String::from("sav"), String::from("io")];
    println!("{}", stringify_name_with_title_1(name));
    // println!("{}", name[0]); // can't do. name was moved into the above call.
}

fn stringify_name_with_title_2(name: &Vec<String>) -> String {
    // to circumvent the ownership problem described above usually
    // we can clone the input argument.
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}

fn move4() {
    let name = vec![String::from("sav"), String::from("io")];
    println!("{}", stringify_name_with_title_2(&name));
    println!("{}", name.len());
}

fn stringify_name_with_title_3(name: &Vec<String>) -> String {
    // the best solution though is simply to add the suffix later to
    // a new variable.
    let mut full = name.join(" "); // slice.join copies `name` into `full`.
    full.push_str(" Esq.");
    full
}

// In general, writing Rust functions is a careful balance of asking
// for the right level of permissions. For this example, it's most idiomatic
// to only expect the read permission on name.

fn move5() {
    let name = vec![String::from("sav"), String::from("io")];
    println!("{}", stringify_name_with_title_3(&name));
    println!("{}", name.len());
}

fn static1() -> &'static str {
    "hello"
}

// Reference-counted smart-pointer.
// At runtime, the Rc checks when the last Rc pointing to data has
// been dropped, and then deallocates the data.

use std::rc::Rc;

fn rc1() -> Rc<String> {
    let s = Rc::new(String::from("hello"));
    Rc::clone(&s) // only clones a pointer to s and not the data itself.
}

fn slot1rep(output: &mut String) {
    output.replace_range(.., "hello");
}

fn slot1() {
    let mut s = String::from("hello");
    slot1rep(&mut s);
    println!("{}", s);
}

// If a value does not own heap data, then it can be copied without a move.
//  - An i32 does not own heap data, so it can be copied without a move.
//  - A String does own heap data, so it cannot be copied without a move.
//  - A &String does not own heap data, so it can be copied without a move.
//
// One exception is mutable references. For example, `&mut i32' is not copyable.
//   let mut n = 0;
//   let a = &mut n;
//   let b = a;
// Then `a' cannot be used after being assigned to `b'. That prevents two mutable
// references to the same data from being used at the same time.

fn collection1() {
    let vi: Vec<i32> = vec![0, 1, 2];
    let vie: &i32 = &vi[0];
    let n: i32 = *vie;
    println!("{}", n);

    let vs: Vec<String> = vec![String::from("hello")];
    let vse: &String = &vs[0];
    // let s: String = *vse; // Can't move, because references are non-owning pointers.
    // Can't copy because 'vs' owns heap data (does not implement Copy trait).

    println!("{}", vse);
}

// Safe versions.

fn collection2() {
    let v: Vec<String> = vec![String::from("hello")];
    let s_ref: &String = &v[0];
    println!("{s_ref}");
}

fn collection3() {
    let v: Vec<String> = vec![String::from("hello")];
    let mut s: String = v[0].clone();
    s.push('!');
    println!("{s}");
}

fn collection4() {
    let mut v: Vec<String> = vec![String::from("hello")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);
}

fn cols() {
    collection1();
    collection2();
    collection3();
    collection4();
}

fn safetuple1() {
    let mut name = (String::from("hello"), String::from("world"));

    let first = &name.0; // removes WO from name.0, and name (cannot be passed to
                         // a function that takes (String, String) for example)
    name.1.push_str("!"); // but name.1 still retains the W permission.
    println!("{first} {}", name.1);
}

fn safearray1() {
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1]; // Rust does not contain different paths for a[0], a[1], and so on.
                       // It uses a single path a[_] that presents all indexes of a. Rust does this because
                       // it cannot always determine the value of an index. For example, a more complex scenario:
                       //
                       //   let idx = a_complex_function();
                       //   let x = &mut a[idx];

    *x += 1;
    println!("{a:?}");
}

fn safearray2() {
    let mut a = [0, 1, 2, 3];
    let (a_l, a_r) = a.split_at_mut(2);

    let x = &mut a_l[1];
    let y = &a_r[0];
    *x += *y;
}

// you might wonder how is how split_at_mut implemented? in some rust libs, especially
// core types like Vec or slice, you'll often find `unsafe' blocks.
// `unsafe' blocks allow the use of "raw" pointers, which are not checked for safety by the borrow
// checker. for example, we could use an unsafe block to accomplish our task:

fn safearray3() {
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1] as *mut i32;
    let y = &a[2] as *const i32;
    unsafe {
        *x += *y;
    } // DO NOT DO THIS unless you know what you're doing!
    println!("{x:p}, {y:p}");
}

fn fixsafe() {
    safetuple1();
    safearray1();
    safearray2();
    safearray3();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn find_contains(heystack: &[String], needle: &str) -> Vec<String> {
    let mut r = vec![];
    for item in heystack.iter() {
        if item.contains(needle) {
            r.push(item.clone());
        }
    }
    r
}

fn slice1() {
    let s = String::from("hello world");
    println!("{}", first_word(&s));

    let a = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    let v = [
        String::from("foo"),
        String::from("bar"),
        String::from("far"),
        String::from("boo"),
    ];
    let w = find_contains(&v, "f");
    for item in w.iter() {
        println!("{}", item);
    }
}

fn slice2() {
    println!("{:?}", &[123]); // array
    println!("{:?}", &[123][..]); // slice

    println!("{:p}", &[123]);
    println!("{:p}", &[123][..]);

    println!("{}", type_of(&[123]));
    println!("{}", type_of(&(&[123][..])));
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn slice3() {
    let l1 = [1, 2, 3, 4, 100];
    println!("{}", largest(&l1));

    let l2 = [92, 34, 100, 89, 54, 2, 43, 8];
    println!("{}", largest(&l2));
}

fn tuplestruct1() {
    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);
    println!("{} {} {}", black.0, black.1, black.2);
}

fn unitstruct1() {
    #[derive(Debug)]
    struct AlwaysEqual;

    let s = AlwaysEqual;
    println!("{:#?}", s);
    dbg!(s);
}

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    // Self is an alias for the `impl' type.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn assocfunc1() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "30x50 area = {} ({})",
        rect1.area(),
        Rectangle::area(&rect1)
    );
    println!("50x50 area = {}", Rectangle::square(50).area());

    impl Rectangle {
        fn foo() {
            println!("foo");
        }
    }
    Rectangle::foo();

    struct Triangle(i32, i32);
    let t = Triangle(10, 20);
    impl Triangle {
        fn a(&self) -> i32 {
            // Notice the syntax `&self'.
            (self.0 * self.1) / 2
        }
    }
    println!("{}, {} => {}", t.0, t.1, Triangle::a(&t));
}

impl Rectangle {
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // self, instead of &self, intentionally getting the ownership.
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

fn assocfunc2() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };
    println!("{}", rect.area());

    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };

    let mut max_rect = rect.max(other_rect);
    println!("max = {}", max_rect.area());

    max_rect.set_width(10);
    println!("max = {}", max_rect.area());
}

fn assocfunc3() {
    // derive Copy and Clone.
    impl Rectangle {
        fn set_to_max(&mut self, other: Self) {
            *self = self.max(other); // exists for mutable reference.
                                     // Requires #[derive(Copy, Clone)] struct Rectangle {}.

            // Notice that unlike before, self.max(other) no longer requires the O permission
            // on *self or other. Remember that self.max(other) desugars to Rectangle::max(*self, other).
            // The dereference *self does not require ownership over *self if Rectangle is copyable.
            //
            // Rust doesn't automatically derive Copy for Rectangle for stability across API changes.
            // Image the author of Rectangle type decided to add a `name: String' field. All client code
            // that relies on Rectangle being Copy would suddenly be rejected by the compiler. To avoid
            // that issue, API authors must explicitly add #[derive(Copy)] to indicate that they expect
            // their struct to always be Copy.
            //
            // To illustrate, imagine we add a `name: String' to Rectangle and implement `set_to_max' as:
            //
            // let max = self.max(other)
            // drop(*self); // usually implicit, added for clarity.
            // *self = max;
            //
            // After calling self.max(), the max() method consumes ownership of both rectangles. When
            // max() returns, Rust deallocates both strings "self" and "other" in the heap. Notice the
            // problem: right before `*self = max', `*self' is supposed to be readable and writable.
            // However, `(*self).name' has been deallocated, so when we do the assignment we encounter
            // an undefined behavior. When overwriting *self Rust will implicitly drop the data
            // previously in *self (illustrated by `drop(*self)'.
        }
    }
    let mut rect4 = Rectangle {
        width: 0,
        height: 1,
    };
    let rect5 = Rectangle {
        width: 1,
        height: 0,
    };
    rect4.set_to_max(rect5);
}

fn mutref1() {
    struct Point(i32, i32);
    impl Point {
        fn x(&self) -> i32 {
            self.0
        }
    }
    let mut p: Point = Point(1, 2);
    let r: &mut Point = &mut p;
    println!("{} {}", r.x(), r.1); // ok to call r.x(). &mut is implicitly converted to &.
}

// Dereferencing a pointer access its data!
//
// Rust will insert as many references and dereferences as needed to make the types
// match up for the self parameter.

fn deref1() {
    let r = &mut Box::new(Rectangle {
        width: 1,
        height: 2,
    });
    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    let area3 = Rectangle::area(&*r);
    let area4 = Rectangle::area(r);
    assert_eq!(area1, area2);
    println!("{}, {}, {}, {}", area1, area2, area3, area4);

    // an old example to jog your memory...
    let x = Box::new(0);
    let y = Box::new(&x);
    println!("{}", *y);
    println!("{}", ***y);
}

enum MyEnum {
    MyField(i32),
}

impl fmt::Debug for MyEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyEnum::MyField(val) => write!(f, "MyField({})", val),
        }
    }
}

fn enum1() {
    let c1: MyEnum = MyEnum::MyField(1);
    println!("{:#?}", c1);
}

fn option1() {
    let mut x = Some(5);
    println!("{} {}", x.is_some(), x.unwrap());

    x = None;
    println!("{} {}", x.is_none(), x.unwrap_or(-1));

    let text: Option<String> = Some("hello world".to_string());
    let text_len: Option<usize> = text.as_ref().map(|s| s.len());
    println!("{} {}", text.unwrap(), text_len.unwrap());

    x = Some(5);
    match x.as_mut() {
        Some(v) => *v = 100,
        None => {}
    }
    assert_eq!(x, Some(100));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => todo!(), // indicates unfinished code.
        Some(i) => Some(i + 1),
    }
}

fn option2() {
    let five = Some(5);
    let six = plus_one(five);
    println!("{}", six.unwrap());
}

fn option3() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(x: u8) {
        println!("{x}");
    }
}

fn option4() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {
        println!("reroll");
    }
}

fn option5() {
    let s: Option<String> = Some(String::from("hello world"));
    match s {
        Some(_) => println!("Some"), // replacing Some(_) with Some(s) will crash the compiler.
        None => println!("None"),
    };
    match &s {
        Some(s) => println!("{s}"),
        None => println!("None"),
    };
    println!("{s:?}");
}

enum Location {
    _Point(i32),
    Range(i32, i32),
}

fn option6() {
    let l: Location = Location::Range(0, 5);
    let n = match l {
        Location::_Point(_) => -1,
        Location::Range(0, _) => 0,
        Location::Range(n, 5) => n,
        _ => -2,
    };
    println!("{n:#?}");
}

fn option7() {
    #[derive(Debug)]
    enum Either {
        _Left(usize),
        Right(String),
    }
    let x = Either::Right(String::from("Hello world"));
    let value = match &x {
        Either::_Left(n) => *n,
        Either::Right(s) => s.len(),
    };
    println!("{x:?} {value}");
}

fn option8() {
    let config_max = Some(3u8);
    // You can think of if let as syntatic sugar for match
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    let mut count = 0;
    if let Some(state) = config_max {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("{count}");
}

fn option9() {
    let f = |o: &Option<String>| o.is_none();
    let v = Some(String::from("hello"));
    println!("{}", f(&v));
    println!("{}", f(&None));
}

fn question_mark_operator_on_option(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn option10() {
    println!(
        "{}",
        question_mark_operator_on_option("abcdefg").unwrap_or('-')
    );
    println!("{}", question_mark_operator_on_option("").unwrap_or('-'));
}

fn collection5() {
    let v = vec![1, 2, 3, 4, 5];

    // usual way to reference a value stored in a vector.
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); // an alternative way to referece a value stored in a vector
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // alternative way using `if let` idiom.
    if let Some(third) = v.get(9) {
        println!("The third element is {third}");
    } else {
        println!("There is no third element.");
    }
}

fn collection6() {
    let v = vec![1, 2, 3];
    for n_ref in &v {
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }

    let mut v = vec![1, 2, 3];
    for n_ref in &mut v {
        *n_ref += 50;
    }

    let til = 3;
    for i in 0..til {
        println!("{}", v[i]);
    }

    let mut v = Vec::new();
    let s = String::from("hello");
    v.push(s);
    v[0].push_str("world");
    println!("{}", v[0]);

    let v = vec![String::from("hello")];
    if let Some(s) = v.get(0) {
        println!("{s}");
    } else {
        println!("none.");
    }

    let mut s = v[0].clone();
    s.push_str("world");
    println!("{s}");
}

fn iter1() {
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: Iter<'_, i32> = v.iter();
    let n1: &i32 = iter.next().unwrap();
    let n2: &i32 = iter.next().unwrap();
    let end: Option<&i32> = iter.next();
    println!("{v:#?} {n1} {n2} {end:?}");
    v.push(3);
}

fn iter2() {
    // one way to iterate without using a pointer is to use Range.
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: Range<usize> = 0..v.len();
    let i1: usize = iter.next().unwrap();
    let n1: &i32 = &v[i1];
    println!("{v:?} {iter:?} {i1} {n1}");
    v.push(3);
}

fn iter3() {
    let mut v = vec![1, 2, 3];
    for i in 0..v.len() {
        v[i] += 1;
    }
    println!("{v:?}");

    let mut v: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<&mut i32> = Vec::new();
    for i in &mut v {
        v2.push(i);
    }

    *v2[0] = 5;
    let a = *v2[0];
    let b = v[0];
    println!("{a} {b}");
}

fn str1() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    // the reason to use this particular expression with the `+` operator has to do with the signature of the add function:
    // fn add(self, s: &str) ->  String {...}
    // we can see in the signature that add takes ownership of self (because it does not have an &).
    let s3 = s1 + &s2;
    // so, s1 is now unusable here, as it was moved.
    println!("{}", s3);

    // the behavior of the `+` operator can get unwieldy, for example:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3; // a bit convoluted perhaps, and
                                        // allocates in the heap many times (one for each time + is called, at max).
                                        // so it's definetely better to use format!(), instead.
    println!("{s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let mut s1 = String::from("hello");
    s1.push_str("world");
    println!("{s1}");
}

fn str2() {
    // Rust strings does not support indexing.
    let s1 = String::from("hello");
    // String is a wrapper over a Vec<u8>.
    let s2 = String::from("Здравствуйте"); // 12 glyphs, 24 bytes.
    let s3 = "नमस्ते".to_string();
    println!("{} {} {}", s1.len(), s2.len(), s3.len());
    // let answer = &s2[0];
    // println!("{answer}"); <- fails.
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}

// You might be wondering whether there is a runtime cost when using generic type parameters.
// The good news is that using generic types won't make your program run any slower than it
// would with concrete types.

// Rust accomplishes this by performing monomorphization of the code using generics at compile time.
// Monomorphization is the process of turning generic code into specific code by filling in the
// concrete types that are used when compiled.

impl Point<f32, f32> {
    // distance from origin.
    fn distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn generic1() {
    let a = Point { x: 5, y: 10 };
    let b = Point { x: 5.0, y: 10.0 };
    let c = Point { x: 5, y: 10.0 };

    println!("p.x = {}, p.y = {}", a.x(), a.y());
    println!(
        "p.x = {}, p.y = {} -> distance = {}",
        b.x(),
        b.y(),
        b.distance()
    );
    println!("p.x = {}, p.y = {}", c.x(), c.y());
}

pub mod trait1 {
    pub trait Summary {
        fn summarize_author(&self) -> String {
            "<empty>".to_string()
        }

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
}

use crate::trait1::*;

struct Pweet {}

impl Summary for Pweet {}

fn trait1() {
    let tweet = Tweet {
        username: String::from("savi0"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", Pweet {}.summarize_author());
}

pub trait Pointer {
    fn my(&self) -> String {
        format!("{:p}", self)
    }
}

impl Pointer for String {}

pub fn print_ptr(p: &impl Pointer) {
    println!("pointer: {}", p.my());
}

// The impl Trait syntax works for straightforward cases but is actually syntax
// sugar for a longer form known as a trait bound; it looks like this:

pub fn address<T: Pointer>(item: &T) -> String {
    // This longer form is equivalent to the example in the previous section but
    // is more verbose. We place trait bounds with the declaration of the generic
    // type parameter after a colon and inside angle brackets.
    format!("address: {:p}", item)
}

fn trait2() {
    println!("pointer: {}", "foobar".to_string().my());
    print_ptr(&"barfoo".to_string());
    println!("{}", address(&"barfoo".to_string()));
}

use std::fmt::{Debug, Display};

pub fn show1(item: &(impl Pointer + Display)) {
    println!("{} @<{:p}>", item, item);
}

pub fn show2<T: Pointer + Display>(item: &T) {
    println!("{} @<{:p}>", item, item);
}

pub fn show3<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    println!("t = {}@<{:p}>, u = {:?}@<{:p}>", t, t, u, u);
    0
}

pub fn show4<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("t = {}@<{:p}>, u = {:?}@<{:p}>", t, t, u, u);
    0
}

pub fn addressable(s: &str) -> impl Pointer {
    String::from(s)
}

fn trait3() {
    show1(&"abc".to_string());
    show2(&"cde".to_string());
    show3(&"hello".to_string(), &["world"]);
    show4(&"hello".to_string(), &["world"]);
}

fn displayable<T: Display>(t: T) -> impl Display {
    t
}

fn trait4() {
    let s = String::from("hello");
    let mut s2 = displayable(s).to_string(); // without to_string()...
    s2.push_str(" world"); //  s2 wouldn't have push_str(), and the code wouldn't compile.
    println!("{s2}");
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Conditionally implement method for this trait-bound.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("largest is x = {}", self.x);
        } else {
            println!("largest is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a trait for any type that implements another trait.
// Implementations of a trait on any type that satisfies the trait bounds are called blanket
// implementations and are extensively used in the Rust standard library. For example,
// the standard library implements the ToString trait on any type that implements the Display
// trait. The impl block in the standard library looks similar to this code:
//
// impl<T: Display> ToString for T {
//    // -- snip --
// }
//
// let s = 3.to_string();

fn trait5() {
    let p1 = Pair::new(1, 2);
    p1.cmp_display();
}

// Lifetime annotations don’t change how long any of the references live. Rather, they describe the
// relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
//
// The function signature now tells Rust that for some lifetime 'a, the function takes two parameters,
// both of which are string slices that live at least as long as lifetime 'a. The function signature
// also tells Rust that the string slice returned from the function will live at least as long as
// lifetime 'a.
//
// In practice, it means that the lifetime of the reference returned by the longest function is the
// same as the smaller of the lifetimes of the values referred to by the function arguments. These
// relationships are what we want Rust to use when analyzing this code.
//
// Remember, when we specify the lifetime parameters in this function signature, we’re not changing
// the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker
// should reject any values that don’t adhere to these constraints.
//
// Note that the longest function doesn’t need to know exactly how long x and y will live, only that
// some scope can be substituted for 'a that will satisfy this signature.
//
// When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is
// the part of the scope of x that overlaps with the scope of y. In other words, the generic lifetime
// 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y.
//
// Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned
// reference will also be valid for the length of the smaller of the lifetimes of x and y.
//
// Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return
// values of functions.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

fn shortest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() <= y.len() {
        x
    } else {
        y
    }
}

fn lifetime1() {
    let s1 = "hello";
    let s2 = "world";

    println!("longest: {}", longest(s1, s2));
    println!("shortest: {}", shortest("hello", "world"));

    // Let’s look at how the lifetime annotations restrict the longest function by passing in
    // references that have different concrete lifetimes. Listing 10-22 is a straightforward example.

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // The code below fails to compile:

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
}

// We can define structs to hold references, but in that case we would need to add a lifetime
// annotation on every reference in the struct’s definition.

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    // This annotation means an instance of `ImportantExcerpt` can’t outlive the reference
    // it holds in its `part` field.
    part: &'a str,
}

fn lifetime2() {
    let s1 = String::from("abc def ghi");
    let first = s1.split(' ').next().expect("couldn't find a space");
    let i = ImportantExcerpt { part: first };
    println!("{:?}, {}", i, i.part);
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn lifetime3() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.level());
    i.announce_and_return_part(&novel);
}

// The compiler uses three rules to figure out the lifetimes of the references when there
// aren’t explicit annotations. The first rule applies to input lifetimes, and the second
// and third rules apply to output lifetimes.
//
// If the compiler gets to the end of the three rules and there are still references for
// which it can’t figure out lifetimes, the compiler will stop with an error. These rules
// apply to fn definitions as well as impl blocks.
//
// The first rule is that the compiler assigns a different lifetime parameter to each
// lifetime in each input type. References like &'_ i32 needs a lifetime parameter,
// and structures like ImportantExcerpt<'_> need a lifetime parameter. For example:
//
//  * The function fn foo(x: &i32) would get one lifetime parameter and become
//    fn foo<'a>(x: &'a i32).
//  * The function fn foo(x: &i32, y: &i32) would get two lifetime parameters
//    and become fn foo<'a, 'b>(x: &'a i32, y: &'b i32).
//  * The function fn foo(x: &ImportantExcerpt) would get two lifetime parameters
//    and become fn foo<'a, 'b>(x: &'a ImportantExcerpt<'b>).
//
// The second rule is that, if there is exactly one input lifetime parameter, that
// lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
//
// The third rule is that, if there are multiple input lifetime parameters, but one of
// them is &self or &mut self because this is a method, the lifetime of self is assigned
// to all output lifetime parameters. This third rule makes methods much nicer to read and
// write because fewer symbols are necessary.

// For further information on lifetime check out the "lifetime elision rules", or start here:
// https://rust-book.cs.brown.edu/ch10-03-lifetime-syntax.html#lifetime-elision

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetime4() {
    let s: &'static str = "I have a static lifetime.";
    println!("{:p}: {}", s, s);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result =
        longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
    println!("The longest string is {}", result);
}

use std::thread;
use std::time::Duration;

fn closure1() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_millis(250));
        num
    };
    println!("{}", expensive_closure(100));
}

fn closure2() {
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    // The add_one_v3 and add_one_v4 lines require the closures to be
    // evaluated to be able to compile because the types will be inferred
    // from their usage.

    println!("{}", add_one_v1(0));
    println!("{}", add_one_v2(1));
    println!("{}", add_one_v3(2));
    println!("{}", add_one_v4(3));
}

fn closure3() {
    let f = |_| {};
    let s = String::from("hello");
    f(s); // a "toilet closure", similar to std::mem::drop, moves an
          // argument and causees it to be dropped.
    println!("s was dropped and is thus unusable.");
}

// Closures can capture values from their environment in three ways, which
// directly map to the three ways a function can take a parameter: borrowing
// immutably, borrowing mutably, and taking ownership. The closure will decide
// which of these to use based on what the body of the function does with the
// captured values.

fn closure4() {
    // borrowing immutably
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn closure5() {
    // borrowing mutably
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // notice that println! is omitted here.
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn closure6() {
    // taking ownership
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // we use the keyword `move` here to force the closure to take
    // ownership of the values it uses in the environment. notice
    // it will do so even though the body of the closure doesn't
    // strictly need owership.

    // This technique is mostly useful when passing a closure to a
    // new thread to move the data so that it’s owned by the new
    // thread.

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

// A closure body can do any of the following: move a captured
// value out of the closure, mutate the captured value, neither
// move nor mutate the value, or capture nothing from the
// environment to begin with.

// The way a closure captures and handles values from the environment
// affects which traits the closure implements, and traits are how
// functions and structs can specify what kinds of closures they can use.
//
// Closures will automatically implement one, two, or all three of
// these Fn traits, in an additive fashion, depending on how the
// closure’s body handles the values:
//
//   - `FnOnce` applies to a closure that can be called once. All
//     closures implement at least this trait, because all closures
//     can be called. Closures that move captured values out of its body
//     will only implement `FnOnce` and none of the other Fn traits,
//     because it can only be called once.
//
//   - `FnMut` applies to closures that don't move captured values out of
//     their body, but that might mutate the captured values. These closures
//     can be called more than once.
//
//   - `Fn` applies to closures that don't move captured values out of
//     their body and that don't mutate captured values, as well as
//     closures that capture nothing from their environment. These closures
//     can be called more than once without mutating their environment,
//     which is important in cases such as calling a closure multiple times
//     concurrently.

use std::option::Option;

struct MyOption<T>(Option<T>);

impl<T> MyOption<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T, // a function that can be called once, with no
                          // arguments, and returns a value of type T.
    {
        match self {
            MyOption(Some(x)) => x,
            MyOption(None) => f(),
        }
    }
}

fn closure7() {
    let o1 = MyOption(Some(3)).unwrap_or_else(|| 0);
    let o2 = MyOption(None).unwrap_or_else(|| 0);

    println!("{} {}", o1, o2);
}

// Now let’s look at the standard library method sort_by_key defined on slices,
// to see how that differs from unwrap_or_else and why sort_by_key uses FnMut
// instead of FnOnce for the trait bound.

fn closure8() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // The closure gets one argument in the form of a reference to the current
    // item in the slice being considered, and returns a value of type K that
    // can be ordered:
    //
    // pub fn sort_by_key<K, F>(&mut self, mut f: F)
    // where
    //     F: FnMut(&T) -> K,
    //     K: Ord, {...}

    list.sort_by_key(|r| r.width);

    // The reason sort_by_key is defined to take an FnMut closure is that it calls
    // the closure multiple times: once for each item in the slice. The closure
    // |r| r.width doesn’t capture, mutate, or move out anything from its environment,
    // so it meets the trait bound requirements.

    println!("{:?}", list);
}

// In contrast, the code below (Listing 13-8) shows an example of a closure that
// implements just the FnOnce trait, because it moves a value out of the environment.
// The compiler won’t let us use this closure with `sort_by_key`:

fn closure9() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations: Vec<Rectangle> = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        // sort_operations.push(value); // can't do, as it would move the
        //                              // captured environment `value`
        //                              // out of the closure, into the vector.
        sort_operations.push(*r);
        r.width
    });

    println!("{:?} {} -> {:?}", list, value, sort_operations);
}

// Note: Functions can implement all three of the `Fn` traits too.
// If what we want to do doesn’t require capturing a value from the environment,
// we can use the name of a function rather than a closure where we need something
// that implements one of the `Fn` traits. For example, on an `Option<Vec<T>>`
// value, we could call `unwrap_or_else(Vec::new)` to get a new, empty vector if
// the value is `None`.

fn closure10() {
    let v1: Vec<i32> = None.unwrap_or_else(Vec::new);
    let v2 = Some(vec![1, 2, 3]).unwrap_or_else(Vec::new);

    println!("{:?}", v1);
    println!("{:?}", v2);
}

// The code below yields the error: hidden type for `impl Fn() -> String` captures
// lifetime that does not appear in bounds
//
// fn make_a_cloner(s_ref: &str) -> impl Fn() -> String {
//     move || s_ref.to_string()
// }
//
// What does it mean? What is a hidden type? Why does it capture a lifetime? Why does
// that lifetime need to appear in a bound?
//
// Let's see what would happen if Rust allowed the above code to compile.
//
// fn main() {
//     let s_own = String::from("hello world");
//     let cloner = make_a_cloner(&s_own);
//     drop(s_own);
//     cloner();
// }
//
// It could eventually lead to an undefined behavior, right?
//
// We must tell Rust that the closure returned from `make_a_cloner` must not live
// longer than s_ref. By using a lifetime parameter like this:

fn make_a_cloner_v1<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
    move || s_ref.to_string()
}

// Note that we can use the lifetime elision rules to make the function type
// more concise. We can remove the <'a> generic so long as we keep an indicator
// that the returned closure depends on _some_ lifetime, like this:

fn make_a_cloner_v2(s_ref: &str) -> impl Fn() -> String + '_ {
    move || s_ref.to_string()
}

fn closure11() {
    let s_own = String::from("hello world");
    let cloner = make_a_cloner_v1(&s_own);
    // drop(s_own); // cannot move out of `s_own` because it is borrowed.
    cloner();

    let cloner = make_a_cloner_v2(&s_own);
    // drop(s_own); // ...
    cloner();
}

fn closure12() {
    let mut s = String::from("hello");
    // Notice that while the following lines won't compile...
    // let add_suffix = || s.push_str(" world");
    // println!("{s}");
    // add_suffix();
    // These will:
    let add_suffix = |s: &mut String| s.push_str(" world");
    println!("{s}");
    add_suffix(&mut s);
}

// Notice that in the case below we only need FnMut, as the value
// is passed by reference to `f`, thus, not moving any value out
// of the closure context.

fn for_each_mut<T, F: FnMut(&mut T)>(v: &mut Vec<T>, mut f: F) {
    for x in v.iter_mut() {
        f(x);
    }
}

fn closure13() {
    let mut v = vec![1, 2, 3];
    for_each_mut(&mut v, |x| *x = 0);
    println!("{:?}", v);
}

// All iterators implement a trait named Iterator that is defined in the
// standard library. The definition of the trait looks like this:
//
// pub trait Iterator {
//     type Item;
//
//     fn next(&mut self) -> Option<Self::Item>;
//
//     // methods with default implementations elided.
// }
//
// Notice this definition uses some new syntax: `type Item` and `Self::Item`
// which defines an "associated type" with this trait. We'll cover this
// later, for now all you need to know is that this code says implementing
// the Iterator trait requires that you also define an `Item` type, and
// this `Item` type is used in the return type of the `next` method. The
// `Item` type will be the type returned from the iterator.
//
// The `Iterator` trait only requires implementors to define one method: `next`,
// which returns one item of the iterator at a time wrapped in `Some` and, when
// iteration is over, returns `None`.

fn iter4() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter(); // needs to be mutable because calling
                                 // `next` changes/consumes its state.

    // The values we get from calls to `next` are immutable references.
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // The `iter` method produces an iterator over immutable references. If we want
    // to create an iterator that takes ownership of `v1` and returns owned values,
    // we can call `into_iter` instead of iter. Similarly, if we want to iterate
    // over mutable references, we can call `iter_mut` instead of `iter`.
}

// `Iterator` has a number of different methods with default implementation
// that calls `next`. Methods that call `next` are called "consuming adaptors",
// because calling them uses up the iterator. One example is the `sum` method,
// which takes ownership of the iterator and iterates through the items by
// repeatedly calling `next`, thus consuming the iterator.

fn iter5() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    // can't use v1_iter anymore because `sum` took its ownership.
    assert_eq!(total, 6);
}

// "Iterator adaptors" are methods defined on the `Iterator` trait that don't
// consume the iterator. Instead, they produce different iterators by changing
// some aspect of the original iterator.

fn iter6() {
    // The "iterator adaptor" `map` takes a closure to call on each item as the items
    // are iterated through. The `map` method returns a new iterator that produces
    // the modified items.

    let v1: Vec<i32> = vec![1, 2, 3];

    // The method `map` returns a new iterator and does not consume anything.
    // The method `collect` consumes the iterator, collecting the resulting values
    // into a collection data type.

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("{:?} -> {:?}", v1, v2);
}

fn greater_than_area(v: Vec<Rectangle>, area: u32) -> Vec<Rectangle> {
    v.into_iter()
        .filter(|s| s.width * s.height > area)
        .collect()
}

fn iter7() {
    let rectangles = vec![
        Rectangle {
            height: 1,
            width: 1,
        },
        Rectangle {
            height: 2,
            width: 2,
        },
        Rectangle {
            height: 3,
            width: 3,
        },
        Rectangle {
            height: 4,
            width: 4,
        },
    ];
    println!("{:?}", greater_than_area(rectangles, 4));
    // rectangles is unusable here.
}

fn iter8() {
    let v = vec![1, 2, 3];

    // the following two snippets are semantically equivalent.

    let mut iter = v.iter();
    while let Some(x) = iter.next() {
        println!("{}", x);
    }

    let iter = v.iter();
    for x in iter {
        println!("{}", x);
    }
}

fn iter9() {
    let v = vec![1, 2, 3, 4];

    let a: Vec<_> = v
        .iter()
        .filter(|x: &&i32| *x % 2 == 0)
        .map(|x: &i32| x * 2)
        .collect();

    let b: Vec<_> = v
        .iter()
        .map(|x: &i32| x * 2)
        .filter(|x: &i32| x % 2 == 0)
        .collect();

    println!("{:?} {:?}", a, b);
}

fn iter10() {
    let mut data = [1; 1000];
    let buffer: &mut [i32] = &mut data;
    let coefficients: [i64; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let qlp_shift: i16 = 8i16;

    // What assembly code would this Rust code compile to? Well, as of this writing,
    // it compiles down to the same assembly you’d write by hand. There’s no loop at
    // all corresponding to the iteration over the values in coefficients: Rust knows
    // that there are 12 iterations, so it “unrolls” the loop. Unrolling is an
    // optimization that removes the overhead of the loop controlling code and instead
    // generates repetitive code for each iteration of the loop.

    // All of the coefficients get stored in registers, which means accessing the values
    // is very fast. There are no bounds checks on the array access at runtime. All these
    // optimizations that Rust is able to apply make the resulting code extremely efficient.
    // Now that you know this, you can use iterators and closures without fear! They make
    // code seem like it’s higher level but don’t impose a runtime performance penalty for
    // doing so.

    // Closures and iterators are Rust features inspired by functional programming language
    // ideas. They contribute to Rust’s capability to clearly express high-level ideas at
    // low-level performance. The implementations of closures and iterators are such that
    // runtime performance is not affected. This is part of Rust’s goal to strive to provide
    // zero-cost abstractions.

    for i in 12..buffer.len() {
        let prediction = coefficients.iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>() >> qlp_shift;
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }
    println!("prediction = {:?}", &buffer[0..16]);
}

fn main() {
    println!("-=- tuple() -=-");
    tuple();

    println!("-=- fmt(0)-=-");
    fmt(0);

    println!("-=- cond(true) -=-");
    cond(true);

    println!("-=- cond(false) -=-");
    cond(false);

    println!("-=- lop() -=-");
    lop();

    println!("-=- iter() -=-");
    iter();

    println!("-=- arr() -=-");
    arr();

    println!("-=- b0x() -=-");
    b0x();

    println!("-=- suf(\"0\") -=-");
    println!("{}", suf(String::from("0")));

    println!("-=- refptr() -=-");
    refptr();

    println!("-=- fmt() -=-");
    pfmt();

    println!("-=- ptr1() -=-");
    ptr1();

    println!("-=- own1() -=-");
    own1();

    println!("-=- own2() -=-");
    own2();

    println!("-=- own3() -=-");
    own3();

    println!("-=- own4() -=-");
    own4();

    println!("-=- own5() -=-");
    own5();

    println!("-=- own6() -=-");
    own6();

    println!("-=- own7() -=-");
    own7();

    println!("-=- own8() -=-");
    own8();

    println!("-=- own9() -=-");
    own9();

    println!("-=- move1() -=-");
    move1();

    println!("-=- move2() -=-");
    move2();

    println!("-=- move3() -=-");
    move3();

    println!("-=- move4() -=-");
    move4();

    println!("-=- move5() -=-");
    move5();

    println!("-=- static1() -=-");
    println!("{}", static1());

    println!("-=- rc1() -=-");
    println!("{}", rc1());

    println!("-=- slot1() -=-");
    slot1();

    println!("-=- cols() -=-");
    cols();

    println!("-=- fixsafe() -=-");
    fixsafe();

    println!("-=- slice1() -=-");
    slice1();

    println!("-=- slice2() -=-");
    slice2();

    println!("-=- slice3() -=-");
    slice3();

    println!("-=- tuplestruct1() -=-");
    tuplestruct1();

    println!("-=- unitstruct1() -=-");
    unitstruct1();

    println!("-=- assocfunc1() -=-");
    assocfunc1();

    println!("-=- assocfunc2() -=-");
    assocfunc2();

    println!("-=- assocfunc3() -=-");
    assocfunc3();

    println!("-=- mutref1() -=-");
    mutref1();

    println!("-=- deref1() -=-");
    deref1();

    println!("-=- enum1() -=-");
    enum1();

    println!("-=- option1() -=-");
    option1();

    println!("-=- option2() -=-");
    option2();

    println!("-=- option3() -=-");
    option3();

    println!("-=- option4() -=-");
    option4();

    println!("-=- option5() -=-");
    option5();

    println!("-=- option6() -=-");
    option6();

    println!("-=- option7() -=-");
    option7();

    println!("-=- option8() -=-");
    option8();

    println!("-=- option9() -=-");
    option9();

    println!("-=- option10() -=-");
    option10();

    println!("-=- collection5() -=-");
    collection5();

    println!("-=- collection6() -=-");
    collection6();

    println!("-=- iter1() -=-");
    iter1();

    println!("-=- iter2() -=-");
    iter2();

    println!("-=- iter3() -=-");
    iter3();

    println!("-=- str1() -=-");
    str1();

    println!("-=- str2() -=-");
    str2();

    println!("-=- generic1() -=-");
    generic1();

    println!("-=- trait1() -=-");
    trait1();

    println!("-=- trait2() -=-");
    trait2();

    println!("-=- trait3() -=-");
    trait3();

    println!("-=- trait4() -=-");
    trait4();

    println!("-=- trait5() -=-");
    trait5();

    println!("-=- lifetime1() -=-");
    lifetime1();

    println!("-=- lifetime2() -=-");
    lifetime2();

    println!("-=- lifetime3() -=-");
    lifetime3();

    println!("-=- lifetime4() -=-");
    lifetime4();

    println!("-=- closure1() -=-");
    closure1();

    println!("-=- closure2() -=-");
    closure2();

    println!("-=- closure3() -=-");
    closure3();

    println!("-=- closure4() -=-");
    closure4();

    println!("-=- closure5() -=-");
    closure5();

    println!("-=- closure6() -=-");
    closure6();

    println!("-=- closure7() -=-");
    closure7();

    println!("-=- closure8() -=-");
    closure8();

    println!("-=- closure9() -=-");
    closure9();

    println!("-=- closure10() -=-");
    closure10();

    println!("-=- closure11() -=-");
    closure11();

    println!("-=- closure12() -=-");
    closure12();

    println!("-=- closure13() -=-");
    closure13();

    println!("-=- iter4() -=-");
    iter4();

    println!("-=- iter5() -=-");
    iter5();

    println!("-=- iter6() -=-");
    iter6();

    println!("-=- iter7() -=-");
    iter7();

    println!("-=- iter8() -=-");
    iter8();

    println!("-=- iter9() -=-");
    iter9();

    println!("-=- iter10() -=-");
    iter10();
}
