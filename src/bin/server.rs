use ddhpsi::util;
use scuttlebutt::{
    AesRng,
    TrackChannel,
    SymChannel,
};

use std::{
    net::TcpListener,
    net::TcpStream,
    time::SystemTime,
};

use curve25519_dalek::{
    scalar::Scalar,
    ristretto::RistrettoPoint,
};

use rand::{
    seq::SliceRandom,
    thread_rng,
};

fn server_protocol(set_size: usize, channel: &mut TrackChannel<SymChannel<TcpStream>>){
    // generate input set for P2, with intersectionsize amount of duplicate points as
    let time = SystemTime::now();
    let time_total = SystemTime::now();
    let mut read_total = 0.0;
    let mut write_total = 0.0;

    let mut rng_shuffle = thread_rng();
    let mut p2_input = util::generate_points(set_size);
    p2_input.shuffle(&mut rng_shuffle);

    println!("server :: generated points in {:?} ms", time.elapsed().unwrap().as_millis());

    let mut rng_scalar = AesRng::new();
    let b: Scalar = Scalar::random(&mut rng_scalar);

    let time = SystemTime::now();
    let p2_input_b = util::cmult_vec(p2_input, b);
    println!("server :: computed p2^b {:?} ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    util::send_pts(p2_input_b, channel);
    println!("server :: sent p2^b in {:?} ms", time.elapsed().unwrap().as_millis());
    println!("server :: communication sent {:?} in Mb", channel.kilobits_written() / 1000.0);
    write_total = write_total + channel.kilobits_written() / 1000.0;

    let time = SystemTime::now();
    let p2_input_ab: Vec<RistrettoPoint> = util::receive_pts(channel);
    println!("server :: received p2^ab in {:?} ms", time.elapsed().unwrap().as_millis());
    println!("server :: communication received {:?} in Mb", channel.kilobits_read() / 1000.0);
    read_total = read_total + channel.kilobits_read() / 1000.0;

    let time = SystemTime::now();
    let mut p1_input_a = util::receive_pts(channel);
    p1_input_a.shuffle(&mut rng_shuffle);

    println!("server :: received p1^a in {:?} ms", time.elapsed().unwrap().as_millis());
    println!("server :: communication received {:?} in Mb", channel.kilobits_read() / 1000.0);
    read_total = read_total + channel.kilobits_read() / 1000.0;

    let time = SystemTime::now();
    let p1_input_ab: Vec<RistrettoPoint>  = util::cmult_vec(p1_input_a, b);
    println!("server :: computed p1^ab in {:?} ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    util::send_pts(p1_input_ab.clone(), channel);
    println!("server :: sent p1^ab in {:?} ms", time.elapsed().unwrap().as_millis());
    println!("server :: communication sent {:?} in Mb", channel.kilobits_written() / 1000.0);
    write_total = write_total + channel.kilobits_written() / 1000.0;

    let time = SystemTime::now();
    let intersection_size = util::intersect_size(p1_input_ab, p2_input_ab);
    println!("server :: computed intersection in {:?} ms", time.elapsed().unwrap().as_millis());

    println!("*************************************");
    println!("RESULT :: server :: intersection_size: {:?} items", intersection_size);
    println!("TOTAL COMMUNICATION READ :: server :: intersection_size: {:?} Mb", read_total);
    println!("TOTAL COMMUNICATION WRITE :: server :: {:?} Mb", write_total);
    println!("TOTAL TIME :: server :: {:?} ms", time_total.elapsed().unwrap().as_millis());
    println!("*************************************");

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
