use ddhpsi::util;
use scuttlebutt::{
    AesRng,
    TrackChannel,
    SymChannel,
};

use std::{
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


fn client_protocol(set_size: usize, channel: &mut TrackChannel<SymChannel<TcpStream>>){
    let time = SystemTime::now();
    let time_total = SystemTime::now();
    let mut read_total = 0.0;
    let mut write_total = 0.0;

    // generate input set for P1 and shuffle.
    let mut rng_shuffle = thread_rng();
    let mut p1_input = util::generate_points(set_size);
    p1_input.shuffle(&mut rng_shuffle);

    println!("client :: generated points in {:?} ms", time.elapsed().unwrap().as_millis());

    // P1: generate random scalar a and calculate scalar multiplication for each point in p1_input
    let mut rng = AesRng::new();
    let a: Scalar = Scalar::random(&mut rng);

    let time = SystemTime::now();
    let p1_input_a = util::cmult_vec(p1_input, a);
    println!("client :: computed p1^a {:?} ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    // P1 reads points from P2 and shuffles
    let mut p2_input_b = util::receive_pts(channel);
    p2_input_b.shuffle(&mut rng_shuffle);
    println!("client :: received p2^b in {:?} ms", time.elapsed().unwrap().as_millis());
    println!("client :: communication received {:?} in Mb", channel.kilobits_read() / 1000.0);
    read_total = read_total + channel.kilobits_read() / 1000.0;

    let time = SystemTime::now();
    let p2_input_ab = util::cmult_vec(p2_input_b, a);
    println!("client :: computed p2^ab in {:?} ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    util::send_pts(p2_input_ab.clone(), channel);
    println!("client :: sent p2^ab in {:?} ms", time.elapsed().unwrap().as_millis());
    println!("client :: communication sent {:?} in Mb", channel.kilobits_written() / 1000.0);
    write_total = write_total + channel.kilobits_written() / 1000.0;

    let time = SystemTime::now();
    util::send_pts(p1_input_a, channel);
    println!("client :: sent p1^a in {:?} ms", time.elapsed().unwrap().as_millis());
    println!("client :: communication sent {:?} in Mb", channel.kilobits_written() / 1000.0);
    write_total = write_total + channel.kilobits_written() / 1000.0;

    let time = SystemTime::now();
    let p1_input_ab: Vec<RistrettoPoint> = util::receive_pts(channel).into_iter().collect();
    println!("client :: received p1^ab in {:?} ms", time.elapsed().unwrap().as_millis());
    println!("client :: communication received {:?} in Mb", channel.kilobits_read() / 1000.0);
    read_total = read_total + channel.kilobits_read() / 1000.0;

    let time = SystemTime::now();
    let intersection_size = util::intersect_size(p1_input_ab, p2_input_ab);
    println!("client :: computed intersection in {:?} ms", time.elapsed().unwrap().as_millis());

    println!("*************************************");
    println!("RESULT :: client :: intersection_size: {:?} items", intersection_size);
    println!("TOTAL COMMUNICATION READ :: client :: intersection_size: {:?} Mb", read_total);
    println!("TOTAL COMMUNICATION WRITE :: client :: {:?} Mb", write_total);
    println!("TOTAL TIME :: client :: {:?} ms", time_total.elapsed().unwrap().as_millis());
    println!("*************************************");
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
