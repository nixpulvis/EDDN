use elite_journal::Event;
use eddn::{subscribe, Message};

fn main() {
    for envelop in subscribe("tcp://eddn.edcd.io:9500") {
        if let Ok(envelop) = envelop {
            match envelop.message {
                Message::Journal(entry) => {
                    match entry.event {
                        Event::Location(e) => {
                            dbg!(e);
                        },
                        Event::FsdJump(e) => {
                            dbg!(e);
                        },
                        Event::Docked(e) => {
                            dbg!(e);
                        },
                        _ => {},
                    }
                },
                _ => {}
            }
        }
    }
}
