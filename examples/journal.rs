use eddn::{subscribe, Message, URL};
use elite_journal::entry::Event;

fn main() {
    for envelop in subscribe(URL) {
        if let Ok(envelop) = envelop {
            match envelop.message {
                Message::Journal(entry) => match entry.event {
                    Event::Location(e) => {
                        dbg!(e);
                    }
                    Event::FsdJump(e) => {
                        dbg!(e);
                    }
                    Event::Docked(e) => {
                        dbg!(e);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
