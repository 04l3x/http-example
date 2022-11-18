use crate::http::{Status, Version};

#[derive(Debug)]
pub struct Response {
    version: Version,
    status: Status,
    content: Option<String>,
}

impl Response {
    pub fn new(version: Version, status: Status, content: Option<&str>) -> Self {
        let content = if let Some(content) = content {
            Some(content.to_owned())
        } else {
            None
        };

        Self {
            version,
            status,
            content,
        }
    }
}

impl From<Response> for String {
    fn from(res: Response) -> Self {
        let (lenght, content) = if let Some(content) = res.content {
            (content.len(), content)
        } else {
            (0, String::new())
        };

        let version = res.version.to_string();

        let status = res.status.to_string();

        format!("{version} {status}\r\nContent-Lenght: {lenght}\r\n\r\n{content}")
    }
}
