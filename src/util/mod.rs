use scuttlebutt::channel::AbstractChannel;

use curve25519_dalek::{
    scalar::Scalar,
    ristretto::{
        CompressedRistretto,
        RistrettoPoint,
    },
};
use sha2::Sha512;
use std::{
    env,
    collections::{
        HashSet,
        HashMap,
    },
    fs::File,
    iter::FromIterator,
    io::{
        BufRead,
        BufReader,
    },
};


pub fn send_pts<C: AbstractChannel>(input: Vec<RistrettoPoint>, channel: &mut C){
    channel.write_usize(input.len()).unwrap();
    channel.flush().unwrap();

    let compressed= input.iter().map(|p| p.compress()).collect::<Vec<_>>();
    for c in compressed{
        let cb = c.as_bytes();
        channel.flush().unwrap();
        channel.write_bytes(cb).unwrap();
        channel.flush().unwrap();
    }
}

pub fn receive_pts<C: AbstractChannel>(channel: &mut C) -> Vec<RistrettoPoint>{
    let mut received: Vec<RistrettoPoint> = Vec::new();

    let len = channel.read_usize().unwrap();
    for _i in 0..len{
        let mut p = vec![0u8; 32];
        channel.read_bytes(&mut p).unwrap();

        let cr = CompressedRistretto::from_slice(&p);
        let r = cr.decompress().unwrap();
        received.push(r);
    }
    received
}

pub fn cmult_vec(points: Vec<RistrettoPoint>, c: Scalar)->Vec<RistrettoPoint>{
    let mut result:Vec<RistrettoPoint> = Vec::new();
    for p in points {
        let rp: RistrettoPoint = p * c;
        result.push(rp);
    }
    result
}

pub fn generate_points(n: usize)->Vec<RistrettoPoint>{
    let mut points = Vec::new();
    for i in 0..n {
        let p = (i as u32).to_le_bytes();
        let rp: RistrettoPoint = RistrettoPoint::hash_from_bytes::<Sha512>(&p);
        points.push(rp);
    }
    points
}

pub fn intersect_size(set1: Vec<RistrettoPoint>, set2: Vec<RistrettoPoint>)-> usize {
    let set1: HashSet<[u8; 32]> = HashSet::from_iter(set1.iter().map(|rp| rp.compress().to_bytes()).collect::<Vec<[u8; 32]>>());
    let set2: HashSet<[u8; 32]> = HashSet::from_iter(set2.iter().map(|rp| rp.compress().to_bytes()).collect::<Vec<[u8; 32]>>());
    set1.intersection(&set2).count()
}


pub fn parse_config() -> HashMap<String, String>{

    let mut path = env::current_exe().unwrap();
    println!("path {:?}", path);
    path.pop();
    path.pop();
    path.pop();
    path.push("config/configuration.txt");

    let absolute_path = path.clone().into_os_string().into_string().unwrap();
    let configuration = File::open(absolute_path).unwrap();
    let buffer = BufReader::new(configuration).lines();
    let mut parameters = HashMap::new();
    for line in buffer.enumerate(){
        let read_line =  line.1.unwrap();
        if !read_line.is_empty(){
            let line_split = read_line.split(": ").map(|item| item.to_string()).collect::<Vec<String>>();
            parameters.insert(line_split[0].clone(), line_split[1].clone());
        }
    }
    parameters
}


pub fn get_config_experiments(parameters: &HashMap<String, String>)->
                                    (String, usize, usize){
    let address = parameters.get("address").unwrap().to_owned();
    let client_set_size = parameters.get("client_set_size").unwrap().parse::<usize>().unwrap();
    let server_set_size = parameters.get("server_set_size").unwrap().parse::<usize>().unwrap();

    (address, client_set_size, server_set_size)
}
