//! Holds literally one utility trait, ToBytes

use crate::interpreter::InterpreterOutput;


/// Trait wrapper to convert `String`'s and `&'static str`'s to byte vectors
pub trait ToBytes {
    fn to_bytes(self) -> Vec<u8>;
}

impl ToBytes for String {
    fn to_bytes(self) -> Vec<u8> {
        self.bytes().collect()
    }
}

impl ToBytes for &'static str {
    fn to_bytes(self) -> Vec<u8> {
        self.to_owned().to_bytes()
    }
}

impl ToBytes for &[u8] {
    fn to_bytes(self) -> Vec<u8> {
        Vec::from(self)
    }
}

impl ToBytes for InterpreterOutput {
    fn to_bytes(self) -> Vec<u8> {
        self.to_vec()
    }
}


// impl<T, J> ToBytes for T where
// T: IntoIterator<Item=J>,
// J: Into<u8> {
    
//     fn to_bytes(self) -> Vec<u8> {
//         self.into_iter().map(|v| v.into()).collect()
//     }
// }