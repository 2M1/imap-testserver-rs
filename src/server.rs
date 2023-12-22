mod server;

struct ImapTestServer {
    pub port: u16,
}

impl ImapTestServer {
    pub fn new(port: u16) -> Self {
        ImapTestServer {
            port: port,
        }
    }
}