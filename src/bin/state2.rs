// state2.rs, Examples from the book "Rust Programming Language"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

/// ## Encoding States and Behavior as Types
///
/// We’ll show you how to rethink the state pattern to get a different set of trade-offs. Rather than
/// encapsulating the states and transitions completely so outside code has no knowledge of them,
/// we’ll encode the states into different types. Consequently, Rust’s type checking system will
/// prevent attempts to use draft posts where only published posts are allowed by issuing a compiler
/// error.

mod blog {
    pub struct Post {
        content: String,
    }

    pub struct DraftPost {
        content: String,
    }

    pub struct PendingReviewPost {
        content: String,
    }

    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }
}

use blog::Post;

/// The changes we needed to make to `main` to reassign post mean that this implementation doesn’t quite follow
/// the object-oriented state pattern anymore: the transformations between the states are no longer encapsulated
/// entirely within the `Post` implementation. However, our gain is that invalid states are now impossible because
/// of the type system and the type checking that happens at compile time! This ensures that certain bugs, such as
/// display of the content of an unpublished post, will be discovered before they make it to production.
///
/// We’ve seen that even though Rust is capable of implementing object-oriented design patterns, other patterns,
/// such as encoding state into the type system, are also available in Rust. These patterns have different trade-offs.
/// Although you might be very familiar with object-oriented patterns, rethinking the problem to take advantage of Rust’s
/// features can provide benefits, such as preventing some bugs at compile time. Object-oriented patterns won’t always
/// be the best solution in Rust due to certain features, like ownership, that object-oriented languages don’t have.

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
