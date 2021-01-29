extern crate std;

use std::io::{Read,Write,Seek};

pub trait Decode: Sized {
    type Error;
    fn decode<Source: Read + Seek>(source: &mut Source) -> Result<Self, Self::Error>;
}

pub trait Encode {
    type Error;
    fn encode<Target: Write + Seek>(&self, target: &mut Target) -> Result<(), Self::Error>;
}