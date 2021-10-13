use ddhpsi::util;
use scuttlebutt::{
    AesRng,
    TrackChannel,
    SymChannel,
    channel::AbstractChannel,
};

use std::{
    net::TcpStream,
    time::SystemTime,
};

use curve25519_dalek::{
    scalar::Scalar,
    ristretto::RistrettoPoint,
};

fn client_protocol<C: AbstractChannel>(set_size: usize, channel: &mut C){
    let time = SystemTime::now();
    // generate input set for P1.
    let p1_input = util::generate_points(set_size);
    println!("client :: generated points in {:?} ms", time.elapsed().unwrap().as_millis());

    // P1: generate random scalar a and calculate scalar multiplication for each point in p1_input
    let mut rng = AesRng::new();
    let a: Scalar = Scalar::random(&mut rng);

    let time = SystemTime::now();
    let p1_input_a = util::cmult_vec(p1_input, a);
    println!("client :: computed p1^a {:?} ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    // P1 reads points from P2
    let p2_input_b = util::receive_pts(channel);
    println!("client :: received p2^b in {:?} ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    let p2_input_ab = util::cmult_vec(p2_input_b, a);
    println!("client :: computed p2^ab in {:?} ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    util::send_pts(p2_input_ab.clone(), channel);
    println!("client :: sent p2^ab in {:?} ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    util::send_pts(p1_input_a, channel);
    println!("client :: sent p1^a in {:?} ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    let p1_input_ab: Vec<RistrettoPoint> = util::receive_pts(channel).into_iter().collect();
    println!("client :: received p1^ab in {:?} ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    let intersection_size = util::intersect_size(p1_input_ab, p2_input_ab);
    println!("client :: computed intersection in {:?} ms", time.elapsed().unwrap().as_millis());
    println!("RESULT :: client :: intersection_size: {:?}", intersection_size);
}

pub fn main(){
    let parameters = util::parse_config();
    let (address, set_size, _) = util::get_config_experiments(&parameters);

    let address = format!("{}{}", address,":3000");
    match TcpStream::connect(address) {
        Ok(stream) => {
            let mut channel = TrackChannel::new(SymChannel::new(stream));
            client_protocol(set_size, &mut channel);
            return;
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
