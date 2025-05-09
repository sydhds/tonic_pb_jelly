// std
// third-party
use tonic::Response;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::{HelloReply, HelloRequest, greeter_client::GreeterClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(8);
    let mut client = GreeterClient::connect("http://127.0.0.1:50051").await?;

    let request_0 = HelloRequest {
        name: "Bobby".into(),
        id: (0..n).collect(),
    };
    let request = tonic::Request::new(request_0);
    let response: Response<HelloReply> = client.say_hello(request).await?;
    println!("Response={:?}", response);
    Ok(())
}
