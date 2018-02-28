extern crate grpc;
extern crate futures;
extern crate RustAndGrpc;

use RustAndGrpc::HelloRust_grpc::*;
use RustAndGrpc::HelloRust::*;

use std::thread;
use std::net::ToSocketAddrs;

struct HelloStruct;


impl HelloRustHandleService for HelloStruct{
    fn hello_rust(&self,  _m : grpc::RequestOptions, req : RequestHelloRust) -> grpc::SingleResponse<ResponseHelloRust>{
        let mut r = ResponseHelloRust::new();
        println!("Client say : {}",req.get_Text());

        r.set_ResponseText("hello Client".to_string());
        grpc::SingleResponse::completed(r)
    }
}


fn main(){
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(9090);
    server.add_service(HelloRustHandleServiceServer::new_service_def(HelloStruct));
    server.http.set_cpu_pool_threads(4);
    let _server = server.build().expect("server");

    loop {
        thread::park();
    }
}