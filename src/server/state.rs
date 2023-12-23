
pub struct Mailbox {
    pub name: String,
    pub messages: Vec<String>,
    pub exists: u32,
}
pub struct State {
    pub mailboxes: Vec<Mailbox>,
    pub selected: Option<String>,
    pub num_idle_connections: u32,
}
