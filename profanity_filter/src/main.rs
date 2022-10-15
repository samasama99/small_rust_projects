pub struct Message {
    content: String,
    _user: String,
}

impl Message {
    pub fn new(ms: String, u: String) -> Message {
        Message {
            content: ms,
            _user: u,
        }
    }
    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.contains("stupid") {
            return None;
        }
        Some(&self.content)
    }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        Some(msg) => (true, msg),
        None => (false, "ERROR: illegal"),
    }
}

fn main() {
    let m0 = Message::new("hello there".to_string(), "toby".to_string());
    println!("{:?}", check_ms(&m0));

    let m1 = Message::new("".to_string(), "toby".to_string());
    println!("{:?}", check_ms(&m1));

    let m2 = Message::new("you are stupid".to_string(), "toby".to_string());
    println!("{:?}", check_ms(&m2));

    let m3 = Message::new("stupid".to_string(), "toby".to_string());
    println!("{:?}", check_ms(&m3));
}
