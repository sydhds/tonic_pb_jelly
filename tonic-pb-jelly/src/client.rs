pub mod hello_world {
    tonic::include_proto!("helloworld.Greeter");
}

// Note: required for tonic auto-generated code (helloworld.Greeter.rs) to compile
mod codec;

use proto_helloworld::proto_helloworld::{HelloReply, HelloRequest};

use hello_world::{
    greeter_client::GreeterClient,
};
use tonic::Response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(8);

    let mut client = GreeterClient::connect("http://127.0.0.1:50051").await?;

    let request_0 = HelloRequest {
        name: "Bobby".into(),
        id: (0..n).collect::<Vec<u8>>().as_slice().try_into().unwrap(),
    };
    let request = tonic::Request::new(request_0);

    let response: Response<HelloReply> = client.say_hello(request).await?;

    println!("Response={:?}", response);

    Ok(())
}
