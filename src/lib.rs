pub mod server;


#[cfg(test)]
mod tests {
    use crate::server::ImapTestServer;

    #[test]
   fn basic_run_test() {
        let server = ImapTestServer::new(1234);

        assert_eq!(server.port, 1234);
    } 
}
