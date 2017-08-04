extern crate lettre;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

use lettre::transport::smtp::{SecurityLevel, SmtpTransportBuilder};
use lettre::email::EmailBuilder;
use lettre::transport::smtp::authentication::Mechanism;
use lettre::transport::smtp::SUBMISSION_PORT;
use lettre::transport::EmailTransport;

fn main() {
    println!("Hello, world!");
    dotenv().ok();

    let mailgun_username = env::var("MAILGUN_SMTP_LOGIN").unwrap();
    let mailgun_password = env::var("MAILGUN_SMTP_PASSWORD").unwrap();
    let mailgun_server = env::var("MAILGUN_SMTP_SERVER").unwrap();
    println!("username: {:?} password: {:?} server: {:?}", mailgun_username, mailgun_password, mailgun_server);
    
    //let mailgun_username = &env::var("MAILGUN_USERNAME").unwrap_or("username".to_string())[..];
    //let mailgun_password = &env::var("MAILGUN_PASSWORD").unwrap_or("password".to_string())[..];

    let email = EmailBuilder::new()
        .to("natboehm15@gmail.com")
        .from(mailgun_username.as_str())
        .subject("hello friend")
        .body("greetings")
        .build()
        .expect("Failed to build message");

    let mut transport = SmtpTransportBuilder::new((mailgun_server.as_str(), SUBMISSION_PORT))
        .expect("Failed to create transport")
        .credentials(&mailgun_username, &mailgun_password)
        .security_level(SecurityLevel::AlwaysEncrypt)
        .smtp_utf8(true)
        .authentication_mechanism(Mechanism::Plain)
        .build();
    println!("built, about to send");
    let result = transport.send(email);
    println!("sent, about to expect");
    let result = result.expect("unable to send email");
    println!("result: {:?}", result);
}
