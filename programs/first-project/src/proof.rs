use crate::error::ErrorCode;
use anchor_lang::prelude::*;

pub fn verify_location_proof(
    backend_admin: &Pubkey,
    message: &[u8],
    signature: &[u8; 64],
) -> Result<()> {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    let key_bytes = backend_admin.to_bytes();
    let verifying_key = VerifyingKey::from_bytes(&key_bytes)
        .map_err(|_| ErrorCode::InvalidPubkeyBytes)?;
    let sig = Signature::from_bytes(signature);
    verifying_key
        .verify(message, &sig)
        .map_err(|_| ErrorCode::InvalidLocationProof)?;
    Ok(())
}