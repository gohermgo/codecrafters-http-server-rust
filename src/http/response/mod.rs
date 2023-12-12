use std::fmt;
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum Status {
    Ok = 200,
    Created = 201,
    NotFound = 404,
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Status::Ok => format!("{} OK", *self as isize),
            Status::Created => format!("{} Created", *self as isize),
            Status::NotFound => format!("{} NotFound", *self as isize),
        };
        fmt::write(f, format_args!("{}", s))
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
                let status = match value.target.path {
                    // Match index
                    s if s.eq("/") => Status::Ok,
                    s => {
                        let request_components = s
                            .split('/')
                            .filter(|s| !s.is_empty())
                            .collect::<Vec<&str>>();
                        match request_components.first() {
                            Some(root) if root.eq(&String::from("echo")) => Status::Ok,
                            Some(_otherpath) => Status::NotFound,
                            None => Status::NotFound,
                        }
                    }
                };
                // let status = match value.target.path.split_once('/') {
                //     Some((_should_be_empty, path)) => match path.split_once('/') {
                //         Some(("echo", _payload)) => Status::Ok,
                //         Some((root, path)) => {
                //             log_from_mod!("root : {}", root);
                //             log_from_mod!("path : {}", path);
                //             Status::NotFound
                //         }
                //         _ => Status::NotFound,
                //     },
                //     None => Status::Ok,
                // };
                let response_startline = Self { version, status };
                Ok(response_startline)
            }
            super::request::Method::Post => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}
