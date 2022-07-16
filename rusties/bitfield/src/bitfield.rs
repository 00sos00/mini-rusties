#![allow(dead_code)]
use std::fmt;

#[derive(PartialEq, Eq)]
pub struct BitField {
    pub bytes: Vec<u8>,
}

impl BitField {
    pub fn with_bytes(size: usize) -> Self {
        Self {
            bytes: vec![0; size],
        }
    }

    pub fn with_bits(size: usize) -> Self {
        let size_in_bytes = (size as f32 / u8::BITS as f32).ceil() as usize;

        Self {
            bytes: vec![0; size_in_bytes],
        }
    }

    pub fn set_nth_bit(&mut self, n: usize) {
        let index = (self.bytes.len() - 1) - (n / u8::BITS as usize);
        let bit_to_change = n % u8::BITS as usize;

        if let Some(byte) = self.bytes.get_mut(index) {
            *byte |= 1 << bit_to_change;
        }
    }

    pub fn and(&self, rhs: &Self) -> Self {
        assert_eq!(self.bytes.len(), rhs.bytes.len(), "2 Different lengths");
        
        let bytes = self
            .bytes
            .iter()
            .zip(rhs.bytes.iter())
            .map(|(x1, x2)| x1 & x2)
            .collect();

        Self { bytes }
    }

    pub fn or(&self, rhs: &Self) -> Self {
        assert_eq!(self.bytes.len(), rhs.bytes.len(), "2 Different lengths");

        let bytes = self
            .bytes
            .iter()
            .zip(rhs.bytes.iter())
            .map(|(x1, x2)| x1 | x2)
            .collect();

        Self { bytes }
    }

    pub fn xor(&self, rhs: &Self) -> Self {
        assert_eq!(self.bytes.len(), rhs.bytes.len(), "2 Different lengths");

        let bytes = self
            .bytes
            .iter()
            .zip(rhs.bytes.iter())
            .map(|(x1, x2)| x1 ^ x2)
            .collect();

        Self { bytes }
    }
}

impl fmt::Display for BitField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sep = '_';

        for &byte in self.bytes.iter() {
            if byte == *self.bytes.last().unwrap() {
                write!(f, "{byte:08b}")?;
            } else {
                write!(f, "{byte:08b}{}", sep)?;
            }
        }

        Ok(())
    }
}
