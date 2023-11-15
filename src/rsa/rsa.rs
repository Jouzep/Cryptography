use rand::Rng;
use num_bigint::BigUint;
use num_traits::{One, FromPrimitive};

fn convert_little_endian(value: String) -> String {
    let mut char_vec: Vec<char> = value.chars().collect();
    let mut result: Vec<String> = Vec::new();
    let mut tmp: String = String::new();

    // if (value.len() % 2) == 1 {
        // char_vec = prepend(char_vec, '0');
    // }
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

    random_biguint % &range + lower
}

fn modular_inverse(base: &BigUint, modulus: &BigUint) -> BigUint {
    base.modpow(&(modulus - BigUint::from(2u32)), modulus)
}

fn gen_key(p: &BigUint, q: &BigUint) {
    let mut rng = rand::thread_rng();
    let result = p * q;
    let totient_n = (p - BigUint::one()) * (q - BigUint::one());
    let two = BigUint::from(2u64);
    let mut e = generate_random_biguint_in_range(&mut rng, &two, &totient_n);
    while gcd(&e, &totient_n) != BigUint::from(1u64) {
        e = generate_random_biguint_in_range(&mut rng, &two, &totient_n);
    }

    let d = modular_inverse(&e, &totient_n);
    let negative_one: BigUint = BigUint::one() - BigUint::from_u32(0).unwrap();

    let n_hex = format!("{:x}", result);
    let n = convert_little_endian(n_hex);

    let e_hex = format!("{:x}", e);
    let e = convert_little_endian(e_hex);

    let d_hex = format!("{:x}", d);
    let d = convert_little_endian(d_hex);

    println!("public key: {}-{}", e, n);
    println!("private key: {}-{}", d, n);
}


pub fn run_rsa(args: Vec<String>, message: String) {
    let p = BigUint::parse_bytes(&args[3].as_bytes(), 16).expect("Failed to decode hexadecimal string");
    let q =BigUint::parse_bytes(&args[4].as_bytes(), 16).expect("Failed to decode hexadecimal string");
    let result = match args[2].as_str() {
        "-g" => gen_key(&p, &q),
        _ => println!("Wrong rsa flag"),
    };
}