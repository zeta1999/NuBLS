use bls12_381::{pairing, G1Affine, G2Affine};

use crate::keys::{PrivateKey, PublicKey};
use crate::traits::ThresholdSignature;

use std::convert::From;

/// This type represents the output of a Signature verification.
///
/// By representing signature verification in an `enum` like this, we are able
/// to construct a safe, misuse resistant API by forcing the user to handle
/// both cases of signature verification logic (Valid/Invalid). This prevents
/// silent failures that otherwise may be present when APIs return `bool`s.
#[derive(Debug, Eq, PartialEq)]
pub enum VerificationResult {
    Valid,
    Invalid,
}

/// A `Signature` is an Affine element of the G_2 group on the BLS12-381 curve.
#[derive(Debug, Eq, PartialEq)]
pub struct Signature(G2Affine);

impl Signature {
    /// Creates a `Signature` and returns it by signing the `message_element`
    /// with the provided `private_key`.
    ///
    /// The preferred API to sign messages is in `PrivateKey.sign`.
    ///
    /// Presently, the API for hashing to the G_2 group of BLS12-381 is not
    /// implemented (see https://github.com/nucypher/NuBLS/issues/1). Therefore,
    /// the message must be prehashed before verification and signing.
    ///
    /// TODO: Implement hash_to_curve
    pub(crate) fn new(private_key: &PrivateKey, message_element: &G2Affine) -> Signature {
        Signature((message_element * &private_key.0).into())
    }

    /// Attempts to verify the signature given a `message_element` and a `public_key`.
    /// Returns a `VerificationResult::Valid` if the `message_element` and `public_key`
    /// are correct, and a `VerificationResult::Invalid` if they are not.
    ///
    /// The preferred API to verify signatures is in `public_key.verify`.
    ///
    /// Presently, the API for hashing to the G_2 group of BLS12-381 is not
    /// implemented (see https://github.com/nucypher/NuBLS/issues/1). Therefore,
    /// the message must be prehashed before verification and signing.
    ///
    /// TODO: Implement hash_to_curve.
    pub(crate) fn verify(
        &self,
        public_key: &PublicKey,
        message_element: &G2Affine,
    ) -> VerificationResult {
        let c_1 = pairing(&public_key.0, &message_element);
        let c_2 = pairing(&G1Affine::generator(), &self.0);

        VerificationResult::from(c_1 == c_2)
    }
}

/// Implements Threshold BLS signatures on `Signature`.
///
/// We use Shamir's Secret Sharing scheme to share `n` fragments of a `PrivateKey`
/// where `m` fragments are needed to recover it.
/// For BLS threshold signatures, this translates to needing `m` signatures of
/// identical data to assemble the final `Signature`.
impl ThresholdSignature for Signature {
    /// Assembles a `Signature` from collected signature `fragments`.
    ///
    /// Note: The data signed by each of the fragment signatures must be identical,
    /// or else the assembled `Signature` will be invalid.
    ///
    /// This calculates the final signature by using Lagrange basis polynomials.
    fn assemble(fragments: &[Signature]) -> Signature {
        unimplemented!()
    }
}

impl From<bool> for VerificationResult {
    fn from(result: bool) -> Self {
        if result {
            VerificationResult::Valid
        } else {
            VerificationResult::Invalid
        }
    }
}
