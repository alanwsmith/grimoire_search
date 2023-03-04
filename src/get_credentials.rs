use keyring::error::Error;
use keyring::Entry;
use std::env;

pub fn get_credentials(
    envkey: &str, credkey: &str, creduser: &str,
) -> Result<String, Error> {
    if let Ok(value) = env::var(envkey) {
        Ok(value)
    }
    else {
        if let Ok(entry) =
            Entry::new(credkey, creduser)
        {
            if let Ok(value) = entry.get_password()
            {
                Ok(value)
            }
            else {
                Err(Error::NoEntry)
            }
        }
        else {
            Err(Error::NoEntry)
        }
    }
}
