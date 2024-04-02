//! # Implementing an Object-Oriented Design Pattern
//!
//! The state pattern is an object-oriented design pattern. The _crux_ of the pattern is that we
//! define a set of states a value can have internally. The states are represented by a set of
//! **state objects**, and the value’s behavior changes based on its state.
//!
//! We’re going to work through an example of a blog post struct that has a field to hold its state,
//! which will be a state object from the set "draft", "review", or "published".

mod blog {
    /// This type implements the _state pattern_ and will hold a value that will be one of the three
    /// state objects representing the various states a post can be: in-draft, waiting for review,
    /// or published. The states change in response to the methods called by our library's user
    /// on the `Post` instance, but they don't have to manage the state changes directly. Also,
    /// users can't make mistakes with the states, like publishing a post before it's reviewed.
    ///
    /// Notice that this is the only object we're interacting with from the crate.
    pub struct Post {
        /// ## Why Not An Enum?
        ///
        /// You may have been wondering why we didn’t use an `enum` with the different possible post
        /// states as variants. That’s certainly a possible solution, try it and compare the end
        /// results to see which you prefer! One disadvantage of using an enum is every place
        /// that checks the value of the enum will need a `match` expression or similar to handle
        /// every possible variant. This could get more repetitive than this trait object solution.
        ///
        /// That would mean we would have to look in several places to understand all the implications
        /// of a post being in the published state! This would only increase the more states we added:
        /// each of those `match` expressions would need another arm.
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        /// Because `state` is an `Option<Box<dyn State>>` when we call `as_ref`, an
        /// `Option<&Box<dyn State>>` is returned. If we didn't call `as_ref`, we would get
        /// an error because we can't move `state` out of the borrowed `&self` of the function
        /// parameter.
        ///
        /// We call the `unwrap` method, which we know will never panic, because we know the
        // methods on `Post` ensure that `state` will always contain a `Some` value when those
        /// methods are done. This is one of the cases seen in Chapter-9, section _"Cases in Which
        /// You Have More Information than The Compiler"_.
        ///
        /// Then we call `content` n the `&Box<dyn State>>`, deref coercion will take effect on the
        /// `&` and the `Box` so the `content` method will ultimately be called on the type that
        /// implements the `State` trait.

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review());
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }

    trait State {
        /// Notice that rather than having `self`, `&self` or `&mut self` as the first parameter
        /// of the method, we have `self: Box<Self>`. This syntax means the method is only valid
        /// when called on a `Box` holding the type. This syntax takes ownership of Box<Self>,
        /// invalidating the old state so the state value of the Post can transform into a new state.
        /// In other words, to consume the old state, the `request_review` method needs to take
        /// ownership of the state value.
        ///
        /// This is where the `Option` in the state field of `Post` comes in: we call the `take`
        /// method to take the `Some` value out of the `state` field and leave a `None` in its place,
        /// because Rust doesn't let us have unpopulated fields in structs. This lets us move the
        /// `state` value out of `Post` rather than borrowing it. Then we'll set the post's `state`
        /// value to the result of this operation.
        ///
        /// We need to set `state` to `None` temporarily rather than setting it directly with code
        /// like `self.state = self.state.request_review()` to ensure `Post` can't use the old `state`
        /// value after we've transformed it into a new state.
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }

    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview{})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    struct PendingReview {}

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }
}

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today.");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
