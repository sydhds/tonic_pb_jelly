// @generated, do not edit
/// Request
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HelloRequest {
    pub name: ::std::string::String,
    pub id: [u8; 8],
}
impl ::std::default::Default for HelloRequest {
    fn default() -> Self {
        HelloRequest {
            name: ::std::default::Default::default(),
            id: ::std::default::Default::default(),
        }
    }
}
::lazy_static::lazy_static! {
  pub static ref HelloRequest_default: HelloRequest = HelloRequest::default();
}
impl ::pb_jelly::Message for HelloRequest {
    fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
        Some(::pb_jelly::MessageDescriptor {
            name: "HelloRequest",
            full_name: "helloworld.HelloRequest",
            fields: &[
                ::pb_jelly::FieldDescriptor {
                    name: "name",
                    full_name: "helloworld.HelloRequest.name",
                    index: 0,
                    number: 1,
                    typ: ::pb_jelly::wire_format::Type::LengthDelimited,
                    label: ::pb_jelly::Label::Optional,
                    oneof_index: None,
                },
                ::pb_jelly::FieldDescriptor {
                    name: "id",
                    full_name: "helloworld.HelloRequest.id",
                    index: 1,
                    number: 2,
                    typ: ::pb_jelly::wire_format::Type::LengthDelimited,
                    label: ::pb_jelly::Label::Optional,
                    oneof_index: None,
                },
            ],
            oneofs: &[],
        })
    }
    fn compute_size(&self) -> usize {
        let mut size = 0usize;
        size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(
            &self.name,
            1,
            ::pb_jelly::wire_format::Type::LengthDelimited,
        );
        size += ::pb_jelly::helpers::compute_size_scalar::<[u8; 8]>(
            &self.id,
            2,
            ::pb_jelly::wire_format::Type::LengthDelimited,
        );
        size
    }
    fn compute_grpc_slices_size(&self) -> usize {
        let mut size = 0usize;
        if self.id != <[u8; 8] as ::std::default::Default>::default() {
            let val = &self.id;
            size += ::pb_jelly::Message::compute_grpc_slices_size(val);
        }
        size
    }
    fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
        ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(
            w,
            &self.name,
            1,
            ::pb_jelly::wire_format::Type::LengthDelimited,
        )?;
        ::pb_jelly::helpers::serialize_scalar::<W, [u8; 8]>(
            w,
            &self.id,
            2,
            ::pb_jelly::wire_format::Type::LengthDelimited,
        )?;
        Ok(())
    }
    fn deserialize<B: ::pb_jelly::PbBufferReader>(
        &mut self,
        mut buf: &mut B,
    ) -> ::std::io::Result<()> {
        while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
            match field_number {
                1 => {
                    let val = ::pb_jelly::helpers::deserialize_length_delimited::<
                        B,
                        ::std::string::String,
                    >(buf, typ, "HelloRequest", 1)?;
                    self.name = val;
                }
                2 => {
                    let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, [u8; 8]>(
                        buf,
                        typ,
                        "HelloRequest",
                        2,
                    )?;
                    self.id = val;
                }
                _ => {
                    ::pb_jelly::skip(typ, &mut buf)?;
                }
            }
        }
        Ok(())
    }
}
impl ::pb_jelly::Reflection for HelloRequest {
    fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
        match oneof_name {
            _ => {
                panic!("unknown oneof name given");
            }
        }
    }
    fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
        match field_name {
            "name" => ::pb_jelly::reflection::FieldMut::Value(&mut self.name),
            "id" => ::pb_jelly::reflection::FieldMut::Value(&mut self.id),
            _ => {
                panic!("unknown field name given")
            }
        }
    }
}

/// Reply
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HelloReply {
    pub message: ::std::string::String,
}
impl ::std::default::Default for HelloReply {
    fn default() -> Self {
        HelloReply {
            message: ::std::default::Default::default(),
        }
    }
}
::lazy_static::lazy_static! {
  pub static ref HelloReply_default: HelloReply = HelloReply::default();
}
impl ::pb_jelly::Message for HelloReply {
    fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
        Some(::pb_jelly::MessageDescriptor {
            name: "HelloReply",
            full_name: "helloworld.HelloReply",
            fields: &[::pb_jelly::FieldDescriptor {
                name: "message",
                full_name: "helloworld.HelloReply.message",
                index: 0,
                number: 1,
                typ: ::pb_jelly::wire_format::Type::LengthDelimited,
                label: ::pb_jelly::Label::Optional,
                oneof_index: None,
            }],
            oneofs: &[],
        })
    }
    fn compute_size(&self) -> usize {
        let mut size = 0usize;
        size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(
            &self.message,
            1,
            ::pb_jelly::wire_format::Type::LengthDelimited,
        );
        size
    }
    fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
        ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(
            w,
            &self.message,
            1,
            ::pb_jelly::wire_format::Type::LengthDelimited,
        )?;
        Ok(())
    }
    fn deserialize<B: ::pb_jelly::PbBufferReader>(
        &mut self,
        mut buf: &mut B,
    ) -> ::std::io::Result<()> {
        while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
            match field_number {
                1 => {
                    let val = ::pb_jelly::helpers::deserialize_length_delimited::<
                        B,
                        ::std::string::String,
                    >(buf, typ, "HelloReply", 1)?;
                    self.message = val;
                }
                _ => {
                    ::pb_jelly::skip(typ, &mut buf)?;
                }
            }
        }
        Ok(())
    }
}
impl ::pb_jelly::Reflection for HelloReply {
    fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
        match oneof_name {
            _ => {
                panic!("unknown oneof name given");
            }
        }
    }
    fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
        match field_name {
            "message" => ::pb_jelly::reflection::FieldMut::Value(&mut self.message),
            _ => {
                panic!("unknown field name given")
            }
        }
    }
}
