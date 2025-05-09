use pb_jelly::Message;
use std::marker::PhantomData;
use crate::codec::JellyCodec;
use tonic::codec::{
    BufferSettings,
    Codec,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct SmallBufferCodec<T, U>(PhantomData<(T, U)>);

impl<T, U> Codec for SmallBufferCodec<T, U>
where
    T: Message + Send + 'static,
    U: Message + Default + Send + 'static,
{
    type Encode = T;
    type Decode = U;

    type Encoder = <JellyCodec<T, U> as Codec>::Encoder; // <ProstCodec<T, U> as Codec>::Encoder;
    type Decoder = <JellyCodec<T, U> as Codec>::Decoder; // <ProstCodec<T, U> as Codec>::Decoder;

    fn encoder(&mut self) -> Self::Encoder {
        // JellyCodec::<T, U>::raw_encoder(BufferSettings::new(512, 4096))
        JellyCodec::<T, U>::raw_encoder(BufferSettings::default())
    }

    fn decoder(&mut self) -> Self::Decoder {
        // JellyCodec::<T, U>::raw_decoder(BufferSettings::new(512, 4096))
        JellyCodec::<T, U>::raw_decoder(BufferSettings::default())
    }
}
