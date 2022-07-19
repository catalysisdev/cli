extern crate keyring;
use serde::{Deserialize, Serialize};
use std::error;

const SERVICE: &str = "catalysis";

/// A struct that represents a user session in an
/// instance of Catalysis. The instances is identified
/// by its fully-qualified-domain-name (fqdn).
#[derive(Serialize, Deserialize)]
pub struct Credentials {
    /// The session token.
    token: String,
}

pub struct CredentialStore {}

impl CredentialStore {
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
}

pub trait CredentialStoring {
    /// It reads the [credentials](Credentials) for a given Catalysis domain
    /// and returns
    ///
    /// # Arguments
    ///
    /// * `fqdn` - A string slice that holds the fully-qualified-domain-value.
    fn read(fqdn: &str) -> Result<Option<Credentials>, Box<dyn error::Error>>;

    /// Deletes an entry in the store that's identified by the given
    /// fully-qualified-domain-name
    ///
    /// # Arguments
    ///
    /// * `fqdn` - A string slice that holds the fully-qualified-domain-value.
    fn delete(fqdn: &str) -> Result<(), Box<dyn error::Error>>;

    /// Updates the [credentials](Credentials) for a given fully-qualified-domain-name
    ///
    /// # Arguments
    /// * `fqdn` - A string slice that holds the fully-qualified-domain-value.
    /// * `credentials` - An inmutable reference to the credentials.
    fn update(fqdn: &str, credentials: &Credentials) -> Result<(), Box<dyn error::Error>>;
}

impl CredentialStoring for CredentialStore {
    fn read(fqdn: &str) -> Result<Option<Credentials>, Box<dyn error::Error>> {
        let entry = CredentialStore::keyring_entry(&fqdn);
        let credentials_json_result = entry.get_password();
        let credentials_json = match credentials_json_result {
            Ok(json) => json,
            Err(keyring::Error::NoEntry) => return Ok(None),
            Err(error) => return Err(Box::new(error)),
        };
        let credentials: Credentials = serde_json::from_str(&credentials_json)?;
        Ok(Some(credentials))
    }

    fn delete(fqdn: &str) -> Result<(), Box<dyn error::Error>> {
        let entry = CredentialStore::keyring_entry(&fqdn);
        let result = entry.delete_password();
        match result {
            Ok(_) => Ok(()),
            Err(error) => return Err(Box::new(error)),
        }
    }

    fn update(fqdn: &str, credentials: &Credentials) -> Result<(), Box<dyn error::Error>> {
        let entry = CredentialStore::keyring_entry(&fqdn);
        let json = serde_json::to_string(&credentials)?;
        let result = entry.set_password(&json);
        match result {
            Ok(_) => Ok(()),
            Err(error) => return Err(Box::new(error)),
        }
    }
}
