use serialize::json;
use serialize::json::ToJson;

use common::Nullable;

#[deriving(Show)]
pub enum WebDriverResponse {
    NewSession(NewSessionResponse),
    DeleteSession,
    WindowSize(WindowSizeResponse),
    ElementRect(ElementRectResponse),
    Cookie(CookieResponse),
    Generic(ValueResponse),
    Void
}

impl WebDriverResponse {
    pub fn to_json_string(self) -> String {
        match self {
            WebDriverResponse::NewSession(x) => json::encode(&x),
            WebDriverResponse::DeleteSession => "".into_string(),
            WebDriverResponse::WindowSize(x) => json::encode(&x),
            WebDriverResponse::ElementRect(x) => json::encode(&x),
            WebDriverResponse::Cookie(x) => json::encode(&x),
            WebDriverResponse::Generic(x) => json::encode(&x),
            WebDriverResponse::Void => "".into_string()
        }
    }
}

#[deriving(Encodable, Show)]
pub struct NewSessionResponse {
    sessionId: String,
    value: json::Json
}

impl NewSessionResponse {
    pub fn new(session_id: String, value: json::Json) -> NewSessionResponse {
        NewSessionResponse {
            value: value,
            sessionId: session_id
        }
    }
}

#[deriving(Encodable, Show)]
pub struct ValueResponse {
    value: json::Json
}

impl ValueResponse {
    pub fn new(value: json::Json) -> ValueResponse {
        ValueResponse {
            value: value
        }
    }
}

#[deriving(Encodable, Show)]
pub struct WindowSizeResponse {
    width: u64,
    height: u64
}

impl WindowSizeResponse {
    pub fn new(width: u64, height: u64) -> WindowSizeResponse {
        WindowSizeResponse {
            width: width,
            height: height
        }
    }
}

#[deriving(Encodable, Show)]
pub struct ElementRectResponse {
    x: u64,
    y: u64,
    width: u64,
    height: u64
}

impl ElementRectResponse {
    pub fn new(x: u64, y: u64, width: u64, height: u64) -> ElementRectResponse {
        ElementRectResponse {
            x: x,
            y: y,
            width: width,
            height: height
        }
    }
}

#[deriving(Encodable, PartialEq, Show)]
pub struct Date(u64);

impl Date {
    pub fn new(timestamp: u64) -> Date {
        Date(timestamp)
    }
}

impl ToJson for Date {
    fn to_json(&self) -> json::Json {
        let &Date(x) = self;
        x.to_json()
    }
}

//TODO: some of these fields are probably supposed to be optional
#[deriving(Encodable, PartialEq, Show)]
pub struct Cookie {
    name: String,
    value: String,
    path: Nullable<String>,
    domain: Nullable<String>,
    expiry: Nullable<Date>,
    maxAge: Date,
    secure: bool,
    httpOnly: bool
}

impl Cookie {
    pub fn new(name: String, value: String, path: Nullable<String>, domain: Nullable<String>,
               expiry: Nullable<Date>, max_age: Date, secure: bool, http_only: bool) -> Cookie {
        Cookie {
            name: name,
            value: value,
            path: path,
            domain: domain,
            expiry: expiry,
            maxAge: max_age,
            secure: secure,
            httpOnly: http_only
        }
    }
}

#[deriving(Encodable, Show)]
pub struct CookieResponse {
    value: Vec<Cookie>
}

impl CookieResponse {
    pub fn new(value: Vec<Cookie>) -> CookieResponse {
        CookieResponse {
            value: value
        }
    }
}
