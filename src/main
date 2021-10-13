extern crate rand_core;
extern crate sha2;

use rand_core::OsRng;
use curve25519_dalek::constants;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::montgomery::MontgomeryPoint;
use sha2::Sha512;
use std::time::Instant;
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");

    let n1 = 500;
    let n2 = 5000;

    let intersectionsize = 100;
    assert!(n1 <= n2);
    assert!(intersectionsize <= n1);

    let start = Instant::now();
    // generate input set for P1.
    let mut p1_input = Vec::new();
    for i in 0..n1 {
        let s = Scalar::hash_from_bytes::<Sha512>(i.to_string().as_bytes());
        let mp: MontgomeryPoint = constants::X25519_BASEPOINT * s;
        p1_input.push(mp);
    }

    // generate input set for P2, with intersectionsize amount of duplicate points as P1
    let mut p2_input = Vec::new();
    for i in n1-intersectionsize..n1+n2-intersectionsize+1 {
        let s = Scalar::hash_from_bytes::<Sha512>(i.to_string().as_bytes());
        let mp: MontgomeryPoint = constants::X25519_BASEPOINT * s;
        p2_input.push(mp);
    }

    let duration = start.elapsed();
    println!("Finsihed generating input sets: {:?}", duration);

    let mut csprng = OsRng;

    // P1: generate random scalar a and calculate scalar multiplication for each point in p1_input
    let a: Scalar = Scalar::random(&mut csprng);
    let mut p1_input_a = Vec::new();
    for i in 0..n1 {
        let mp: MontgomeryPoint = p1_input[i] * a;
        p1_input_a.push(mp);
    }

    let duration = start.elapsed() - duration;
    println!("P1 computes p1_input^a: {:?}", duration);

    // P2: generate random scalar b and calculate scalar multiplication for each point in p2_input, p1_input_a

    let b: Scalar = Scalar::random(&mut csprng);
    let mut p2_input_b = Vec::new();
    for i in 0..n2 {
        let mp: MontgomeryPoint = p2_input[i] * b;
        p2_input_b.push(mp);
    }

    let duration = start.elapsed() - duration;
    println!("P2 computes p2_input^b: {:?}", duration);

    // let mut p1_input_a_b = Vec::new();
    // for i in 0..n1 {
    //     let mp: MontgomeryPoint = p1_input_a[i] * b;
    //     p1_input_a_b.push(mp);
    // }

    let mut p1_input_a_b_set = HashSet::new();
    for i in 0..n1 {
        let mp: MontgomeryPoint = p1_input_a[i] * b;
        assert!(p1_input_a_b_set.insert(mp));
    }

    let duration = start.elapsed() - duration;
    println!("P2 computes p1_input^a^b: {:?}", duration);

    // P1: calculate scalar multiplication for each point in p2_input_b

    // let mut p2_input_b_a = Vec::new();
    // for i in 0..n2 {
    //     let mp: MontgomeryPoint = p2_input_b[i] * a;
    //     p2_input_b_a.push(mp);
    // }

    let mut p2_input_b_a_set = HashSet::new();
    for i in 0..n2 {
        let mp: MontgomeryPoint = p2_input_b[i] * a;
        p2_input_b_a_set.insert(mp);
    }


    let duration = start.elapsed() - duration;
    println!("P1 computes p2_input^b^a: {:?}", duration);


    // P1: Compare p1_input_a_b and p2_input_b_a, return intersectionsize(count)
    
    // let mut count = 0;
    // for i in 0..n1 {
    //     if p2_input_b_a.contains(&p1_input_a_b[i]){
    //         count += 1;
    //     }
    // }

    // for i in 0..n2 {
    //     if p1_input_a_b_set.contains(&p2_input_b_a[i]){
    //         count += 1;
    //     }
    // }

    let intersection: HashSet<_> = p1_input_a_b_set.intersection(&p2_input_b_a_set).collect();
    let count = intersection.len();

    let duration = start.elapsed() - duration;
    println!("P1 count intersetion size: {:?}", duration);

    println!("The intersection size is: {}", count)

}
