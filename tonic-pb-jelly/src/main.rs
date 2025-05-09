
pub mod codec;
mod common;

// std
// third party
use tonic::{Request, Response, Status, transport::Server};
// local
use proto_helloworld::proto_helloworld::{
    HelloReply,
    HelloRequest,
};

pub mod hello_world {
    // Include generated code (see build.rs)
    // ./target/debug/build/helloworld-tonic-07b533b6938c251d/out/helloworld.rs
    tonic::include_proto!("helloworld.Greeter");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("helloworld_descriptor");
}

use crate::hello_world::greeter_server::{Greeter, GreeterServer};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    // trait Greeter is defined in included generated code from proto
    // see cat ./target/debug/build/helloworld-tonic-07b533b6938c251d/out/helloworld.rs

    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);
        // Get a HelloRequest
        let inner = request.into_inner();
        let reply = HelloReply {
            message: format!(
                "Hello {} {} !",
                inner.name,
                String::from_utf8_lossy(inner.id.as_slice())
            ),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    println!("[tonic + pb-jelly server] Listening on {}", addr);
    // init the service
    let greeter = MyGreeter::default();

    let reflexion_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(hello_world::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    // Start the server with the service
    Server::builder()
        .add_service(reflexion_service)
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
