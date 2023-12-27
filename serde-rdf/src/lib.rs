mod de;
mod error;
mod ser;

#[doc(inline)]
pub use de::{from_str, Deserializer};
#[doc(inline)]
pub use error::{Error, Result};
#[doc(inline)]
pub use ser::{to_string, Serializer};
