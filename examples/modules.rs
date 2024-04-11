// modules.rs, Examples from the book "Rust by Example"
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

// `#[outer_attribute]` applies to the item immediately following it.
#[allow(dead_code)]

mod my_mod {
    // `#![inner_attribute]` applies to the enclosing `item`.
    #![allow(dead_code)]

    fn private_function() {
        println!("private -> `my_mod::private_function()`");
    }

    pub fn function() {
        println!("public -> `my_mod::function()`");
    }

    pub fn indirect_access() {
        print!("public with indirection -> `my_mod::indirect_access()`, that calls\n\t> ");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("public -> `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("private -> `my_mod::nested::private_function()`");
        }

        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("visible within path(crate::my_mod) -> `my_mod::nested::public_function_in_my_mod()`, that calls\n\t> ");
            public_function_in_nested();
        }

        pub(self) fn public_function_in_nested() {
            println!("visible within the current module ->  `my_mod::nested::public_function_in_nested()`");
        }

        pub(super) fn public_function_in_super_mod() {
            println!("visible within the parent module -> `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("public -> `my_mod::call_public_function_in_my_mod()`, that calls\n\t> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    pub(crate) fn public_function_in_crate() {
        println!("visible within the current crate -> `my_mod::public_function_in_crate()`");
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("public -> `my_mod::private_nested::function()`");
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("still private despite pub(crate) -> `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("local -> `function()`");
}

mod cool {
    pub fn function() {
        println!("public -> `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        print!("called `my::indirect_call()`, that\n> ");

        self::function();
        function();

        self::cool::function();
        super::function();

        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    function();
    my_mod::function();
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();
    my_mod::public_function_in_crate();

    my::indirect_call();
}
