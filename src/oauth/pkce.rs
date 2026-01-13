use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use sha2::{Digest, Sha256};

const VERIFIER_LENGTH: usize = 128;

pub struct PkceChallenge {
    pub verifier: String,
    pub challenge: String,
}

impl PkceChallenge {
    pub fn generate() -> Self {
        let verifier = generate_verifier();
        let challenge = generate_challenge(&verifier);

        Self {
            verifier,
            challenge,
        }
    }
}

fn generate_verifier() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";

    let mut rng = rand::thread_rng();

    (0..VERIFIER_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

fn generate_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();

    URL_SAFE_NO_PAD.encode(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifier_has_correct_length() {
        let pkce = PkceChallenge::generate();
        assert_eq!(pkce.verifier.len(), VERIFIER_LENGTH);
    }

    #[test]
    fn verifier_uses_valid_characters() {
        let pkce = PkceChallenge::generate();
        let valid_chars: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";

        for c in pkce.verifier.chars() {
            assert!(valid_chars.contains(c));
        }
    }

    #[test]
    fn challenge_is_base64url_encoded() {
        let pkce = PkceChallenge::generate();
        assert!(URL_SAFE_NO_PAD.decode(&pkce.challenge).is_ok());
    }

    #[test]
    fn challenge_is_sha256_of_verifier() {
        let pkce = PkceChallenge::generate();
        let expected = generate_challenge(&pkce.verifier);
        assert_eq!(pkce.challenge, expected);
    }
}
