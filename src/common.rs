use core::u16;
use serialize::{json, Encodable, Encoder};
use serialize::json::{ToJson, ParserError};
use std::collections::TreeMap;
use std::error::{Error, FromError};

#[deriving(PartialEq, Show)]
pub enum ErrorStatus {
    ElementNotSelectable,
    ElementNotVisible,
    InvalidArgument,
    InvalidCookieDomain,
    InvalidElementCoordinates,
    InvalidElementState,
    InvalidSelector,
    InvalidSessionId,
    JavascriptError,
    MoveTargetOutOfBounds,
    NoSuchAlert,
    NoSuchElement,
    NoSuchFrame,
    NoSuchWindow,
    ScriptTimeout,
    SessionNotCreated,
    StaleElementReference,
    Timeout,
    UnableToSetCookie,
    UnexpectedAlertOpen,
    UnknownError,
    UnknownPath,
    UnknownMethod,
    UnsupportedOperation,
}

pub type WebDriverResult<T> = Result<T, WebDriverError>;

#[deriving(Show)]
pub struct WebDriverError {
    pub status: ErrorStatus,
    pub message: String
}

impl WebDriverError {
    pub fn new(status: ErrorStatus, message: &str) -> WebDriverError {
        WebDriverError {
            status: status,
            message: message.to_string().clone()
        }
    }

    pub fn status_code(&self) -> &str {
    // This expands to status_code<'a>(&'a self) -> &'a str; consider
    // status_code(&self) -> &'static str.
        match self.status {
            ErrorStatus::ElementNotSelectable => "element not selectable",
            ErrorStatus::ElementNotVisible => "element not visible",
            ErrorStatus::InvalidArgument => "invalid argument",
            ErrorStatus::InvalidCookieDomain => "invalid cookie domain",
            ErrorStatus::InvalidElementCoordinates => "invalid element coordinates",
            ErrorStatus::InvalidElementState => "invalid element state",
            ErrorStatus::InvalidSelector => "invalid selector",
            ErrorStatus::InvalidSessionId => "invalid session id",
            ErrorStatus::JavascriptError => "javascript error",
            ErrorStatus::MoveTargetOutOfBounds => "move target out of bounds",
            ErrorStatus::NoSuchAlert => "no such alert",
            ErrorStatus::NoSuchElement => "no such element",
            ErrorStatus::NoSuchFrame => "no such frame",
            ErrorStatus::NoSuchWindow => "no such window",
            ErrorStatus::ScriptTimeout => "script timeout",
            ErrorStatus::SessionNotCreated => "session not created",
            ErrorStatus::StaleElementReference => "stale element reference",
            ErrorStatus::Timeout => "timeout",
            ErrorStatus::UnableToSetCookie => "unable to set cookie",
            ErrorStatus::UnexpectedAlertOpen => "unexpected alert open",
            ErrorStatus::UnknownError => "unknown error",
            ErrorStatus::UnknownPath => "unknown command",
            ErrorStatus::UnknownMethod => "unknown command",
            ErrorStatus::UnsupportedOperation => "unsupported operation",
        }
    }

    pub fn http_status(&self) -> int {
        match self.status {
            ErrorStatus::UnknownPath => 404,
            ErrorStatus::UnknownMethod => 405,
            _ => 500
        }
    }

    pub fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }
}

impl ToJson for WebDriverError {
    fn to_json(&self) -> json::Json {
        let mut data = TreeMap::new();
        data.insert("status".to_string(), self.status_code().to_json());
        data.insert("error".to_string(), self.message.to_json());
        json::Object(data)
    }
}

impl Error for WebDriverError {
    fn description(&self) -> &str {
        self.status_code()
    }

    fn detail(&self) -> Option<String> {
        Some(self.message.clone())
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl FromError<ParserError> for WebDriverError {
    fn from_error(err: ParserError) -> WebDriverError {
        let msg = format!("{}", err);
        WebDriverError::new(ErrorStatus::UnknownError, msg.as_slice())
    }
}

#[deriving(PartialEq, Clone, Show)]
pub enum Nullable<T: ToJson> { // Curious.
    Value(T),
    Null
}

impl<T: ToJson> Nullable<T> {
     pub fn is_null(&self) -> bool {
        match *self {
            Nullable::Value(_) => false,
            Nullable::Null => true
        }
    }

     pub fn is_value(&self) -> bool {
        match *self {
            Nullable::Value(_) => true,
            Nullable::Null => false
        }
    }
}

impl<T: ToJson> Nullable<T> {
    //This is not very pretty
    pub fn from_json<F: FnOnce(&json::Json) -> WebDriverResult<T>>(value: &json::Json, f: F) -> WebDriverResult<Nullable<T>> {
        if value.is_null() {
            Ok(Nullable::Null)
        } else {
            Ok(Nullable::Value(try!(f(value))))
        }
    }
}

impl<T: ToJson> ToJson for Nullable<T> {
    fn to_json(&self) -> json::Json {
        match *self {
            Nullable::Value(ref x) => x.to_json(),
            Nullable::Null => json::Json::Null
        }
    }
}

impl<S: Encoder<E>, E, T: ToJson> Encodable<S, E> for Nullable<T> {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        match *self {
            Nullable::Value(ref x) => x.to_json().encode(s),
            Nullable::Null => s.emit_nil()
        }
    }
}

#[deriving(PartialEq)]
pub struct WebElement {
    pub id: String
}

impl WebElement {
    pub fn new(id: String) -> WebElement {
        WebElement {
            id: id
        }
    }

    pub fn from_json(data: &json::Json) -> WebDriverResult<WebElement> {
        Ok(WebElement::new(
            try_opt!(
                try_opt!(
                    try_opt!(data.as_object(),
                             ErrorStatus::InvalidArgument,
                             "Could not convert webelement to object").get(
                        "element-6066-11e4-a52e-4f735466cecf"),
                    ErrorStatus::InvalidArgument,
                    "Could not find webelement key").as_string(),
                ErrorStatus::InvalidArgument,
                "Could not convert web element to string").into_string()))
        // Not very readable...
        let object = try_opt!(data.as_object(),
                              ErrorStatus::InvalidArgument,
                              "Could not convert webelement to object");
        let key_value = try_opt!(object.get("element-6066-11e4-a52e-4f735466cecf"),
                                 ErrorStatus::InvalidArgument,
                                 "Could not find webelement key");
        let key = try_opt!(key_value.as_string(),
                           ErrorStatus::InvalidArgument,
                           "Could not convert web element to string").into_string();
        Ok(WebElement::new(key))
    }
}

impl ToJson for WebElement {
    fn to_json(&self) -> json::Json {
        let mut data = TreeMap::new();
        data.insert("element-6066-11e4-a52e-4f735466cecf".to_string(), self.id.to_json());
                    // ^ constant!
        json::Object(data)
    }
}

#[deriving(PartialEq)]
pub enum FrameId {
    Short(u16),
    Element(WebElement),
    Null
}

impl FrameId {
    pub fn from_json(data: &json::Json) -> WebDriverResult<FrameId> {
      match data {
          // indentation
            &json::Json::U64(x) => {
                if x <= u16::MAX as u64 {
                    Ok(FrameId::Short(x as u16))
                } else {
                    Err(WebDriverError::new(ErrorStatus::NoSuchFrame,
                                            "frame id out of range"))
                }
                // Or... use std::num::ToPrimitive;
                match x.to_u16() {
                    Some(x) => Ok(FrameId::Short(x)),
                    None => Err(WebDriverError::new(ErrorStatus::NoSuchFrame,
                                                    "frame id out of range")),
                }
            },
          &json::Json::Null => Ok(FrameId::Null),
          &json::Json::String(ref x) => Ok(FrameId::Element(WebElement::new(x.clone()))),
          _ => Err(WebDriverError::new(ErrorStatus::NoSuchFrame,
                                       "frame id has unexpected type"))
        }
    }
}

impl ToJson for FrameId {
    fn to_json(&self) -> json::Json {
        match *self {
            FrameId::Short(x) => {
                json::Json::U64(x as u64)
            },
            FrameId::Element(ref x) => {
                json::Json::String(x.id.clone())
            },
            FrameId::Null => {
                json::Json::Null
            }
        }
    }
}

#[deriving(PartialEq)]
pub enum LocatorStrategy {
    CSSSelector,
    LinkText,
    PartialLinkText,
    XPath
}

impl LocatorStrategy {
    pub fn from_json(body: &json::Json) -> WebDriverResult<LocatorStrategy> {
        match try_opt!(body.as_string(),
                       ErrorStatus::InvalidArgument,
                       "Cound not convert strategy to string") {
            "css selector" => Ok(LocatorStrategy::CSSSelector),
            "link text" => Ok(LocatorStrategy::LinkText),
            "partial link text" => Ok(LocatorStrategy::PartialLinkText),
            "xpath" => Ok(LocatorStrategy::XPath),
            _ => Err(WebDriverError::new(ErrorStatus::InvalidArgument,
                                         "Unknown locator strategy"))
        }
    }
}

impl ToJson for LocatorStrategy {
    fn to_json(&self) -> json::Json {
        json::Json::String(match *self {
            LocatorStrategy::CSSSelector => "css selector",
            LocatorStrategy::LinkText => "link text",
            LocatorStrategy::PartialLinkText => "partial link text",
            LocatorStrategy::XPath => "xpath"
        }.into_string())
    }
}
