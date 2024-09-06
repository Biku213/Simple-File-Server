use std::env;
use std::error::Error;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use infer::Infer;
use url_escape::decode;

mod request;
mod response;
use crate::request::Request;
use crate::response::Response;

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:5500")?;
    println!("Server listening on http://127.0.0.1:5500");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    eprintln!("Error handling connection: {}", e);
                }
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let request = Request::from_buffer(&buffer)?;
    let response = handle_request(&request)?;

    stream.write_all(response.to_bytes().as_slice())?;
    stream.flush()?;

    Ok(())
}

fn handle_request(request: &Request) -> Result<Response, Box<dyn Error>> {
    let root_cwd = env::current_dir()?;
    
    let decoded_path = decode(&request.resource.path).into_owned();
    let resource_path = PathBuf::from(decoded_path.trim_start_matches('/'));
    let resource = root_cwd.join(resource_path);

    if !resource.exists() {
        return Ok(Response::not_found());
    }

    // Check if the resource is within or equal to the root directory
    if !resource.starts_with(&root_cwd) {
        return Ok(Response::forbidden());
    }

    if resource.is_dir() {
        Ok(handle_directory(&resource)?)
    } else {
        Ok(handle_file(&resource)?)
    }
}

fn handle_directory(dir: &Path) -> Result<Response, Box<dyn Error>> {
    let mut entries = Vec::new();
    
    // Add parent directory link if not in root
    if dir != env::current_dir()? {
        entries.push(("..".to_string(), "Parent Directory".to_string(), true));
    }

    for entry in WalkDir::new(dir).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        let is_dir = path.is_dir();
        let relative_path = path.strip_prefix(env::current_dir()?).unwrap_or(path).to_string_lossy().to_string();
        entries.push((relative_path, name, is_dir));
    }

    // Sort entries: directories first, then files, both in alphabetical order
    entries.sort_by(|a, b| {
        match (a.2, b.2) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.1.cmp(&b.1),
        }
    });

    let content = entries.into_iter().map(|(path, name, is_dir)| {
        let icon = if is_dir { "📁" } else { "📄" };
        format!(
            r#"<li><a href="{}"><span class="icon">{}</span><span class="name">{}</span></a></li>"#,
            path,
            icon,
            name
        )
    }).collect::<Vec<_>>().join("\n");

    let body = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Directory listing for {}</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
        }}
        h1 {{
            border-bottom: 1px solid #ccc;
            padding-bottom: 10px;
        }}
        ul {{
            list-style-type: none;
            padding: 0;
        }}
        li {{
            margin-bottom: 10px;
        }}
        a {{
            text-decoration: none;
            color: #0066cc;
            display: flex;
            align-items: center;
        }}
        a:hover {{
            text-decoration: underline;
        }}
        .icon {{
            margin-right: 10px;
            font-size: 1.2em;
        }}
        .name {{
            flex-grow: 1;
        }}
    </style>
</head>
<body>
    <h1>Directory listing for {}</h1>
    <ul>
    {}
    </ul>
</body>
</html>"#,
        dir.to_string_lossy(),
        dir.to_string_lossy(),
        content
    );

    Ok(Response::ok(body.into_bytes(), Some("text/html")))
}

fn handle_file(file: &Path) -> Result<Response, Box<dyn Error>> {
    let content = fs::read(file)?;
    let extension = file.extension().and_then(|e| e.to_str()).unwrap_or("");
    let mime_type = get_mime_type(extension);

    let mut response = Response::ok(content, Some(mime_type));
    
    if should_display_in_browser(mime_type) {
        response.add_header("Content-Disposition", "inline");
    } else {
        let filename = file.file_name().unwrap_or_default().to_string_lossy();
        response.add_header("Content-Disposition", &format!("attachment; filename=\"{}\"", filename));
    }

    Ok(response)
}

fn should_display_in_browser(mime_type: &str) -> bool {
    let displayable_types = [
        "text/",
        "image/",
        "audio/",
        "video/",
        "application/pdf",
        "application/json",
        "application/xml",
    ];

    displayable_types.iter().any(|&t| mime_type.starts_with(t))
}

fn get_mime_type(extension: &str) -> &'static str {
    match extension.to_lowercase().as_str() {
        "txt" => "text/plain",
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "pdf" => "application/pdf",
        "mp3" => "audio/mpeg",
        "mp4" => "video/mp4",
        _ => "application/octet-stream",
    }
}