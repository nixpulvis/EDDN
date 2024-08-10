use eddn::{subscribe, Message, URL};

fn main() {
    for envelop in subscribe(URL) {
        if let Ok(envelop) = envelop {
            match envelop.message {
                Message::Other(o) => {
                    let event = o.as_object().unwrap().get("event");
                    if let Some(e) = event {
                        println!("{}", e);
                    }
                    // else {
                    //     dbg!(o);
                    // }
                },
                _ => {}
            }
        }
    }
}
