extern crate rand_core;
extern crate sha2;

use rand_core::OsRng;
use curve25519_dalek::constants;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::montgomery::MontgomeryPoint;
use sha2::Sha512;

fn main() {
    println!("Hello, world!");

    let n1 = 500;
    let n2 = 5000;

    let intersectionsize = 101;

    assert!(n1 <= n2);
    assert!(intersectionsize <= n1);

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

    // assert!(p1_input[400] == p2_input[0]);

    let mut csprng = OsRng;

    // P1: generate random scalar a and calculate scalar multiplication for each point in p1_input
    let a: Scalar = Scalar::random(&mut csprng);
    let mut p1_input_a = Vec::new();
    for i in 0..n1 {
        let mp: MontgomeryPoint = p1_input[i] * a;
        p1_input_a.push(mp);
    }

    // P2: generate random scalar b and calculate scalar multiplication for each point in p2_input, p1_input_a

    let b: Scalar = Scalar::random(&mut csprng);
    let mut p2_input_b = Vec::new();
    for i in 0..n2 {
        let mp: MontgomeryPoint = p2_input[i] * b;
        p2_input_b.push(mp);
    }

    let mut p1_input_a_b = Vec::new();
    for i in 0..n1 {
        let mp: MontgomeryPoint = p1_input_a[i] * b;
        p1_input_a_b.push(mp);
    }

    // P2: Sort p1_input_a_b, or maybe random shuffle?
    // Need to implement.................



    // P1: calculate scalar multiplication for each point in p2_input_b
    let mut p2_input_b_a = Vec::new();
    for i in 0..n2 {
        let mp: MontgomeryPoint = p2_input_b[i] * a;
        p2_input_b_a.push(mp);
    }

    // P1: Sort p2_input_b_a, or maybe create a hash map?
    // Need to implement.................


    // P1: Compare p1_input_a_b and p2_input_b_a, return intersectionsize(count)
    let mut count = 0;

    for i in 0..n1 {
        if p2_input_b_a.contains(&p1_input_a_b[i]){
            count += 1;
        }
    }

    println!("The intersection size is: {}", count)

}
