
use num_bigint::BigInt;
use num_traits::{One, Zero, ToPrimitive};

const G: i128 = 3; 
const P: i128 = 170141183460469231731687303715884105727; 

#[derive(Debug, Clone)]
pub struct Rational {
    pub num: BigInt,
    pub den: BigInt,
}

#[derive(Debug)]
pub struct Share {
    pub base: u32,
    pub value: String,
    pub x: BigInt,
    pub y: BigInt,
}

impl Rational {
    pub fn new(n: BigInt, d: BigInt) -> Self {
        Self { num: n, den: d }.reduce()
    }

    fn reduce(self) -> Self {
        let gcd = gcd(&self.num, &self.den);
        Self {
            num: self.num / gcd.clone(),
            den: self.den / gcd,
        }
    }
}

impl std::ops::Mul for Rational {
    type Output = Rational;

    fn mul(self, other: Rational) -> Rational {
        Rational::new(self.num * other.num, self.den * other.den).reduce()
    }
}

impl std::ops::Add for Rational {
    type Output = Rational;

    fn add(self, other: Rational) -> Rational {
        Rational::new(
            &self.num * &other.den + &self.den * &other.num,
            &self.den * &other.den,
        )
        .reduce()
    }
}

fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    let mut a = a.clone();
    let mut b = b.clone();
    while !b.is_zero() {
        let t = b.clone();
        b = &a % &b;
        a = t;
    }
    a
}

fn convert_to_decimal(value: &str, base: u32) -> Option<BigInt> {
    let mut result = BigInt::zero();
    let base_big = BigInt::from(base);

    for c in value.chars() {
        let digit = match c {
            '0'..='9' => (c as u32 - '0' as u32),
            'a'..='f' => (c as u32 - 'a' as u32 + 10),
            'A'..='F' => (c as u32 - 'A' as u32 + 10),
            _ => return None,
        };

        if digit >= base {
            return None;
        }

        result = result * &base_big + BigInt::from(digit);
    }

    Some(result)
}


pub fn convert_from_decimal(mut num: BigInt, base: u32) -> String {
    if num.is_zero() {
        return "0".to_string();
    }

    let base_big = BigInt::from(base);
    let mut result = String::new();

    while !num.is_zero() {
        let digit = (&num % &base_big).to_u32().unwrap();
        let char = match digit {
            0..=9 => (digit as u8 + b'0') as char,
            10..=35 => ((digit - 10) as u8 + b'a') as char,
            _ => panic!("Invalid digit"),
        };
        result.push(char);
        num /= &base_big;
    }

    result.chars().rev().collect()
}

pub fn parse_shares(input: &str) -> Result<Vec<Share>, Box<dyn std::error::Error>> {
    let data: serde_json::Value = serde_json::from_str(input)?;
    let mut shares = Vec::new();

    
    let n = data["keys"]["n"].as_u64().unwrap() as usize;
    let k = data["keys"]["k"].as_u64().unwrap() as usize;

    for i in 1..=n {
        let key = i.to_string();
        if let Some(share) = data.get(&key) {
            let base = share["base"].as_str().unwrap().parse::<u32>()?;
            let value = share["value"].as_str().unwrap();

            // Convert value from its base to decimal
            if let Some(y) = convert_to_decimal(value, base) {
                shares.push(Share {
                    base,
                    value: value.to_string(),
                    x: BigInt::from(i),
                    y,
                });
            }
        }
    }

    Ok(shares)
}

pub fn generate_secret(x: &[BigInt], y: &[BigInt], m: usize) -> BigInt {
    let mut ans = Rational::new(BigInt::zero(), BigInt::one());

    for i in 0..m {
        let mut l = Rational::new(y[i].clone(), BigInt::one());
        for j in 0..m {
            if i != j {
                let temp = Rational::new(-x[j].clone(), &x[i] - &x[j]);
                l = l * temp;
            }
        }
        ans = ans + l;
    }

    ans.num
}

pub fn recover_secret(shares: &[Share], k: usize) -> Option<BigInt> {
    if shares.len() < k {
        println!("Not enough shares to recover the secret.");
        return None;
    }

    let x: Vec<BigInt> = shares.iter().take(k).map(|s| s.x.clone()).collect();
    let y: Vec<BigInt> = shares.iter().take(k).map(|s| s.y.clone()).collect();

    Some(generate_secret(&x, &y, k))
}


pub fn generate_commitments(poly: &Vec<i128>) -> Vec<i128> {
    let mut commitments = Vec::new();
    for &coeff in poly.iter() {
        // Computing g^coeff mod P as the commitment for VSS
        let commitment = mod_exp(G, coeff, P);
        commitments.push(commitment);
    }
    commitments
}

pub fn mod_exp(base: i128, exp: i128, modulus: i128) -> i128 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1; 
        base = (base * base) % modulus; 
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_conversion() {
        let test_value = "aed7015";
        let base = 15;
        let decimal = convert_to_decimal(test_value, base).unwrap();

        let result = convert_from_decimal(decimal, base);

        let normalized_test = test_value.trim_start_matches('0');
        let normalized_result = result.trim_start_matches('0');

        assert_eq!(normalized_test, normalized_result);
    }

    #[test]
    fn test_share_recovery() {
        let json_input = r#"{
            "keys": {"n": 3, "k": 2},
            "1": {"base": "16", "value": "ff"},
            "2": {"base": "16", "value": "1fe"},
            "3": {"base": "16", "value": "2fd"}
        }"#;

        let shares = parse_shares(json_input).unwrap();
        let secret = recover_secret(&shares, 2).unwrap();

        let hex_secret = convert_from_decimal(secret, 16);
        assert_eq!(hex_secret, "0"); 
    }
}
