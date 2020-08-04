use std::fmt;
use std::error::Error as StdError;
use wasm_bindgen::JsValue;
use web_sys::Element;


pub struct JsError(JsValue);

pub type JsResult<T> = std::result::Result<T, JsError>;

#[cold]
pub fn cast_failed<E: AsRef<JsValue>>(e: E) -> JsError {
    JsError(JsValue::from(format!("js cast failed: {:?}", e.as_ref())))
}

impl From<JsValue> for JsError {
    fn from(val: JsValue) -> JsError {
        JsError(val)
    }
}

impl From<String> for JsError {
    fn from(val: String) -> JsError {
        JsError(JsValue::from(val))
    }
}

impl From<&'static str> for JsError {
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
