// From: https://rust-book.cs.brown.edu/ch09-03-to-panic-or-not-to-panic.html

// It’s advisable to have your code panic when it’s possible that your code
// could end up in a bad state. In this context, a bad state is when some
// assumption, guarantee, contract, or invariant has been broken, such as when
// invalid values, contradictory values, or missing values are passed to your
// code—plus one or more of the following:
//
//  - The bad state is something that is unexpected, as opposed to something that
//    will likely happen occasionally, like a user entering data in the wrong format.
//  - Your code after this point needs to rely on not being in this bad state, rather
//    than checking for the problem at every step.
//  - There’s not a good way to encode this information in the types you use.

// If someone calls your code and passes in values that don’t make sense, it’s
// best to return an error if you can so the user of the library can decide what
// they want to do in that case. However, in cases where continuing could be
// insecure or harmful, the best choice might be to call panic!

// Panicking when the contract is violated makes sense because a contract violation
// always indicates a caller-side bug and it’s not a kind of error you want the
// calling code to have to explicitly handle. In fact, there’s no reasonable way for
// calling code to recover; the calling programmers need to fix the code.

// However, having lots of error checks in all of your functions would be verbose and
// annoying. Fortunately, you can use Rust’s type system (and thus the type checking done
// by the compiler) to do many of the checks for you. If your function has a particular
// type as a parameter, you can proceed with your code’s logic knowing that the compiler
// has already ensured you have a valid value.

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

use std::fmt;

impl fmt::Display for Guess {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.value)
}
}

fn main() {

    let guess = Guess::new(1);
    println!("{}", guess);

    // will abort:
    let guess = Guess::new(0);
    println!("{}", guess);
}
