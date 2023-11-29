#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum Status {
    Ok = 200,
    NotFound = 404,
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Ok => std::fmt::write(f, format_args!("{} OK\r\n", *self as u8)),
            Status::NotFound => std::fmt::write(f, format_args!("{} NotFound\r\n", *self as u8)),
        }
    }
}
pub struct Startline {
    pub version: super::Version,
    pub status: Status,
}
impl std::fmt::Display for Startline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::write(f, format_args!("{} {}\r\n", self.version, self.status))
    }
}
impl Startline {
    #[allow(dead_code)]
    pub fn try_constuct(request_startline: crate::http::request::Startline) -> Option<Self> {
        match request_startline.method {
            super::request::Method::Get => {
                let version = request_startline.version;
                let status = match request_startline.target.path.split_once('/') {
                    Some((should_be_empty, path)) => match path.split_once('/') {
                        Some(("echo", payload)) => Status::Ok,
                        Some((root, path)) => {
                            log_from_mod!("root : {}", root);
                            log_from_mod!("path : {}", path);
                            Status::NotFound
                        }
                        _ => Status::NotFound,
                    },
                    None => Status::Ok,
                };
                let response_startline = Self { version, status };
                Some(response_startline)
            }
            super::request::Method::Post => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}
