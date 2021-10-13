use ddhpsi::util;
use scuttlebutt::{
    AesRng,
    TrackChannel,
    SymChannel,
    channel::AbstractChannel,
};

use std::{
    net::TcpListener,
    time::SystemTime,
};

use curve25519_dalek::{
    scalar::Scalar,
    ristretto::RistrettoPoint,
};

fn server_protocol<C: AbstractChannel>(set_size: usize, channel: &mut C){
    // generate input set for P2, with intersectionsize amount of duplicate points as
    let time = SystemTime::now();
    let p2_input = util::generate_points(set_size);
    println!("server :: generated points in {:?} ms", time.elapsed().unwrap().as_millis());

    let mut rng = AesRng::new();
    let b: Scalar = Scalar::random(&mut rng);

    let time = SystemTime::now();
    let p2_input_b = util::cmult_vec(p2_input, b);
    println!("server :: computed p2^b {:?} ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    util::send_pts(p2_input_b, channel);
    println!("server :: sent p2^b in {:?} ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    let p2_input_ab: Vec<RistrettoPoint> = util::receive_pts(channel);
    println!("server :: received p2^ab in {:?} ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    let p1_input_a = util::receive_pts(channel);
    println!("server :: received p1^a in {:?} ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    let p1_input_ab: Vec<RistrettoPoint>  = util::cmult_vec(p1_input_a, b);
    println!("server :: computed p1^ab in {:?} ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    util::send_pts(p1_input_ab.clone(), channel);
    println!("server :: sent p1^ab in {:?} ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    let intersection_size = util::intersect_size(p1_input_ab, p2_input_ab);
    println!("server :: computed intersection in {:?} ms", time.elapsed().unwrap().as_millis());
    println!("RESULT :: server :: intersection_size: {:?}", intersection_size);



}


pub fn main(){
    let parameters = util::parse_config();
    let (address, _, set_size) = util::get_config_experiments(&parameters);

    let address = format!("{}{}", address,":3000");
    let listener = TcpListener::bind(address).unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3000");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                    let mut channel = TrackChannel::new(SymChannel::new(stream));
                    server_protocol(set_size, &mut channel);
                    return;

            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}
