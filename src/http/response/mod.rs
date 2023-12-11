#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum Status {
    Ok = 200,
    NotFound = 404,
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Ok => std::fmt::write(f, format_args!("{} OK", *self as u8)),
            Status::NotFound => std::fmt::write(f, format_args!("{} NotFound", *self as usize)),
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
impl TryFrom<super::request::Startline> for Startline {
    type Error = String;

    fn try_from(value: super::request::Startline) -> Result<Self, Self::Error> {
        match value.method {
            super::request::Method::Get => {
                let version = value.version;
                let status = match value.target.path.split_once('/') {
                    Some((_should_be_empty, path)) => match path.split_once('/') {
                        Some(("echo", _payload)) => Status::Ok,
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
                Ok(response_startline)
            }
            super::request::Method::Post => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}
