//! Contains the Error and Result type used by the deserializer.
use std::fmt::Display;

/// Various errors that can occur during deserialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error(String);

/// Convenience type for Result.
pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl serde::de::Error for Error {
    #[inline(always)]
    fn custom<T: Display>(msg: T) -> Self {
        Error(msg.to_string())
    }
}

// TODO: Separate error types for ser and de?
impl serde::ser::Error for Error {
    #[inline(always)]
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error(msg.to_string())
    }
}

impl From<std::io::Error> for Error {
    #[inline(always)]
    fn from(e: std::io::Error) -> Self {
        Error(format!("io error: {}", e))
    }
}

impl Error {
    #[inline(always)]
    pub(crate) fn invalid_tag(tag: u8) -> Error {
        Error(format!("invalid nbt tag value: {}", tag))
    }

    #[inline(always)]
    pub(crate) fn no_root_compound() -> Error {
        Error("invalid nbt: no root compound".to_owned())
    }

    #[inline(always)]
    pub(crate) fn nonunicode_string(data: &[u8]) -> Error {
        Error(format!(
            "invalid nbt string: nonunicode: {}",
            String::from_utf8_lossy(data)
        ))
    }

    #[inline(always)]
    pub(crate) fn unexpected_eof() -> Error {
        Error("eof: unexpectedly ran out of input".to_owned())
    }

    #[inline(always)]
    pub(crate) fn array_as_seq() -> Error {
        Error("expected NBT Array, found seq: use ByteArray, IntArray or LongArray types".into())
    }

    #[inline(always)]
    pub(crate) fn array_as_other() -> Error {
        Error("expected NBT Array: use ByteArray, IntArray or LongArray types".into())
    }

    #[inline(always)]
    pub(crate) fn bespoke(msg: String) -> Error {
        Error(msg)
    }
}
