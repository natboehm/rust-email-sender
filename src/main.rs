extern crate lettre;

use std::env;
use lettre::transport::smtp::{SecurityLevel, SmtpTransport, SmtpTransportBuilder};
use lettre::email::EmailBuilder;
use lettre::transport::smtp::authentication::Mechanism;
use lettre::transport::smtp::SUBMISSION_PORT;
use lettre::transport::EmailTransport;

fn main() {
    println!("Hello, world!");

    let email = EmailBuilder::new()
        .to("person@person.person")
        .from("sender@sender.sender")
        .subject("hello friend")
        .body("greetings")
        .build()
        .expect("Failed to build message");

    let mut transport = SmtpTransportBuilder::new(("smtp.mailgun.org", SUBMISSION_PORT))
        .expect("Failed to create transport")
        .credentials(&env::var("MAILGUN_USERNAME").unwrap_or("username".to_string())[..], &env::var("MAILGUN_PASSWORD").unwrap_or("password".to_string())[..])
        .security_level(SecurityLevel::AlwaysEncrypt)
        .smtp_utf8(true)
        .authentication_mechanism(Mechanism::CramMd5)
        .connection_reuse(true).build();
    println!("{:?}", transport.send(email.clone()));
    transport.send(email)?;
}
