use anyhow::{anyhow, Ok, Result};

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng as ChaOsRng},
    ChaCha20Poly1305, Error as ChaError, KeySizeUser, Nonce,
};
use core::result::Result as CoreResult;
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::{collections::HashMap, io::Read};

use crate::cli::text::TextSignMethod;

use super::genpass::process_genpass;

pub type KeyOutput = HashMap<&'static str, Vec<u8>>;

pub trait TextSigner {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: ed25519_dalek::SigningKey,
}

pub struct Ed25519Verifier {
    key: ed25519_dalek::VerifyingKey,
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        // cloned
        Ok(hash.as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes() == sig)
    }
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, msg: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        msg.read_to_end(&mut buf)?;
        // using trait function must include it
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerifier for Ed25519Verifier {
    fn verify(&self, msg: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        msg.read_to_end(&mut buf)?;
        // NOTE
        let sig = (&sig[..64]).try_into()?;
        let sig = Signature::from_bytes(sig);
        Ok(self.key.verify_strict(&buf, &sig).is_ok())
    }
}

impl Blake3 {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self { key })
    }

    fn generate() -> Result<KeyOutput> {
        let key = process_genpass(32, true, true, true, true)?;
        let mut map = HashMap::new();
        map.insert("blake3.txt", key.as_bytes().to_vec());
        Ok(map)
    }
}

impl Ed25519Signer {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    pub fn new(key: &[u8; 32]) -> Self {
        let key = SigningKey::from_bytes(key);
        Self { key }
    }

    fn generate() -> Result<KeyOutput> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk: VerifyingKey = (&sk).into();
        let mut map = HashMap::new();
        map.insert("ed25519.sk", sk.to_bytes().to_vec());
        map.insert("ed25519.pk", pk.to_bytes().to_vec());
        Ok(map)
    }
}

impl Ed25519Verifier {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        Ok(Self { key })
    }
}

pub fn process_text_sign(
    msg: &mut dyn Read,
    key: &[u8],
    method: TextSignMethod,
) -> Result<Vec<u8>> {
    let signer: Box<dyn TextSigner> = match method {
        TextSignMethod::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignMethod::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
    };

    signer.sign(msg)
}

// verify signature with session key (blake3) or public key (ed25519)
pub fn process_text_verify(
    msg: &mut dyn Read,
    key: &[u8],
    sig: &[u8],
    method: TextSignMethod,
) -> Result<bool> {
    let verifier: Box<dyn TextVerifier> = match method {
        TextSignMethod::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignMethod::Ed25519 => Box::new(Ed25519Verifier::try_new(key)?),
    };

    verifier.verify(msg, sig)
}

pub fn process_text_key_generate(method: TextSignMethod) -> Result<KeyOutput> {
    match method {
        TextSignMethod::Blake3 => Blake3::generate(),
        TextSignMethod::Ed25519 => Ed25519Signer::generate(),
    }
}

pub fn process_text_encrypt(reader: &mut dyn Read, key: &[u8], encrypt: bool) -> Result<Vec<u8>> {
    let mut text = Vec::new();
    reader.read_to_end(&mut text)?;

    let chacha = ChaCha20Poly1305::new_from_slice(key).map_err(|_| {
        anyhow!(
            "invalid key length: require {}, got {}",
            ChaCha20Poly1305::key_size(),
            key.len()
        )
    })?;

    if encrypt {
        let nonce = ChaCha20Poly1305::generate_nonce(&mut ChaOsRng);
        let result: CoreResult<Vec<u8>, ChaError> = chacha.encrypt(&nonce, text.as_slice());
        let ciphertext = result.map_err(|e| anyhow!(e.to_string()))?;
        Ok([nonce.as_slice(), &ciphertext].concat())
    } else {
        let nsize = Nonce::default().len();
        if text.len() <= nsize {
            return Err(anyhow!(
                "invalid message length: require > {}, got {}",
                nsize,
                text.len()
            ));
        }
        let nonce = Nonce::from_slice(&text[..nsize]);
        let result: CoreResult<Vec<u8>, ChaError> = chacha.decrypt(nonce, &text[nsize..]);
        let plaintext = result.map_err(|e| anyhow!(e.to_string()))?;
        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const KEY_BLAKE3: &[u8] = include_bytes!("../../fixtures/blake3.txt");
    const PKEY_ED25519: &[u8] = include_bytes!("../../fixtures/ed25519.pk");
    const SKEY_ED25519: &[u8] = include_bytes!("../../fixtures/ed25519.sk");

    #[test]
    fn t_blake3_sign_verify() -> Result<()> {
        let msg = "hello";
        let sig = process_text_sign(&mut msg.as_bytes(), KEY_BLAKE3, TextSignMethod::Blake3)?;
        let ok = process_text_verify(
            &mut msg.as_bytes(),
            KEY_BLAKE3,
            &sig,
            TextSignMethod::Blake3,
        )?;
        assert!(ok);
        Ok(())
    }

    #[test]
    fn t_ed25519_sign_verify() -> Result<()> {
        let msg = "hello";
        let sig = process_text_sign(&mut msg.as_bytes(), SKEY_ED25519, TextSignMethod::Ed25519)?;
        let ok = process_text_verify(
            &mut msg.as_bytes(),
            PKEY_ED25519,
            &sig,
            TextSignMethod::Ed25519,
        )?;
        assert!(ok);
        Ok(())
    }

    fn encrypt_decrypt(msg: &[u8], key: &[u8]) -> anyhow::Result<Vec<u8>> {
        let ciphertext = process_text_encrypt(&mut &msg[0..], key, true)?;
        let plaintext = process_text_encrypt(&mut ciphertext.as_slice(), key, false)?;
        Ok(plaintext)
    }

    #[test]
    fn t_encrypt_decrypt() -> Result<()> {
        let msg = b"hello, world";
        let key32 = b"01234567890123456789012345678901";
        let key16 = &key32[..16];

        let plaintext = encrypt_decrypt(msg, key32)?;
        assert_eq!(msg, plaintext.as_slice());

        let result = encrypt_decrypt(msg, key16);
        assert!(result.is_err());

        Ok(())
    }
}
