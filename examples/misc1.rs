// misc1.rs

// further examples form `the rust programming language`, but from this new
// interactive version with quizzes: https://rust-book.cs.brown.edu

use std::any::type_name;

fn type_of<T>(_: &T) -> String {
    let type_name = type_name::<T>();
    type_name.to_string()
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
// Shared reference = immutable.
// Unique reference = mutable.

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
    let mut v: Vec<i32> = vec![1,2,3];
    let num: &i32 = &/*R*/v[2]; // borrow. requires v to be readable (R).
    // v./*W*/push(4); // requires v to be readable (R) and writable (W),
    //                  // but v is not writable anymore.
    println!("{}", *num);
    v.push(4); // v here gets its W permission back after num is unused.
}

fn own5() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];

    *num += 1;

    println!("third element is {}", *num);
    println!("vector is now {:?}", v);
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
}
