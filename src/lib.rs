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
