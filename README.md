# Rust Imap in-memory server for testing imap client applications

This library aims to provided a simple imap compliant server to use in `#[test]` functions, to test different imap features and circumstances, such as: Basic connection, idle receive, idle reconnect, etc.

## Status

This library is still in development, and is not yet ready for use, in fact I just started it, so it will take some time to get it to a usable state.

## (Planned) Usage

Something along the lines of the code below, however I'm not sure if this is the best way yet and might change it in a way, that better controls the parallel running server:

```rust
#[cfg(test)]
use imap_testserver::ImapTestServer;

#[test]
fn test_idle_reset() {
    let server = ImapTestServer::new(9993);
    server.new_mail("tester@example.com", "test1", "INBOX");
    server.on_connection(|ctx| {
        // will run in a separate thread
        thread::sleep(Duration::from_seconds(1));
        ctx.new_mail("tester@example.com", "test2", "INBOX");
    });

    let new_mails: Vec<Mail> = function_to_test("INBOX");

    assert_eq!(new_mails.len(), 2);
    assert!(new_mails.any(|m| m.subject == "test1"));
    assert!(new_mails.any(|m| m.subject == "test2"));
}

```