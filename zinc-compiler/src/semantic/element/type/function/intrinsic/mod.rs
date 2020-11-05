//!
//! The semantic analyzer intrinsic function element.
//!

#[cfg(test)]
mod tests;

pub mod debug;
pub mod error;
pub mod require;
pub mod stdlib;
pub mod zksync;

use std::fmt;

use zinc_build::LibraryFunctionIdentifier;

use zinc_lexical::Location;

use self::debug::Function as DebugFunction;
use self::require::Function as RequireFunction;
use self::stdlib::array_pad::Function as StdArrayPadFunction;
use self::stdlib::array_reverse::Function as StdArrayReverseFunction;
use self::stdlib::array_truncate::Function as StdArrayTruncateFunction;
use self::stdlib::collections_mtreemap_contains::Function as StdCollectionsMTreeMapContainsFunction;
use self::stdlib::collections_mtreemap_get::Function as StdCollectionsMTreeMapGetFunction;
use self::stdlib::collections_mtreemap_insert::Function as StdCollectionsMTreeMapInsertFunction;
use self::stdlib::collections_mtreemap_remove::Function as StdCollectionsMTreeMapRemoveFunction;
use self::stdlib::convert_from_bits_field::Function as StdConvertFromBitsFieldFunction;
use self::stdlib::convert_from_bits_signed::Function as StdConvertFromBitsSignedFunction;
use self::stdlib::convert_from_bits_unsigned::Function as StdConvertFromBitsUnsignedFunction;
use self::stdlib::convert_to_bits::Function as StdConvertToBitsFunction;
use self::stdlib::crypto_blake2s::Function as StdCryptoBlake2sFunction;
use self::stdlib::crypto_pedersen::Function as StdConvertPedersenFunction;
use self::stdlib::crypto_schnorr_signature_verify::Function as StdCryptoSchnorrSignatureVerifyFunction;
use self::stdlib::crypto_sha256::Function as StdCryptoSha256Function;
use self::stdlib::ff_invert::Function as StdFfInvertFunction;
use self::stdlib::Function as StandardLibraryFunction;
use self::zksync::transfer::Function as ZkSyncTransferFunction;
use self::zksync::Function as ZkSyncLibraryFunction;

///
/// The semantic analyzer intrinsic function element.
///
#[derive(Debug, Clone)]
pub enum Function {
    /// The `require(...)` function. See the inner element description.
    Require(RequireFunction),
    /// The `dbg!(...)` function. See the inner element description.
    Debug(DebugFunction),
    /// The standard library function. See the inner element description.
    StandardLibrary(StandardLibraryFunction),
    /// The zkSync library function. See the inner element description.
    ZkSyncLibrary(ZkSyncLibraryFunction),
}

impl Function {
    ///
    /// A shortcut constructor.
    ///
    pub fn new_require() -> Self {
        Self::Require(RequireFunction::default())
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_debug() -> Self {
        Self::Debug(DebugFunction::default())
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_library(identifier: LibraryFunctionIdentifier) -> Self {
        match identifier {
            LibraryFunctionIdentifier::CryptoSha256 => Self::StandardLibrary(
                StandardLibraryFunction::CryptoSha256(StdCryptoSha256Function::default()),
            ),
            LibraryFunctionIdentifier::CryptoPedersen => Self::StandardLibrary(
                StandardLibraryFunction::CryptoPedersen(StdConvertPedersenFunction::default()),
            ),
            LibraryFunctionIdentifier::CryptoBlake2s => Self::StandardLibrary(
                StandardLibraryFunction::CryptoBlake2s(StdCryptoBlake2sFunction::default()),
            ),
            LibraryFunctionIdentifier::CryptoSchnorrSignatureVerify => {
                Self::StandardLibrary(StandardLibraryFunction::CryptoSchnorrSignatureVerify(
                    StdCryptoSchnorrSignatureVerifyFunction::default(),
                ))
            }

            LibraryFunctionIdentifier::ConvertToBits => Self::StandardLibrary(
                StandardLibraryFunction::ConvertToBits(StdConvertToBitsFunction::default()),
            ),
            LibraryFunctionIdentifier::ConvertFromBitsUnsigned => {
                Self::StandardLibrary(StandardLibraryFunction::ConvertFromBitsUnsigned(
                    StdConvertFromBitsUnsignedFunction::default(),
                ))
            }
            LibraryFunctionIdentifier::ConvertFromBitsSigned => {
                Self::StandardLibrary(StandardLibraryFunction::ConvertFromBitsSigned(
                    StdConvertFromBitsSignedFunction::default(),
                ))
            }
            LibraryFunctionIdentifier::ConvertFromBitsField => {
                Self::StandardLibrary(StandardLibraryFunction::ConvertFromBitsField(
                    StdConvertFromBitsFieldFunction::default(),
                ))
            }

            LibraryFunctionIdentifier::ArrayReverse => Self::StandardLibrary(
                StandardLibraryFunction::ArrayReverse(StdArrayReverseFunction::default()),
            ),
            LibraryFunctionIdentifier::ArrayTruncate => Self::StandardLibrary(
                StandardLibraryFunction::ArrayTruncate(StdArrayTruncateFunction::default()),
            ),
            LibraryFunctionIdentifier::ArrayPad => Self::StandardLibrary(
                StandardLibraryFunction::ArrayPad(StdArrayPadFunction::default()),
            ),

            LibraryFunctionIdentifier::FfInvert => Self::StandardLibrary(
                StandardLibraryFunction::FfInvert(StdFfInvertFunction::default()),
            ),

            LibraryFunctionIdentifier::ZksyncTransfer => Self::ZkSyncLibrary(
                ZkSyncLibraryFunction::Transfer(ZkSyncTransferFunction::default()),
            ),

            LibraryFunctionIdentifier::CollectionsMTreeMapGet => {
                Self::StandardLibrary(StandardLibraryFunction::CollectionsMTreeMapGet(
                    StdCollectionsMTreeMapGetFunction::default(),
                ))
            }
            LibraryFunctionIdentifier::CollectionsMTreeMapContains => {
                Self::StandardLibrary(StandardLibraryFunction::CollectionsMTreeMapContains(
                    StdCollectionsMTreeMapContainsFunction::default(),
                ))
            }
            LibraryFunctionIdentifier::CollectionsMTreeMapInsert => {
                Self::StandardLibrary(StandardLibraryFunction::CollectionsMTreeMapInsert(
                    StdCollectionsMTreeMapInsertFunction::default(),
                ))
            }
            LibraryFunctionIdentifier::CollectionsMTreeMapRemove => {
                Self::StandardLibrary(StandardLibraryFunction::CollectionsMTreeMapRemove(
                    StdCollectionsMTreeMapRemoveFunction::default(),
                ))
            }
        }
    }

    ///
    /// Whether the function requires the Rust-macro-like `!` specifier.
    ///
    pub fn requires_exclamation_mark(&self) -> bool {
        matches!(self, Self::Debug(_))
    }

    ///
    /// Whether the function must be called from mutable context.
    ///
    pub fn is_mutable(&self) -> bool {
        match self {
            Self::Require(_) => false,
            Self::Debug(_) => false,
            Self::StandardLibrary(inner) => inner.is_mutable(),
            Self::ZkSyncLibrary(inner) => inner.is_mutable(),
        }
    }

    ///
    /// Returns the function identifier, which is known at compile time.
    ///
    pub fn identifier(&self) -> &'static str {
        match self {
            Self::Require(inner) => inner.identifier,
            Self::Debug(inner) => inner.identifier,
            Self::StandardLibrary(inner) => inner.identifier(),
            Self::ZkSyncLibrary(inner) => inner.identifier(),
        }
    }

    ///
    /// Sets the function call location in the code.
    ///
    pub fn set_location(&mut self, location: Location) {
        match self {
            Self::Require(inner) => inner.location = Some(location),
            Self::Debug(inner) => inner.location = Some(location),
            Self::StandardLibrary(inner) => inner.set_location(location),
            Self::ZkSyncLibrary(inner) => inner.set_location(location),
        }
    }

    ///
    /// Returns the location of the function call.
    ///
    pub fn location(&self) -> Option<Location> {
        match self {
            Self::Require(inner) => inner.location,
            Self::Debug(inner) => inner.location,
            Self::StandardLibrary(inner) => inner.location(),
            Self::ZkSyncLibrary(inner) => inner.location(),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Require(inner) => write!(f, "{}", inner),
            Self::Debug(inner) => write!(f, "{}", inner),
            Self::StandardLibrary(inner) => write!(f, "std::{}", inner),
            Self::ZkSyncLibrary(inner) => write!(f, "zksync::{}", inner),
        }
    }
}
