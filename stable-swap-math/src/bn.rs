//! Big number types
#![allow(clippy::assign_op_pattern)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::integer_arithmetic)]

use borsh::{BorshDeserialize, BorshSerialize};
use stable_swap_client::error::SwapError;
use std::borrow::BorrowMut;
use std::io::{Error, ErrorKind, Write};
use std::mem::size_of;
use uint::construct_uint;

macro_rules! impl_borsh_serialize_for_bn {
    ($type: ident) => {
        impl BorshSerialize for $type {
            #[inline]
            fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
                writer.write_all(&self.to_little_endian())
            }
        }
    };
}

macro_rules! impl_borsh_deserialize_for_bn {
    ($type: ident) => {
        impl BorshDeserialize for $type {
            #[inline]
            fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
                if buf.len() < size_of::<$type>() {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Unexpected length of input",
                    ));
                }
                let res = $type::from_le_bytes(buf[..size_of::<$type>()].try_into().unwrap());
                *buf = &buf[size_of::<$type>()..];
                Ok(res)
            }
        }
    };
}

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

impl U256 {
    /// Convert [U256] to u64
    pub fn to_u64(self) -> Option<u64> {
        self.try_to_u64().map_or_else(|_| None, Some)
    }

    /// Convert [U256] to u64
    pub fn try_to_u64(self) -> Result<u64, SwapError> {
        self.try_into().map_err(|_| SwapError::ConversionFailure)
    }

    /// Convert [U256] to u128
    pub fn to_u128(self) -> Option<u128> {
        self.try_to_u128().map_or_else(|_| None, Some)
    }

    /// Convert [U256] to u128
    pub fn try_to_u128(self) -> Result<u128, SwapError> {
        self.try_into().map_err(|_| SwapError::ConversionFailure)
    }

    /// Convert from little endian bytes
    pub fn from_le_bytes(bytes: [u8; 32]) -> Self {
        U256::from_little_endian(&bytes)
    }

}

impl_borsh_deserialize_for_bn!(U256);
impl_borsh_serialize_for_bn!(U256);

construct_uint! {
    /// 192-bit unsigned integer.
    pub struct U192(3);
}

impl U192 {
    /// Convert [U192] to u64
    pub fn to_u64(self) -> Option<u64> {
        self.try_to_u64().map_or_else(|_| None, Some)
    }

    /// Convert [U192] to u64
    pub fn try_to_u64(self) -> Result<u64, SwapError> {
        self.try_into().map_err(|_| SwapError::ConversionFailure)
    }

    /// Convert [U192] to u128
    pub fn to_u128(self) -> Option<u128> {
        self.try_to_u128().map_or_else(|_| None, Some)
    }

    /// Convert [U192] to u128
    pub fn try_to_u128(self) -> Result<u128, SwapError> {
        self.try_into().map_err(|_| SwapError::ConversionFailure)
    }

    /// Convert from little endian bytes
    pub fn from_le_bytes(bytes: [u8; 24]) -> Self {
        U192::from_little_endian(&bytes)
    }
}

impl_borsh_deserialize_for_bn!(U192);
impl_borsh_serialize_for_bn!(U192);
