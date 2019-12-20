use bech32::ToBase32;
use blake2b_simd::Params;
use secp256k1::rand::thread_rng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use std::env;
use std::sync::{atomic::AtomicBool, atomic::AtomicU64, atomic::Ordering, Arc};
use std::time::SystemTime;

#[macro_use]
extern crate hex_literal;

const CHARSET: [char; 32] = [
    'q', 'p', 'z', 'r', 'y', '9', 'x', '8', 'g', 'f', '2', 't', 'v', 'd', 'w', '0', 's', '3', 'j',
    'n', '5', '4', 'k', 'h', 'c', 'e', '6', 'm', 'u', 'a', '7', 'l',
];
const FORMAT_TYPE_SHORT: u8 = 1;
const CODE_INDEX_SECP256K1_SINGLE: u8 = 0;

fn seckey_to_address(secret_key: &SecretKey, secp: &Secp256k1<secp256k1::All>) -> String {
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let hash_result = Params::new()
        .hash_length(32)
        .personal(b"ckb-default-hash")
        .to_state()
        .update(&public_key.serialize()[..])
        .finalize()
        .as_bytes()[0..20]
        .to_vec();
    let address = bech32::encode(
        "ckb",
        [
            vec![FORMAT_TYPE_SHORT, CODE_INDEX_SECP256K1_SINGLE],
            hash_result,
        ]
        .concat()
        .to_base32(),
    )
    .unwrap();
    address
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_seckey_to_address() {
        let secp = Secp256k1::new();
        let raw_seckey = hex!("0a1fbfe9855a03e283ef1bf5a2e6d9ff1c279a32f683e40d1a29f64f9762ddd6");
        let seckey = SecretKey::from_slice(&raw_seckey).unwrap();
        assert_eq!(
            seckey_to_address(&seckey, &secp),
            "ckb1qyqfle5hj4lrrwl8a5l6600td6tt67c8ch8qehxkt2".to_string()
        );
    }
}

fn run(id: i32, prefix: String, counter: Arc<AtomicU64>, flag: Arc<AtomicBool>) {
    let mut local_count = 0;
    let start_time = SystemTime::now();
    let sync_num = 10000;
    let log_num = 100000;
    // payload is [FORMAT_TYPE_SHORT, CODE_INDEX_SECP256K1_SINGLE, ...], which is 00000 00100 00000 0xxx...
    let estimated_hash_num = 32.0_f64.powi((prefix.len() - "ckb1qyq".len()) as i32) / 2.0;
    let secp = Secp256k1::new();
    loop {
        let secret_key = SecretKey::new(&mut thread_rng());
        let address = seckey_to_address(&secret_key, &secp);

        if address.starts_with(&prefix) {
            println!("result:");
            println!("privkey:\t0x{}", secret_key);
            println!("address:\t{}", address);
            flag.store(true, Ordering::SeqCst);
            break;
        }

        local_count += 1;
        if local_count % sync_num == 0 {
            if flag.load(Ordering::SeqCst) {
                break;
            }
            counter.fetch_add(sync_num, Ordering::SeqCst);
        }
        if id == 0 && local_count % log_num == 0 {
            let elapsed_secs = start_time.elapsed().unwrap().as_millis() as f64 / 1000.0;
            let total_count = counter.load(Ordering::SeqCst);
            let speed = (total_count as f64) / elapsed_secs;
            let time_left = (estimated_hash_num - total_count as f64) / speed;
            println!(
                "count: {}\telapsed: {:.2}min\tspeed: {:.2}/s\tprogress(est): {:.2}%\tleft(est): {:.2}min",
                total_count,
                elapsed_secs / 60.0,
                speed,
                ((total_count as f64) / estimated_hash_num * 100.0),
                time_left / 60.0
            );
        }
    }
}

fn main() {
    if env::args().len() < 2 {
        eprintln!(
            "usage: {} <the address prefix to match>",
            env::args().nth(0).unwrap()
        );
        return;
    }
    let prefix = env::args().nth(1).unwrap();
    if !prefix.starts_with("ckb1qyq") {
        eprintln!("address must have prefix ckb1qyq");
        return;
    }
    let real_start_position = "ckb1qyq".len();
    for c in prefix[real_start_position..].chars() {
        if !CHARSET.contains(&c) {
            eprintln!("invalid char: {}", c);
            return;
        }
    }
    let first_candidate_char = prefix.chars().nth(real_start_position).unwrap();
    if CHARSET
        .iter()
        .position(|&c| c == first_candidate_char)
        .unwrap()
        >= 16
    {
        eprintln!(
            "the {}th char('{}') is impossible, change your prefix",
            real_start_position, first_candidate_char
        );
        return;
    }

    println!("checking prefix {}", prefix);

    let counter = Arc::new(AtomicU64::new(0));
    let flag = Arc::new(AtomicBool::new(false));
    let thread_num = num_cpus::get() / 2;
    let mut threads = Vec::new();
    for idx in 0..thread_num {
        let local_counter = counter.clone();
        let local_flag = flag.clone();
        let prefix = prefix.clone();
        let thread = std::thread::spawn(move || run(idx as i32, prefix, local_counter, local_flag));
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
}
