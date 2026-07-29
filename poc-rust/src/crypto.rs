use anyhow::Context;
use num_bigint_dig::{BigInt, BigUint};
use num_integer::Integer;
use rand::RngCore;
use rsa::traits::{PrivateKeyParts, PublicKeyParts};
use sha2::{Digest, Sha256};

pub type RsaPrivateKey = rsa::RsaPrivateKey;
pub type RsaPublicKey = rsa::RsaPublicKey;

pub const SUITE_ID: &str = "BLIND_SIG_RSA_FDH_3072_SHA256";

pub fn domain(election_id: &str) -> Vec<u8> {
    format!("{SUITE_ID}:{election_id}").into_bytes()
}

fn message_representative(ballot_cbor: &[u8], election_id: &str, _modulus_bits: usize) -> BigUint {
    let mut hasher = Sha256::new();
    hasher.update(domain(election_id));
    hasher.update(ballot_cbor);
    let digest = hasher.finalize();
    BigUint::from_bytes_be(&digest)
}

fn random_in_zn_star(n: &BigUint, rng: &mut impl RngCore) -> BigUint {
    let size = (n.bits() + 7) / 8;
    loop {
        let mut buf = vec![0u8; size];
        rng.fill_bytes(&mut buf);
        let r = BigUint::from_bytes_be(&buf) % n;
        if r != BigUint::from(0u8) && r.gcd(n) == BigUint::from(1u8) {
            return r;
        }
    }
}

fn mod_inverse(a: &BigUint, n: &BigUint) -> Option<BigUint> {
    let a_bi = BigInt::from(a.clone());
    let n_bi = BigInt::from(n.clone());
    let egcd = a_bi.extended_gcd(&n_bi);
    if egcd.gcd != BigInt::from(1) {
        return None;
    }
    let mut inv = egcd.x % &n_bi;
    if inv < BigInt::from(0) {
        inv += &n_bi;
    }
    inv.to_biguint()
}

pub struct BlindingFactors {
    pub r: BigUint,
    pub blinded: BigUint,
}

pub fn blind(
    pub_key: &RsaPublicKey,
    ballot_cbor: &[u8],
    election_id: &str,
    rng: &mut impl RngCore,
) -> BlindingFactors {
    let n = pub_key.n();
    let e = pub_key.e();
    let m = message_representative(ballot_cbor, election_id, n.bits());
    let r = random_in_zn_star(n, rng);
    let r_e = r.modpow(e, n);
    let blinded = (&m * &r_e) % n;
    BlindingFactors { r, blinded }
}

pub fn sign(priv_key: &RsaPrivateKey, blinded: &BigUint) -> BigUint {
    let n = priv_key.n();
    let d = priv_key.d();
    blinded.modpow(d, n)
}

pub fn unblind(
    pub_key: &RsaPublicKey,
    blind_signature: &BigUint,
    r: &BigUint,
) -> anyhow::Result<BigUint> {
    let n = pub_key.n();
    let r_inv = mod_inverse(r, n).context("blinding factor not invertible")?;
    Ok((blind_signature * r_inv) % n)
}

pub fn verify(
    pub_key: &RsaPublicKey,
    ballot_cbor: &[u8],
    election_id: &str,
    signature: &BigUint,
) -> bool {
    let n = pub_key.n();
    let e = pub_key.e();
    let m = message_representative(ballot_cbor, election_id, n.bits());
    signature.modpow(e, n) == m
}

pub fn serialize_public_key(key: &RsaPublicKey) -> anyhow::Result<Vec<u8>> {
    use rsa::pkcs1::EncodeRsaPublicKey;
    Ok(key.to_pkcs1_der()?.as_bytes().to_vec())
}

pub fn deserialize_public_key(bytes: &[u8]) -> anyhow::Result<RsaPublicKey> {
    use rsa::pkcs1::DecodeRsaPublicKey;
    Ok(RsaPublicKey::from_pkcs1_der(bytes)?)
}

pub fn generate_key() -> anyhow::Result<RsaPrivateKey> {
    let mut rng = rand::thread_rng();
    Ok(RsaPrivateKey::new(&mut rng, 3072)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blind_signature_roundtrip() {
        let priv_key = generate_key().unwrap();
        let pub_key = priv_key.to_public_key();
        let election_id = "test-election";
        let ballot = b"some cbor ballot";

        let mut rng = rand::thread_rng();
        let blinded = blind(&pub_key, ballot, election_id, &mut rng);
        let signed = sign(&priv_key, &blinded.blinded);
        let unblinded = unblind(&pub_key, &signed, &blinded.r).unwrap();

        assert!(verify(&pub_key, ballot, election_id, &unblinded));
    }

    #[test]
    fn verify_rejects_wrong_ballot() {
        let priv_key = generate_key().unwrap();
        let pub_key = priv_key.to_public_key();
        let election_id = "test-election";
        let ballot = b"some cbor ballot";

        let mut rng = rand::thread_rng();
        let blinded = blind(&pub_key, ballot, election_id, &mut rng);
        let signed = sign(&priv_key, &blinded.blinded);
        let unblinded = unblind(&pub_key, &signed, &blinded.r).unwrap();

        assert!(!verify(&pub_key, b"different ballot", election_id, &unblinded));
    }
}
