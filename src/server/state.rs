

pub struct Mailbox {
    pub name: String,
    pub messages: Vec<String>,
    pub exists: u32,
}


pub struct State {
    pub mailboxes: Vec<Mailbox>,

}