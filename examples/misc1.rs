// misc1.rs

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
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    *x += 1;                 // *x on the left-side modifies the heap value, so x points to the value 2

    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value

    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;        // so only one dereference is needed to read it

    println!("{}, {}, {}, {}, {}, {}", x, a, r1, b, r2, c);

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs();      // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs();       // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len();      // implicit reference
    assert_eq!(s_len1, s_len2);
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
}
