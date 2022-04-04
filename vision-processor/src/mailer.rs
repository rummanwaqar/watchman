use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::transport::smtp::Error;
use lettre::{Message, SmtpTransport, Transport};

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub from: String,
    pub to: String,
    pub username: String,
    pub password: String,
    pub smtp: String,
}

pub fn send_notification(settings: Settings, filename: &str) -> Result<Response, Error> {
    let email = Message::builder()
        .from(settings.from.parse().unwrap())
        .reply_to(settings.from.parse().unwrap())
        .to(settings.to.parse().unwrap())
        .subject("Watchman Alert")
        .body(
            "Motion detected. Check footage at http://localhost:8000/admin/video/".to_string()
                + filename,
        )
        .unwrap();

    let creds = Credentials::new(settings.username, settings.password);

    let mailer = SmtpTransport::relay(&settings.smtp)
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email)
}
