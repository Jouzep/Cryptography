use rand::Rng;
use num_bigint::{BigUint, ToBigInt, BigInt};
use num_traits::{One};

const MINIMUM_EXPONENT: u64 = 3;

fn convert_little_endian(value: String) -> String {
    let mut char_vec: Vec<char> = value.chars().collect();
    let mut result: Vec<String> = Vec::new();
    let mut tmp: String = String::new();

    if (value.len() % 2) == 1 {
        char_vec.insert(0, '0');
    }
    for (i, &c) in char_vec.iter().enumerate() {
        tmp.push(c);
        if i % 2 != 0 {
            result.push(tmp.clone());
            tmp.clear();
        }
    }
    result.reverse();
    result.join("")
}

fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    if b == &BigUint::from(0u64) {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}

fn generate_random_biguint_in_range<R: Rng>(rng: &mut R, lower: &BigUint, upper: &BigUint) -> BigUint {
    let range = upper - lower;
    let random_bytes: Vec<u8> = (0..range.bits() / 8).map(|_| rng.gen()).collect();
    let mut random_biguint = BigUint::from_bytes_be(&random_bytes);

    &random_biguint % &range + lower;
    random_biguint
}

fn modular_inverse(phi: &BigUint, e: &BigUint) -> Option<BigInt> {
    let (gcd, x, _) = extended_gcd(&e.to_bigint().unwrap(), &phi.to_bigint().unwrap());

    if gcd != BigInt::from(1) {
        return None;
    }

    let result = x % &phi.to_bigint().unwrap();
    if result < BigInt::from(0) {
        Some(result + &phi.to_bigint().unwrap())
    } else {
        Some(result)
    }
}

fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if a == &BigInt::from(0) {
        (b.clone(), BigInt::from(0), BigInt::from(1))
    } else {
        let (gcd, x, y) = extended_gcd(&(b % a), a);
        (gcd, y - (b / a) * x.clone(), x)
    }
}

fn gen_key(args: Vec<String>) {
    let p = BigUint::parse_bytes(&convert_little_endian(args[3].to_string()).as_bytes(), 16).expect("Failed to decode hexadecimal string");
    let q = BigUint::parse_bytes(&convert_little_endian(args[4].to_string()).as_bytes(), 16).expect("Failed to decode hexadecimal string");

    let mut rng = rand::thread_rng();
    let result = &p * &q;
    let totient_n = (&p - BigUint::one()) * (&q - BigUint::one());
    let two = BigUint::from(MINIMUM_EXPONENT);
    let mut e = generate_random_biguint_in_range(&mut rng, &two, &totient_n);
    while gcd(&e, &totient_n) != BigUint::from(1u64) {
        e = generate_random_biguint_in_range(&mut rng, &two, &totient_n);
    }

    let d = modular_inverse(&totient_n, &e);

    let n_hex = format!("{:x}", result);
    let n = convert_little_endian(n_hex);

    let e_hex = format!("{:x}", e);
    let e = convert_little_endian(e_hex);

    println!("public key: {}-{}", e, n);
    if let Some(d_value) = d {
        let d_hex = format!("{:x}", d_value);
        let d = convert_little_endian(d_hex);
        println!("private key: {}-{}", d, n);
    } else {
        println!("Failed to generate private key");
    }
}

fn parse_biguint(hex_str: &str) -> BigUint {
    BigUint::parse_bytes(&convert_little_endian(hex_str.to_string()).as_bytes(), 16)
        .expect("Failed to decode hexadecimal string")
}

fn crypt_rsa(args: Vec<String>, message: String) {
    let m = parse_biguint(&message);
    let key_str: Vec<&str> = args[3].split("-").collect();
    let key = parse_biguint(key_str[0]);
    let phi = parse_biguint(key_str[1]);

    let result_hex = format!("{:x}", m.modpow(&key, &phi));
    let result = convert_little_endian(result_hex);
    println!("{}", result);
}

pub fn run_rsa(args: Vec<String>, message: String) {
    match args[2].as_str() {
        "-g" => gen_key(args),
        "-c" => crypt_rsa(args, message),
        "-d" => crypt_rsa(args, message),
        _ => println!("Wrong rsa flag"),
    };
}