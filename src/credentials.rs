extern crate keyring;
use serde::{Deserialize, Serialize};
use serde_json::Result as SerdeResult;
use std::error::Error;

const SERVICE: &str = "catalysis";

/// A struct that represents a user session in an
/// instance of Catalysis. The instances is identified
/// by its fully-qualified-domain-name (fqdn).
#[derive(Serialize, Deserialize)]
pub struct Credentials {
    /// The session token.
    token: String,
}

/// It reads the credentials for a given Catalysis domain
/// and returns
pub fn read(fqdn: &str) -> Result<Credentials, Box<dyn Error>> {
    let entry = keyring_entry(&fqdn);
    let credentials_json = entry.get_password()?;
    let credentials: Credentials = serde_json::from_str(&credentials_json)?;
    Ok(credentials)
}

/// Given a fully-qualified-domain-name it returns a keyring::Entry
/// instance that can be used to read, update, and delete credentials
/// from the platform's credentials' manager.
///
/// # Arguments
///
/// * `fqdn` - A string slice that holds the fully-qualified-domain-value.
fn keyring_entry(fqdn: &str) -> keyring::Entry {
    keyring::Entry::new(&SERVICE, &fqdn)
}

#[cfg(test)]
mod test {}
