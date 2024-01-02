extern crate num_cpus;
extern crate num_bigint;
extern crate num_traits;

use std::thread;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::str::FromStr;
use num_integer::Integer;

fn main() {
    let start: BigUint = BigUint::from_str("1300000000000000001").unwrap_or_else(|_| Zero::zero());
    let end: BigUint = BigUint::from_str("1700000000000000001").unwrap_or_else(|_| Zero::zero());
    let start = Arc::new(start);
    let end = Arc::new(end);
    let num_threads = num_cpus::get() as u128;
    println!("Using {num_threads} threads");
    check_range_for_primes(start,end);
}

fn check_range_for_primes(start: Arc<BigUint>, end:Arc<BigUint>){
    let mut it = (*start).clone();
    println!("PRIMES:");
    while it <= *end{
        if prime_check(&it){
            println!("{it}");
        }
        it += 2u8; // Skip even numbers
    }

    println!("Done");
}


fn prime_check(x_arc: &BigUint) -> bool {
    let x = &*x_arc;

    if *x <= BigUint::one() {
        return false;
    }
    if x.is_even() {
        return false;
    }

    let sqrt_x = approximate_sqrt(&x);
    let num_threads = num_cpus::get() as u128;
    let segment_size = (&sqrt_x / num_threads) + 1u8;
    let mut handles = vec![];
    let found_divisor = Arc::new(AtomicBool::new(false));
    //let x = Arc::new(x);

    for i in 0..num_threads {
        let x_clone = x_arc.clone();
        let found_divisor_clone = found_divisor.clone();
        let start = BigUint::from(3u8) + &segment_size * i;
        let end = &start + &segment_size;

        let handle = thread::spawn(move || {
            let mut y = start;
            while y < end {
                if found_divisor_clone.load(Ordering::Relaxed) {
                    return;
                }
                if &x_clone % &y == Zero::zero() {
                    found_divisor_clone.store(true, Ordering::Relaxed);
                    return;
                }
                y += 2u8;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    !found_divisor.load(Ordering::Relaxed)
}

fn approximate_sqrt(x:&BigUint) -> BigUint {
    let two = BigUint::from(2u8);
    let mut s = x.clone();
    let mut next_s = (&s + x / &s) / &two;

    while next_s < s {
        s = next_s;
        next_s = (&s + x / &s) / &two;
    }
    s
}

