# tonic_pb_jelly

[Tonic](https://github.com/hyperium/tonic) + custom codec + [pb_jelly](https://github.com/dropbox/pb-jelly)

How to use a custom codec for Tonic: encoding && decoding using crate pb-jelly (instead of prost)

## Architecture

* pb-jelly-gen
  * project to generate rust code from proto file using pb-jelly
  * code is generated in folder: "gen"
* gen: generated file (see pb-jelly-gen)
* proto: protobuf file
* proto_include: include file (in order to process proto file for tonic)
  * [extensions.proto](https://github.com/dropbox/pb-jelly/blob/main/pb-jelly-gen/proto/rust/extensions.proto) copied from pb-jelly git repo
* tonic-helloworld
  * basic grpc example from [tonic examples](https://github.com/hyperium/tonic/tree/master/examples) 
* tonic-pb-jelly
  * tonic + custom codec using pb-jelly crate 

## Running tonic + pb-jelly

## Server

* cargo run --bin tonic-pb-jelly-server

Note:
* Will reject any Message with field id with length != 8 (see proto file)

## Client

* cargo run --bin tonic-pb-jelly-client 8: Ok
* cargo run --bin tonic-pb-jelly-client 7: Will not compile (expect [u8; 8])

## grpcurl

* grpcurl -plaintext -d '{"name": "Tonicooo", "id": "AAECAwQFBgc="}' '127.0.0.1:50051' helloworld.Greeter/SayHello
* grpcurl -plaintext -d '{"name": "Tonicooo", "id": "AAECAwQFBgcI"}' '127.0.0.1:50051' helloworld.Greeter/SayHello

Note: id is base64 encoded data
Generate:

```python3
import base64
base64.b64encode(bytes(bytearray([0, 1, 2, 3, 4, 5, 6, 7])))
```

## Running tonic (prost) 

## Server

* cargo run --bin tonic-server-helloworld

## Client

Note: the first argument is the number of bytes for field "id"

* cargo run --bin tonic-client-helloworld 8: Ok
* cargo run --bin tonic-client-helloworld 9: Expect an error message "Not of lenght 8"
