// misc1.rs

// further examples form `the rust programming language`, but from this new
// interactive version with quizzes: https://rust-book.cs.brown.edu

use std::any::type_name;
use std::fmt;

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
    let mut rect4 = Rectangle { width: 0, height: 1 };
    let rect5 = Rectangle { width: 1, height: 0 };
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
    let c1 : MyEnum = MyEnum::MyField(1);
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
        None => {},
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
    fn move_player(x: u8) { println!("{x}"); }
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
    fn reroll() { println!("reroll"); }
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
    Range(i32, i32)
}

fn option6() {
    let l: Location = Location::Range(0, 5);
    let n = match l {
        Location::_Point(_) => -1,
        Location::Range(0, _) => 0,
        Location::Range(n, 5) => n,
        _ => -2
    };
    println!("{n:#?}");
}

fn option7() {
    #[derive(Debug)]
    enum Either {
        _Left(usize),
        Right(String)
    }
    let x = Either::Right(String::from("Hello world"));
    let value = match &x {
        Either::_Left(n) => *n,
        Either::Right(s) => s.len()
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

fn collection5() {
 let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
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

    println!("-=- collection5() -=-");
    collection5();
}
