use eddn::{subscribe, Message, URL};

fn main() {
    for envelop in subscribe(URL) {
        if let Ok(envelop) = envelop {
            match envelop.message {
                Message::Journal(entry) => { dbg!(entry); },
                _ => {}
            }
        }
    }
}
