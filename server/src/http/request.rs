use super::Method;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request{
    fn from_byte_array(buf: &u[8])->Result<Self,String>{    }
}

impl TryFrom
