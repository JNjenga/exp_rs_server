
#[derive(Debug)]
pub enum Method
{
    GET,
    POST,
}

#[derive(Debug)]
pub struct Response
{
    pub version:       String,
    pub status:        String,
    pub body:          String,
}

#[derive(Debug)]
pub struct Request
{
    pub method:  Method,
    pub route:   String,
}

impl std::string::ToString for Response
{
    fn to_string(&self) -> String
    {
        String::from(format!("HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
                             self.status,
                             self.body.len(),
                             self.body))
    }
}

impl std::str::FromStr for Request
{
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let string = s.replace("\r", "");
        let v: Vec<&str>= string.split(&[' ', '\n'][..]).collect();

        let method = Method::GET;
        if v[0] == "POST"
        {
            let method = Method::POST;
        }

        Ok(Request{ method:  method,
                    route:   String::from(v[1]),
                    })
    }
}
