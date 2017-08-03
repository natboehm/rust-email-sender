extern crate lettre;

use std::env;
use lettre::transport::smtp::{SecurityLevel, SmtpTransportBuilder};
use lettre::email::EmailBuilder;
use lettre::transport::smtp::authentication::Mechanism;
use lettre::transport::smtp::SUBMISSION_PORT;
use lettre::transport::EmailTransport;

fn main() {
    println!("Hello, world!");

    let email = EmailBuilder::new()
        .to("natboehm15@gmail.com")
        .from("postmaster@sandboxc2cd6ddb46a044bc8980ce9d77f67c6a.mailgun.org")
        .subject("hello friend")
        .body("greetings")
        .build()
        .expect("Failed to build message");

    let mailgun_username = "postmaster@sandboxc2cd6ddb46a044bc8980ce9d77f67c6a.mailgun.org";
    let mailgun_password = "5c87f977632540b7f8bab87af6c5ba0f";
    //let mailgun_username = &env::var("MAILGUN_USERNAME").unwrap_or("username".to_string())[..];
    //let mailgun_password = &env::var("MAILGUN_PASSWORD").unwrap_or("password".to_string())[..];
    println!("username: {:?} password: {:?}", mailgun_username, mailgun_password);
    let mut transport = SmtpTransportBuilder::new(("sandboxc2cd6ddb46a044bc8980ce9d77f67c6a.mailgun.org", SUBMISSION_PORT))
        .expect("Failed to create transport")
        .credentials(mailgun_username, mailgun_password)
        .security_level(SecurityLevel::AlwaysEncrypt)
        .smtp_utf8(true)
        .authentication_mechanism(Mechanism::CramMd5)
        .build();
    println!("built, about to send");
    let result = transport.send(email);
    println!("sent, about to expect");
    let result = result.expect("unable to send email");
    println!("result: {:?}", result);
}
