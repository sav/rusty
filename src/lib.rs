// lib.rs, Examples from the book "Rust Programming Language"
// Copyright (C) 2022-2024, Savio Sena <savio.sena@gmail.com>

pub mod network;

#[cfg(test)]
mod tests {
    use super::network;

    #[test]
    fn network_connect() {
        // assert_eq!(result, 4);
        network::connect();
    }
}
