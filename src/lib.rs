use arraystring::{typenum::U255, ArrayString};
use validator::ValidationError;

pub type Mail = ArrayString<U255>;

pub fn validate_email(mail: &Mail) -> Result<(), ValidationError> {
    if mailchecker::is_valid(mail) {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid mail"))
    }
}

#[cfg(feature = "sendmail")]
pub fn connect_mailer() -> anyhow::Result<SmtpTransport> {
    use anyhow::anyhow;
    use lettre::smtp::authentication::{Credentials, Mechanism};
    use lettre::smtp::extension::ClientId;
    use lettre::smtp::ConnectionReuseParameters;
    use lettre::{SmtpClient, SmtpTransport, Transport};

    let smtp_server = std::env::var("SMTP_SERVER")
        .map_err(|e| anyhow!("Error getting SMTP_SERVER value: {}", e))?;
    let domain_server = std::env::var("DOMAIN_SERVER")
        .map_err(|e| anyhow!("Error getting DOMAIN_SERVER value: {}", e))?;
    let smtp_username = std::env::var("SMTP_USERNAME")
        .map_err(|e| anyhow!("Error getting SMTP_USERNAME value: {}", e))?;
    let smtp_password = std::env::var("SMTP_PASSWORD")
        .map_err(|e| anyhow!("Error getting SMTP_PASSWORD value: {}", e))?;

    Ok(
        SmtpClient::new_simple(&smtp_server) // Connect to a remote server on a custom port
            .map_err(|e| anyhow!("Error building new SmtpClient. {}", e))?
            .hello_name(ClientId::Domain(domain_server)) // Set the name sent during EHLO/HELO, default is `localhost`
            .credentials(Credentials::new(smtp_username, smtp_password)) // Add credentials for authentication
            .smtp_utf8(true) // Enable SMTPUTF8 if the server supports it
            .authentication_mechanism(Mechanism::Plain) // Configure expected authentication mechanism
            .connection_reuse(ConnectionReuseParameters::ReuseUnlimited) // Enable connection reuse
            .transport(),
    )
}

#[cfg(feature = "sendmail")]
pub fn sendmail(
    mailer: &mut lettre::SmtpTransport,
    dest_address: &str,
    dest_name: &str,
    from_address: &str,
    from_name: &str,
    subject: &str,
    text: &str,
) -> anyhow::Result<()> {
    use anyhow::anyhow;
    use lettre::smtp::authentication::{Credentials, Mechanism};
    use lettre::smtp::extension::ClientId;
    use lettre::smtp::ConnectionReuseParameters;
    use lettre::{SmtpClient, SmtpTransport, Transport};

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
