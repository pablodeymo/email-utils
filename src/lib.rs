use anyhow::{anyhow, Result};
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::extension::ClientId;
use lettre::smtp::ConnectionReuseParameters;
use lettre::{SmtpClient, SmtpTransport, Transport}; //SendableEmail, Envelope, EmailAddress,

pub fn main() {
//    let mut mailer = connect_mailer();

//    mailer.close();
}

pub fn sendmail(
    mailer: &mut SmtpTransport,
    dest_address: &str,
    dest_name: &str,
    from_address: &str,
    from_name: &str,
    subject: &str,
    text: &str,
) -> Result<()> {
    let email = lettre_email::EmailBuilder::new()
        .to((dest_address, dest_name))
        .from((from_address, from_name))
        .subject(subject)
        .text(text)
        .build()
        .map_err(|e| anyhow!("Error building mail. {}", e))?;

    mailer
        .send(email.into())
        .map_err(|e| anyhow!("Error sending mail. {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
