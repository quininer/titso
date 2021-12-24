use std::fmt;
use std::error::Error as StdError;
use wasm_bindgen::JsValue;


pub struct JsError(JsValue);

pub type JsResult<T> = std::result::Result<T, JsError>;

#[cold]
pub fn cast_failed<E: AsRef<JsValue>>(e: E) -> JsError {
    JsError(JsValue::from(format!("js cast failed: {:?}", e.as_ref())))
}

#[cold]
pub fn cast_debug(err: &dyn fmt::Debug) -> JsError {
    JsError(JsValue::from(format!("{:?}", err)))
}

impl From<indexed_kv::JsError> for JsError {
    #[inline]
    fn from(val: indexed_kv::JsError) -> JsError {
        JsError(val.0)
    }
}

impl From<JsValue> for JsError {
    #[inline]
    fn from(val: JsValue) -> JsError {
        JsError(val)
    }
}

impl From<std::io::Error> for JsError {
    #[inline]
    fn from(err: std::io::Error) -> JsError {
        cast_debug(&err)
    }
}

impl From<getrandom::Error> for JsError {
    #[inline]
    fn from(err: getrandom::Error) -> JsError {
        cast_debug(&err)
    }
}

impl From<titso_core::error::Error> for JsError {
    #[inline]
    fn from(err: titso_core::error::Error) -> JsError {
        cast_debug(&err)
    }
}

impl From<cbor4ii::EncodeError<std::collections::TryReserveError>> for JsError {
    #[inline]
    fn from(val: cbor4ii::EncodeError<std::collections::TryReserveError>) -> JsError {
        cast_debug(&val)
    }
}


impl From<cbor4ii::DecodeError<std::convert::Infallible>> for JsError {
    #[inline]
    fn from(val: cbor4ii::DecodeError<std::convert::Infallible>) -> JsError {
        cast_debug(&val)
    }
}

impl From<String> for JsError {
    #[inline]
    fn from(val: String) -> JsError {
        JsError(JsValue::from(val))
    }
}

impl From<&'static str> for JsError {
    #[inline]
    fn from(val: &'static str) -> JsError {
        JsError(JsValue::from(val))
    }
}

impl fmt::Display for JsError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Debug for JsError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl StdError for JsError {}
