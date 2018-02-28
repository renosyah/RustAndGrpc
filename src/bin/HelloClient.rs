extern crate RustAndGrpc;
extern crate grpc;
extern crate futures;

use RustAndGrpc::HelloRust_grpc::*;
use RustAndGrpc::HelloRust::*;

use std::env;
use std::net::ToSocketAddrs;

fn main() {
    let name = env::args().nth(1).map(|s| s.to_owned()).unwrap_or_else(|| "world".to_owned());

    let client = HelloRustHandleServiceClient::new_plain("localhost", 9090, Default::default()).unwrap();

    let mut req = RequestHelloRust::new();
    req.set_Text(name);

    let resp = client.hello_rust(grpc::RequestOptions::new(), req);

    println!("{:?}", resp.wait());
}