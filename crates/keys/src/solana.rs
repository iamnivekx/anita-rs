use solana_sdk::bs58;
use solana_sdk::signature::{Keypair, Signer};

use crate::{Chain, KeypairStrategy, Keypairs};

pub struct SolanaKeyPair;

impl SolanaKeyPair {
    pub fn new() -> Self {
        SolanaKeyPair
    }

    pub fn to_keypairs(keypair: Keypair) -> Keypairs {
        let secret = keypair.to_base58_string();
        let pubkey = keypair.pubkey();
        let address = bs58::encode(pubkey).into_string();
        Keypairs { chain: Chain::SOLANA, secret, pubkey: address.clone(), address }
    }
}

impl KeypairStrategy for SolanaKeyPair {
    fn chain(&self) -> Chain {
        Chain::SOLANA
    }

    fn generate(&self) -> Keypairs {
        let keypair = Keypair::new();
        Self::to_keypairs(keypair)
    }

    fn from_secret(&self, secret: &str) -> Keypairs {
        let keypair = Keypair::from_base58_string(secret);
        Self::to_keypairs(keypair)
    }

    fn sign(&self, secret: &str, message: &[u8]) -> String {
        let keypair = Keypair::from_base58_string(secret);
        let signature = keypair.sign_message(message);
        bs58::encode(signature).into_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        let strategy = Box::new(SolanaKeyPair);
        let pairs = strategy.generate();
        assert!(pairs.pubkey.eq_ignore_ascii_case(pairs.address.as_str()));
    }
}
