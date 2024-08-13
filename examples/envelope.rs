use eddn::{subscribe, Message, URL};

fn main() {
    for envelop in subscribe(URL) {
        dbg!(envelop);
    }
}
