use rand::prelude::*;
use std::io;

#[derive(Clone)]
struct Keys{
    public: u64,
    private: u64,
    n: u64,
    p: u64,
    q: u64,
    e: u64,
    k: u64,
    d: u64,
    theta_of_n: u64
}

fn random_prime() -> u64{
    let primes: [u64; 10] = [2, 3, 5, 7, 11 ,13 ,17, 19, 23, 29];
    let mut rng = rand::thread_rng();
    let selector = rng.gen_range(0..=9);
    primes[selector]
}

fn mod_inverse(a: u64, m: u64) -> u64{
let mut multiplicative_inverse: u64 = 1;
if num::integer::gcd(a, m) == 1{
    loop{
        if (multiplicative_inverse*a)%m != 1{
            multiplicative_inverse += 1;
        }else{
            break;
        }
    }
}else{
    println!("Multiplicative inverse does not exist for E and theta of N");
}
multiplicative_inverse
}

fn gen_keys() -> Keys{
    let mut rng = rand::thread_rng();
    let mut e_count = 2;
    let primes: [u64; 10] = [2, 3, 5, 7, 11 ,13 ,17, 19, 23, 29];
    let mut keys =  Keys{
        public: 0,
        private: 0,
        n: 0,
        p: 0,
        q: 0,
        e: 0,
        k: 0,
        d: 0,
        theta_of_n: 0
    };

    keys.p = 67;
    keys.q = 71;
    keys.n = keys.p*keys.q;
    keys.theta_of_n = num::integer::lcm(keys.q-1, keys.p-1);
    keys.k = rng.gen_range(0..=100);

    keys.e = 3;
    while num::integer::gcd(keys.theta_of_n, keys.e) != 1{
        keys.e = primes[e_count];
        e_count += 1;
    }
    let _throwaway = keys.q.checked_pow(keys.e.try_into().unwrap());
    match _throwaway{
        Some(_throwaway) => keys.public = keys.p * _throwaway,
        None => println!("Exponent not accetable, triggers overflow on raising q to e. Exponent is {}", keys.e)
    }

    keys.d = mod_inverse(keys.e, keys.theta_of_n);

    keys.private = (keys.k * keys.theta_of_n + 1)/keys.e;
    keys
}

fn get_input() -> String{
    let mut input: String = String::new();
    println!("Please enter your text: ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    print!("Input: {}", input);
    input
}

fn encrypt(input: u64, keys: Keys) -> u64{
    let encrypted_number: u64 = (input.pow(keys.e as u32)) % keys.n;
    encrypted_number
}

fn decrypt(input: u64, keys: Keys) -> u64{
    let decrypted_number: u64 = input.pow(keys.d as u32) % keys.d;
    decrypted_number
}

fn main(){
    let keys: Keys = gen_keys();
    let input: String = get_input();
    let input_number: u64 = 5;
    let encrypted_number: u64 = encrypt(input_number, keys.clone());
    let decrypted_number: u64 = decrypt(encrypted_number, keys.clone());

    println!("\nP: {}\nQ: {}\nE: {}\nTheta of n: {}\nD: {}", keys.p, keys.q, keys.e, keys.theta_of_n, keys.d);
    if num::integer::gcd(keys.theta_of_n, keys.e) == 1{
        println!("E and theta of N are coprime.");
    }

    println!("Input: {}\nEncrypted Number: {}\nDecrypted Number: {}", input_number, encrypted_number, decrypted_number);

    //let ciphertext_bytes: Vec<u64> = encrypt(input_bytes.clone(), keys.clone());
    //let decrypted_bytes: Vec<u64> = decrypt(ciphertext_bytes.clone(), keys.clone());

    //Pass input.as_bytes() and keys to fn encrypt(), should pass back &[u8] of encrypted 'characters'
    
    //Pass encrypted &[u8] and keys to fn decrypt(), should pass back &[u8] identical to input.as_bytes()
}