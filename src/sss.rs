use crate::utils;
use crate::utils::SSError;
use num::BigInt;

pub struct SecretSharing {
    threshold: u64,
    total: u64,
    charset: Charset,
    prime: Option<BigInt>,
}

impl SecretSharing {
    pub fn new(threshold: u64, total: u64, charset: Charset) -> Self {
        SecretSharing {
            threshold,
            total,
            charset,
            prime: None,
        }
    }

    #[inline]
    pub fn threshold(&self) -> u64 {
        self.threshold
    }

    #[inline]
    pub fn total(&self) -> u64 {
        self.total
    }

    #[inline]
    pub fn charset(&self) -> &str {
        self.charset.charset_str()
    }

    #[inline]
    fn set_prime(&mut self, prime: BigInt) {
        self.prime = Some(prime)
    }

    #[inline]
    pub fn prime(&self) -> Result<&BigInt, SSError> {
        self.prime.as_ref().ok_or(SSError::PrimeNotSet)
    }

    pub fn generate_shares(&mut self, secret: &str) -> Result<Vec<String>, SSError> {
        if self.threshold() < 2 {
            return Err(SSError::LowThreshold);
        }
        if self.threshold() > self.total() {
            return Err(SSError::HighThreshold);
        }
        let secret_int = utils::charset_repr_to_int(secret, self.charset())?;
        self.set_prime(utils::next_prime(&secret_int)?);

        let points =
            utils::secret_int_to_points(secret_int, self.threshold(), self.total(), self.prime()?);

        let shares: Result<Vec<_>, SSError> = points
            .iter()
            .map(|point| utils::point_to_share_str(point, self.charset()))
            .collect();
        shares
    }

    pub fn reconstruct_secret(&self, shares: &[String]) -> Result<String, SSError> {

        if (shares.len() as u64) < self.threshold() {
            return Err(SSError::InsufficientShares);
        }

        let point_shares: Result<Vec<_>, SSError> = shares
            .iter()
            .map(|share| utils::share_str_to_point(share.as_str(), self.charset()))
            .collect();

        let secret_int = utils::points_to_secret_int(point_shares?, self.prime()?)?;
        utils::int_to_charset_repr(secret_int, self.charset())
    }
}

/// Possible charsets for secret.
pub enum Charset {
    Hex,
    Alphanumeric,
    Base58,
}

impl Charset {
    pub fn charset_str(&self) -> &str {
        match self {
            Charset::Hex => "0123456789abcdef",
            Charset::Alphanumeric => {
                "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            }
            Charset::Base58 => {
                "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
            }
        }
    }
}
