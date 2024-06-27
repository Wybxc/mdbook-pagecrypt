#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use pbkdf2::pbkdf2_hmac;
use rand::Rng;
use sha2::Sha256;
use thiserror::Error;

/// Error type for [`PageCrypt`].
#[derive(Debug, Error)]
pub enum Error {
    /// Fail to encrypt payload
    #[error("Fail to encrypt")]
    EncryptError(#[from] aes_gcm::Error),
}

/// Result type for [`PageCrypt`].
pub type Result<T> = std::result::Result<T, Error>;

const HTML_TEMPLATE: &str = include_str!(concat!(env!("OUT_DIR"), "/decrypt.minify.html"));
const JS_TEMPLATE: &str = include_str!(concat!(env!("OUT_DIR"), "/decrypt.minify.js"));

const SALT_LEN: usize = 32;
const NONCE_LEN: usize = 12;
const HMAC_LEN: usize = 32;

/// Builder for [`PageCrypt`].
pub struct PageCryptBuilder {
    /// Password
    pub password: String,
    /// Number of rounds
    pub rounds: u32,
}

impl Default for PageCryptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PageCryptBuilder {
    /// Create a new [`PageCryptBuilder`].
    pub fn new() -> Self {
        Self {
            password: String::new(),
            rounds: 600_000,
        }
    }

    /// Set password.
    pub fn password(mut self, password: String) -> Self {
        self.password = password;
        self
    }

    /// Set number of rounds.
    pub fn rounds(mut self, rounds: u32) -> Self {
        self.rounds = rounds;
        self
    }

    /// Build [`PageCrypt`].
    pub fn build(self) -> Result<PageCrypt> {
        if self.rounds < 100_000 {
            log::warn!(
                "The specified number of password rounds ({}) is not secure. If possible, use at least 100_000 or more.",
                self.rounds
            )
        }

        let salt = getrandom::<SALT_LEN>();
        log::info!("Salt generated for password hashing.");

        let mut hmac = [0; HMAC_LEN];
        pbkdf2_hmac::<Sha256>(self.password.as_bytes(), &salt, self.rounds, &mut hmac);
        log::info!("Password hashed.");

        Ok(PageCrypt::from_hmac(self.rounds, salt, hmac))
    }
}

/// PageCrypt implementation.
pub struct PageCrypt {
    rounds: u32,
    salt: [u8; SALT_LEN],
    hmac: [u8; HMAC_LEN],
}

impl PageCrypt {
    /// Create a new [`PageCryptBuilder`].
    pub fn builder() -> PageCryptBuilder {
        PageCryptBuilder::new()
    }

    /// Create a new [`PageCrypt`] from hmac.
    pub fn from_hmac(rounds: u32, salt: [u8; SALT_LEN], hmac: [u8; HMAC_LEN]) -> Self {
        Self { rounds, salt, hmac }
    }

    /// Encrypt payload.
    pub fn encrypt_payload(&self, data: &[u8]) -> Result<Vec<u8>> {
        let nonce = getrandom::<NONCE_LEN>();
        let nonce = Nonce::from_slice(&nonce);
        log::info!("Nonce generated for encryption.");

        let key = Key::<Aes256Gcm>::from_slice(&self.hmac);
        let cipher = Aes256Gcm::new(key);
        let cipher_text = cipher.encrypt(nonce, data)?;
        log::info!("Payload encrypted.");

        // salt + iv + cipher_text
        let mut payload = Vec::with_capacity(SALT_LEN + NONCE_LEN + cipher_text.len());
        payload.extend_from_slice(&self.salt);
        payload.extend_from_slice(nonce);
        payload.extend_from_slice(&cipher_text);
        log::info!("Payload assembled.");

        Ok(payload)
    }

    /// Encrypt HTML.
    pub fn encrypt_html(&self, html: &[u8]) -> Result<String> {
        let encrypted = self.encrypt_payload(html)?;
        log::info!("HTML encrypted.");

        let encrypted = BASE64_STANDARD.encode(encrypted);
        let encrypted = urlencoding::encode(&encrypted);
        log::info!("HTML encoded.");

        let result = HTML_TEMPLATE
            .replacen("{{ rounds }}", &self.rounds.to_string(), 1)
            .replacen("{{ encrypted }}", &encrypted, 1);
        log::info!("HTML rendered.");

        Ok(result)
    }

    /// Encrypt JS.
    pub fn encrypt_js(&self, js: &[u8]) -> Result<String> {
        let encrypted = self.encrypt_payload(js)?;
        log::info!("JS encrypted.");

        let encrypted = BASE64_STANDARD.encode(encrypted);
        log::info!("JS encoded.");

        let result = JS_TEMPLATE.replacen("{{ encrypted }}", &encrypted, 1);
        log::info!("JS rendered.");

        Ok(result)
    }
}

fn getrandom<const N: usize>() -> [u8; N] {
    let mut buf = [0; N];
    if getrandom::getrandom(&mut buf).is_err() {
        log::warn!(
            "Fail to generate random from system entropy. Using pseudo-random generator instead."
        );
        rand::thread_rng().fill(buf.as_mut_slice());
    }
    buf
}
