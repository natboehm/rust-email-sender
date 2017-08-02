extern crate lettre;

use lettre::transport::stub::StubEmailTransport;
use lettre::email::EmailBuilder;
use lettre::transport::EmailTransport;

fn main() {
    println!("Hello, world!");

    let email = EmailBuilder::new()
        .to("person@person.person")
        .from("sender@sender.sender")
        .subject("hello friend")
        .body("greetings")
        .build()
        .unwrap();

    let mut sender = StubEmailTransport;
    let result = sender.send(email);
    assert!(result.is_ok());
}
