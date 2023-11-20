use rand::Rng;
use num_bigint::{BigUint, ToBigInt, BigInt};
use num_traits::{One, FromPrimitive, ToPrimitive};
use num_traits::Pow;
use num_integer::Integer;

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
    let random_biguint = BigUint::from_bytes_be(&random_bytes);

    random_biguint % &range + lower;
    BigUint::from(65537u64)
}

fn modular_inverse(phi: &BigUint, e: &BigUint) -> Option<BigInt> {
    let e_convert = e.to_bigint().expect("Error converting e to BigInt");
    let phi_convert = phi.to_bigint().expect("Error converting phi to BigInt");
    let mut d = BigInt::from(3);

    while d < phi_convert.clone() {
        println!("{}", d);
        let result = (d.clone() * e_convert.clone()).mod_floor(&phi_convert.clone());
        if result == BigInt::from(1) {
            return Some(d);
        }
        d += BigInt::from(1);
    }
    None
}

fn gen_key(args: Vec<String>) {
    let p = BigUint::parse_bytes(&convert_little_endian(args[3].to_string()).as_bytes(), 16).expect("Failed to decode hexadecimal string");
    let q =BigUint::parse_bytes(&convert_little_endian(args[4].to_string()).as_bytes(), 16).expect("Failed to decode hexadecimal string");

    let mut rng = rand::thread_rng();
    let result = &p * &q;
    let totient_n = (&p - BigUint::one()) * (&q - BigUint::one());
    let two = BigUint::from(3u64);
    let mut e = generate_random_biguint_in_range(&mut rng, &two, &totient_n);
    // while gcd(&e, &totient_n) != BigUint::from(1u64) {
        // e = generate_random_biguint_in_range(&mut rng, &two, &totient_n);
    // }
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

fn crypt_rsa(args: Vec<String>, message: String) {
    let m = BigUint::parse_bytes(&convert_little_endian(message).as_bytes(), 16).expect("Failed to decode hexadecimal string");
    let key_str: Vec<&str> = args[3].split("-").collect();
    let key = BigUint::parse_bytes(&convert_little_endian(key_str[0].to_string()).as_bytes(), 16).expect("Failed to decode hexadecimal string");
    let phi = BigUint::parse_bytes(&convert_little_endian(key_str[1].to_string()).as_bytes(), 16).expect("Failed to decode hexadecimal string");
    println!("m = {:x} key = {:x}", m, key);
    let result_hex = format!("{:x}", ((m.pow(key)).mod_floor(&phi)));
    let result = convert_little_endian(result_hex);
    println!("{}", result);
}

fn decrypt_rsa(args: Vec<String>, message: String) {
    
}

pub fn run_rsa(args: Vec<String>, message: String) {
    match args[2].as_str() {
        "-g" => gen_key(args),
        "-c" => crypt_rsa(args, message),
        "-d" => crypt_rsa(args, message),
        _ => println!("Wrong rsa flag"),
    };
}