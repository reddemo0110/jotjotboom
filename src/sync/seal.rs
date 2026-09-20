// SPDX-License-Identifier: GPL-3.0-only

//! End-to-end encryption for sync: what turns "the server never reads a
//! note" from a promise into arithmetic. Nothing here is home-made — it is
//! glue around Argon2id (passphrase → key), XChaCha20-Poly1305 (sealing)
//! and blake3 (sub-keys, hidden names). DECISIONS.md, 2026-09-20.
//!
//! - One passphrase per account. [`KeyFile`] (salt, cost, a check value)
//!   sits on the backend in the clear — it has to, a new device needs it
//!   before it can derive anything — and is useless without the passphrase.
//! - Short things (note envelopes, file meta) are sealed whole:
//!   `jjb1.` + base64(nonce ‖ ciphertext).
//! - Files are sealed as a stream of 64 KiB chunks, so a 200 MB PDF never
//!   sits in memory and a truncated or reordered upload fails to open.
//! - File keys become a keyed hash of the path, so the backend cannot test
//!   guesses ("is there an assets/passport.jpg?") either.
//!
//! The local notes folder is never encrypted: losing the passphrase loses
//! the cloud copy, not the notes.

// Only `probe` is wired into the app so far; the rest waits for the
// passphrase field in Options → Sync.
#![allow(dead_code)]

use anyhow::{Context, Result, anyhow};
use base64::Engine;
use base64::engine::general_purpose::STANDARD_NO_PAD as B64;
use chacha20poly1305::aead::stream::{DecryptorBE32, EncryptorBE32};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::path::Path;

const TEXT_PREFIX: &str = "jjb1.";
const FILE_MAGIC: &[u8; 6] = b"JJBS1\n";
const CHUNK: usize = 64 * 1024;
const TAG: usize = 16;
const CHECK: &str = "jotjotboom";

/// The account's master key, derived from the passphrase. Wiped on drop.
pub struct Key {
    data: [u8; 32],
    names: [u8; 32],
}

impl Drop for Key {
    fn drop(&mut self) {
        self.data.fill(0);
        self.names.fill(0);
    }
}

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Key(…)")
    }
}

/// What a new device needs before it can derive the key. Public.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyFile {
    pub v: u32,
    pub kdf: String,
    /// Argon2id memory in KiB, passes, lanes.
    pub m: u32,
    pub t: u32,
    pub p: u32,
    pub salt: String,
    /// A known word sealed with the key: tells a wrong passphrase from a
    /// right one without touching a note.
    pub check: String,
}

fn random<const N: usize>() -> Result<[u8; N]> {
    let mut buf = [0u8; N];
    getrandom::fill(&mut buf).map_err(|e| anyhow!("no randomness available: {e}"))?;
    Ok(buf)
}

fn derive(passphrase: &str, salt: &[u8], m: u32, t: u32, p: u32) -> Result<Key> {
    let params = argon2::Params::new(m, t, p, Some(32)).map_err(|e| anyhow!("bad key settings: {e}"))?;
    let argon = argon2::Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    let mut master = [0u8; 32];
    argon
        .hash_password_into(passphrase.as_bytes(), salt, &mut master)
        .map_err(|e| anyhow!("deriving the key: {e}"))?;
    let key = Key {
        data: blake3::derive_key("JotJotBoom 2026-09-20 sync data", &master),
        names: blake3::derive_key("JotJotBoom 2026-09-20 sync names", &master),
    };
    master.fill(0);
    Ok(key)
}

impl KeyFile {
    /// A fresh account: new salt, today's cost settings.
    pub fn create(passphrase: &str) -> Result<(KeyFile, Key)> {
        // 64 MiB, 3 passes: about half a second on a laptop, once per
        // sign-in — and the same again for every guess.
        Self::create_with(passphrase, 64 * 1024, 3, 1)
    }

    pub fn create_with(passphrase: &str, m: u32, t: u32, p: u32) -> Result<(KeyFile, Key)> {
        if passphrase.chars().count() < 8 {
            anyhow::bail!("the passphrase needs at least 8 characters");
        }
        let salt: [u8; 16] = random()?;
        let key = derive(passphrase, &salt, m, t, p)?;
        let file = KeyFile {
            v: 1,
            kdf: "argon2id".into(),
            m,
            t,
            p,
            salt: B64.encode(salt),
            check: key.seal_text(CHECK)?,
        };
        Ok((file, key))
    }

    /// The key for this account, or an error for the wrong passphrase.
    pub fn unlock(&self, passphrase: &str) -> Result<Key> {
        if self.v != 1 || self.kdf != "argon2id" {
            anyhow::bail!("this account was encrypted by a newer JotJotBoom");
        }
        let salt = B64.decode(&self.salt).context("reading the key settings")?;
        let key = derive(passphrase, &salt, self.m, self.t, self.p)?;
        match key.open_text(&self.check) {
            Ok(word) if word == CHECK => Ok(key),
            _ => Err(anyhow!("that is not the passphrase")),
        }
    }

    pub fn encode(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    pub fn decode(text: &str) -> Result<Self> {
        serde_json::from_str(text).context("reading the key settings")
    }
}

impl Key {
    /// Raw bytes for the keyring (so the passphrase is asked once per
    /// device, not once per launch).
    pub fn to_secret(&self) -> String {
        let mut both = [0u8; 64];
        both[..32].copy_from_slice(&self.data);
        both[32..].copy_from_slice(&self.names);
        let out = B64.encode(both);
        both.fill(0);
        out
    }

    pub fn from_secret(secret: &str) -> Result<Self> {
        let mut bytes = B64.decode(secret.trim()).context("reading the stored key")?;
        if bytes.len() != 64 {
            anyhow::bail!("the stored key is damaged");
        }
        let mut key = Key {
            data: [0; 32],
            names: [0; 32],
        };
        key.data.copy_from_slice(&bytes[..32]);
        key.names.copy_from_slice(&bytes[32..]);
        bytes.fill(0);
        Ok(key)
    }

    /// What the backend sees instead of a path.
    pub fn name(&self, path: &str) -> String {
        blake3::keyed_hash(&self.names, path.as_bytes()).to_hex().to_string()
    }

    pub fn seal_text(&self, plain: &str) -> Result<String> {
        let nonce: [u8; 24] = random()?;
        let cipher = XChaCha20Poly1305::new((&self.data).into());
        let sealed = cipher
            .encrypt(XNonce::from_slice(&nonce), plain.as_bytes())
            .map_err(|_| anyhow!("sealing failed"))?;
        let mut out = nonce.to_vec();
        out.extend(sealed);
        Ok(format!("{TEXT_PREFIX}{}", B64.encode(out)))
    }

    pub fn open_text(&self, sealed: &str) -> Result<String> {
        let body = sealed
            .strip_prefix(TEXT_PREFIX)
            .ok_or_else(|| anyhow!("not encrypted (or by a newer JotJotBoom)"))?;
        let bytes = B64.decode(body).context("reading sealed text")?;
        if bytes.len() < 24 + TAG {
            anyhow::bail!("sealed text is too short");
        }
        let (nonce, ct) = bytes.split_at(24);
        let cipher = XChaCha20Poly1305::new((&self.data).into());
        let plain = cipher
            .decrypt(XNonce::from_slice(nonce), ct)
            .map_err(|_| anyhow!("could not open it: wrong key, or it was tampered with"))?;
        String::from_utf8(plain).context("sealed text is not text")
    }

    /// Seal `src` into `dest`, chunk by chunk.
    pub fn seal_file(&self, src: &mut dyn Read, dest: &Path) -> Result<()> {
        let nonce: [u8; 19] = random()?;
        let cipher = XChaCha20Poly1305::new((&self.data).into());
        let mut enc = EncryptorBE32::from_aead(cipher, (&nonce).into());
        let mut out = std::io::BufWriter::new(std::fs::File::create(dest)?);
        out.write_all(FILE_MAGIC)?;
        out.write_all(&nonce)?;
        // One chunk of read-ahead: the last chunk is sealed differently,
        // which is what makes a truncated file fail to open.
        let mut current = read_chunk(src, CHUNK)?;
        loop {
            let next = read_chunk(src, CHUNK)?;
            if next.is_empty() {
                let sealed = enc.encrypt_last(current.as_slice()).map_err(|_| anyhow!("sealing failed"))?;
                out.write_all(&sealed)?;
                break;
            }
            let sealed = enc.encrypt_next(current.as_slice()).map_err(|_| anyhow!("sealing failed"))?;
            out.write_all(&sealed)?;
            current = next;
        }
        out.flush()?;
        Ok(())
    }

    /// Open a sealed file into `dest`. On any failure `dest` is removed.
    pub fn open_file(&self, src: &Path, dest: &Path) -> Result<()> {
        let result = self.open_file_inner(src, dest);
        if result.is_err() {
            std::fs::remove_file(dest).ok();
        }
        result
    }

    fn open_file_inner(&self, src: &Path, dest: &Path) -> Result<()> {
        let broken = || anyhow!("could not open the file: wrong key, or it was damaged on the way");
        let mut input = std::io::BufReader::new(std::fs::File::open(src)?);
        let mut magic = [0u8; 6];
        let mut nonce = [0u8; 19];
        input.read_exact(&mut magic).map_err(|_| broken())?;
        if &magic != FILE_MAGIC {
            anyhow::bail!("not encrypted (or by a newer JotJotBoom)");
        }
        input.read_exact(&mut nonce).map_err(|_| broken())?;
        let cipher = XChaCha20Poly1305::new((&self.data).into());
        let mut dec = DecryptorBE32::from_aead(cipher, (&nonce).into());
        let mut out = std::io::BufWriter::new(std::fs::File::create(dest)?);
        let mut current = read_chunk(&mut input, CHUNK + TAG)?;
        loop {
            let next = read_chunk(&mut input, CHUNK + TAG)?;
            if next.is_empty() {
                let plain = dec.decrypt_last(current.as_slice()).map_err(|_| broken())?;
                out.write_all(&plain)?;
                break;
            }
            let plain = dec.decrypt_next(current.as_slice()).map_err(|_| broken())?;
            out.write_all(&plain)?;
            current = next;
        }
        out.flush()?;
        Ok(())
    }
}

/// Up to `size` bytes; short only at the end of the input.
fn read_chunk(src: &mut dyn Read, size: usize) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; size];
    let mut filled = 0;
    while filled < size {
        let n = src.read(&mut buf[filled..])?;
        if n == 0 {
            break;
        }
        filled += n;
    }
    buf.truncate(filled);
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Cheap settings: the real ones take half a second per derivation.
    pub fn test_key(passphrase: &str) -> (KeyFile, Key) {
        KeyFile::create_with(passphrase, 64, 1, 1).unwrap()
    }

    #[test]
    fn the_passphrase_unlocks_and_nothing_else_does() {
        let (file, key) = test_key("correct horse battery");
        let again = KeyFile::decode(&file.encode()).unwrap().unlock("correct horse battery").unwrap();
        assert_eq!(again.open_text(&key.seal_text("hello").unwrap()).unwrap(), "hello");
        let err = file.unlock("correct horse batterz").unwrap_err().to_string();
        assert_eq!(err, "that is not the passphrase");
        assert!(KeyFile::create("short").is_err());
        // The keyring round trip.
        let stored = Key::from_secret(&key.to_secret()).unwrap();
        assert_eq!(stored.name("assets/a.jpg"), key.name("assets/a.jpg"));
        assert!(Key::from_secret("bm9wZQ").is_err());
    }

    #[test]
    fn sealed_text_hides_and_detects_tampering() {
        let (_, key) = test_key("correct horse battery");
        let (_, other) = test_key("correct horse battery");
        let sealed = key.seal_text("# Secret plans\n").unwrap();
        assert!(sealed.starts_with("jjb1.") && !sealed.contains("Secret"));
        assert_ne!(sealed, key.seal_text("# Secret plans\n").unwrap(), "fresh nonce each time");
        assert_eq!(key.open_text(&sealed).unwrap(), "# Secret plans\n");
        // Same passphrase, different salt: a different key.
        assert!(other.open_text(&sealed).is_err());
        let mut bent = sealed.clone().into_bytes();
        let last = bent.len() - 1;
        bent[last] = if bent[last] == b'A' { b'B' } else { b'A' };
        assert!(key.open_text(&String::from_utf8(bent).unwrap()).is_err());
        assert!(key.open_text("{\"v\":1}").is_err(), "plain text is refused, not passed through");
    }

    #[test]
    fn names_are_keyed() {
        let (_, key) = test_key("correct horse battery");
        let (_, other) = test_key("correct horse battery");
        assert_eq!(key.name("assets/a.jpg").len(), 64);
        assert_ne!(key.name("assets/a.jpg"), key.name("assets/b.jpg"));
        assert_ne!(key.name("assets/a.jpg"), other.name("assets/a.jpg"));
        assert_ne!(key.name("assets/a.jpg"), crate::sync::files::key_for("assets/a.jpg"));
    }

    #[test]
    fn files_round_trip_at_every_awkward_size() {
        let (_, key) = test_key("correct horse battery");
        let tmp = tempfile::tempdir().unwrap();
        let sealed = tmp.path().join("sealed");
        let opened = tmp.path().join("opened");
        for size in [0, 1, CHUNK - 1, CHUNK, CHUNK + 1, 2 * CHUNK, 2 * CHUNK + 7] {
            let plain: Vec<u8> = (0..size).map(|i| (i % 251) as u8).collect();
            key.seal_file(&mut plain.as_slice(), &sealed).unwrap();
            let on_wire = std::fs::read(&sealed).unwrap();
            assert!(size < 64 || !on_wire.windows(32).any(|w| w == &plain[..32]), "size {size}");
            key.open_file(&sealed, &opened).unwrap();
            assert_eq!(std::fs::read(&opened).unwrap(), plain, "size {size}");
        }
    }

    #[test]
    fn damaged_files_do_not_open() {
        let (_, key) = test_key("correct horse battery");
        let (_, other) = test_key("another passphrase");
        let tmp = tempfile::tempdir().unwrap();
        let sealed = tmp.path().join("sealed");
        let opened = tmp.path().join("opened");
        let plain = vec![7u8; 3 * CHUNK];
        key.seal_file(&mut plain.as_slice(), &sealed).unwrap();
        let good = std::fs::read(&sealed).unwrap();

        assert!(other.open_file(&sealed, &opened).is_err());
        // Truncated at a chunk boundary: every chunk is genuine, but the
        // last one was not sealed as last.
        std::fs::write(&sealed, &good[..6 + 19 + 2 * (CHUNK + TAG)]).unwrap();
        assert!(key.open_file(&sealed, &opened).is_err());
        assert!(!opened.exists(), "no half-opened file left behind");
        // Two chunks swapped.
        let mut swapped = good.clone();
        let (a, b) = (6 + 19, 6 + 19 + CHUNK + TAG);
        for i in 0..CHUNK + TAG {
            swapped.swap(a + i, b + i);
        }
        std::fs::write(&sealed, &swapped).unwrap();
        assert!(key.open_file(&sealed, &opened).is_err());
        // One flipped bit.
        let mut flipped = good.clone();
        flipped[100] ^= 1;
        std::fs::write(&sealed, &flipped).unwrap();
        assert!(key.open_file(&sealed, &opened).is_err());
        // A plain file is refused, not passed through.
        std::fs::write(&sealed, b"just a jpeg").unwrap();
        assert!(key.open_file(&sealed, &opened).is_err());
    }
}
