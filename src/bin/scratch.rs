// scratch.rs, Examples from the book "Rust Programming Language"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

#[derive(Debug)]
struct A1 {
    value: i32,
}

impl std::fmt::Display for A1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn f1<T>(v: &mut Vec<Box<T>>, a: T) {
    v.push(Box::new(a));
}

/// needs a lifetime specifier `'a` now because of the dynamic dispatch nature of trait objects.
fn f2<'a, T: std::fmt::Display + 'a>(v: &mut Vec<Box<dyn std::fmt::Display + 'a>>, t: T) {
    v.push(Box::new(t));
}

fn f3<T: std::fmt::Display + 'static>(v: &mut Vec<Box<dyn std::fmt::Display>>, t: T) {
    v.push(Box::new(t));
}

pub trait T1 {
    fn noop(&self);
}

impl T1 for A1 {
    fn noop(&self) {
        println!("noop");
    }
}

fn f4<T: T1 + 'static>(v: &mut Vec<Box<dyn T1>>, t: T) {
    v.push(Box::new(t));
}

fn ex1() {
    let a: A1 = A1 { value: 1 };
    let b = Box::new(a); // move
    println!("{}", b.value);

    let a: A1 = A1 { value: 1 };
    let mut v = vec![Box::new(A1 { value: 0 })];
    f1(&mut v, a);

    let a: A1 = A1 { value: 1 };
    // notice that we need the explicit type on the left hand side here.
    let mut v: Vec<Box<dyn std::fmt::Display>> = vec![Box::new(A1 { value: 0 })];
    f2(&mut v, a);

    let a: A1 = A1 { value: 1 };
    let mut v: Vec<Box<dyn std::fmt::Display>> = vec![Box::new(A1 { value: 0 })];
    f3(&mut v, a);

    let a = A1{ value: 1 };
    let mut v: Vec<Box<dyn T1>> = vec![];
    f4(&mut v, a);
}

// ---------------------------------
// Context: You are designing a simple user interface framework that consists of a tree of
// widgets, such as text and buttons.
//
// Functionality: The API provides a Widget trait that defines how a widget works. The API
// client implements the Widget trait, and calls it to render a UI.
//
// Designs: Below are several proposed designs to implement the functionality.

// Children must be Self.
pub trait Widget1: Sized {
    fn render(&self) -> Vec<Self>;
}

// Children are a Trait paramter.
pub trait Widget2<Children> {
    fn render(&self) -> Vec<Children>;
}

// Children are an associated type.
pub trait Widget3 {
    type Children: Widget3;
    fn render(&self) -> Vec<Self::Children>;
}

// Children are a reference trait object.
pub trait Widget4 {
    fn render(&self) -> Vec<&dyn Widget4>;
}

// Children are a boxed trait object.
pub trait Widget5 {
    fn render(&self) -> Vec<Box<dyn Widget5>>;
}

// ---------------------------------
// Requirement: The API client is expected to provide a single `WidgetImpl` `enum` representing
// all possible widgets, and implement the `Widget` trait for `WidgetImpl`.

#[derive(Debug)]
struct Q1aButton {
    value: String,
}

#[derive(Debug)]
struct Q1aTextField {
    value: String,
}

#[derive(Debug)]
enum WidgetImpl {
    Button(Q1aButton),
    TextField(Q1aTextField),
}

impl Widget1 for WidgetImpl {
    fn render(&self) -> Vec<Self> {
        match self {
            WidgetImpl::Button(x) => println!("{}", x.value),
            WidgetImpl::TextField(x) => println!("{}", x.value),
        }
        let w = WidgetImpl::Button(Q1aButton {
            value: String::from("button1"),
        });

        let u = WidgetImpl::TextField(Q1aTextField {
            value: String::from("textfield1"),
        });
        vec![w, u]
    }
}

impl Widget2<WidgetImpl> for WidgetImpl {
    fn render(&self) -> Vec<WidgetImpl> {
        let w = WidgetImpl::Button(Q1aButton {
            value: String::from("button1"),
        });

        let u = WidgetImpl::TextField(Q1aTextField {
            value: String::from("textfield1"),
        });

        vec![w, u]
    }
}

impl Widget3 for WidgetImpl {
    type Children = WidgetImpl;
    fn render(&self) -> Vec<Self::Children> {
        let w = WidgetImpl::Button(Q1aButton {
            value: String::from("button1"),
        });

        let u = WidgetImpl::TextField(Q1aTextField {
            value: String::from("textfield1"),
        });

        vec![w, u]
    }
}

impl Widget4 for WidgetImpl {
    fn render(&self) -> Vec<&dyn Widget4> {
        vec![]
    }
}

impl Widget5 for WidgetImpl {
    fn render(&self) -> Vec<Box<dyn Widget5>> {
        vec![]
    }
}

fn ex2() {
    let w = WidgetImpl::Button(Q1aButton {
        value: String::from("button1"),
    });
    let u = WidgetImpl::TextField(Q1aTextField {
        value: String::from("textfield1"),
    });

    dbg!(Widget1::render(&w));
    dbg!(Widget2::render(&w));
    dbg!(Widget3::render(&w));
    Widget4::render(&w);
    Widget5::render(&w);

    dbg!(Widget1::render(&u));
    dbg!(Widget2::render(&u));
    dbg!(Widget3::render(&u));
    Widget4::render(&u);
    Widget5::render(&u);
}

// ---------------------------------

pub struct Events1 {
    // ...
}

// Option 1: parallel and sequential are two separate methods
impl Events1 {
    pub fn register<E, F: Fn(E)>(&mut self, f: F) { /* .. */
    }
    pub fn register_sequential<E, F: Fn(E)>(&mut self, f: F) { /* .. */
    }
}

pub struct Events {}

// Option 2: parallel and sequential are two members of an enum
pub enum Callback<F> {
    Parallel(F),
    Sequential(F),
}
impl Events {
    pub fn register<E, F: Fn(E)>(&mut self, f: Callback<F>) { /* .. */
    }
}

// Option 3: parallel and sequential are markers in a trait method
pub trait Register<Marker, F, E> {
    fn register(&mut self, f: F);
}
pub struct Parallel;
pub struct Sequential;
impl<F, E> Register<Parallel, F, E> for Events
where
    F: Fn(E),
{
    fn register(&mut self, f: F) { /* .. */
    }
}
impl<F, E> Register<Sequential, F, E> for Events
where
    F: Fn(Sequential, E),
{
    fn register(&mut self, f: F) { /* .. */
    }
}

// ---------------------------------

fn main() {
    ex1();
    ex2();
}
