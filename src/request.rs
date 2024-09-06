pub struct Request {
    pub method: String,
    pub resource: Resource,
    pub headers: Vec<(String, String)>,
}

pub struct Resource {
    pub path: String,
}

impl Request {
    pub fn from_buffer(buffer: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let request_str = String::from_utf8_lossy(buffer);
        let lines: Vec<&str> = request_str.lines().collect();

        if lines.is_empty() {
            return Err("Empty request".into());
        }

        let first_line: Vec<&str> = lines[0].split_whitespace().collect();
        if first_line.len() < 2 {
            return Err("Invalid request line".into());
        }

        let method = first_line[0].to_string();
        let path = first_line[1].to_string();

        let headers = lines[1..]
            .iter()
            .take_while(|line| !line.is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line.splitn(2, ": ").collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            })
            .collect();

        Ok(Request {
            method,
            resource: Resource { path },
            headers,
        })
    }
}
