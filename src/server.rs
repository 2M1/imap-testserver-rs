pub mod state;
pub mod imap;
/// The server ist the main struct of the library. It is used for the test to start and communicate with the server.
/// 
/// calling new will start the server on the given port in a separate thread.
pub struct ImapTestServer {
    pub port: u16,
}

impl ImapTestServer {
    pub fn new(port: u16) -> Self {
        ImapTestServer {
            port: port,
        }
    }
}