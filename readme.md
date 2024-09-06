# Simple File Server

- This is a simple file server implemented in Rust. 
- It serves files and directories as a simple HTML document, listing all directories and files as clickable links.

## Features

- Serves files and directories from the current working directory
- Generates an HTML interface for browsing directories
- Supports various file types with appropriate MIME types
- Handles file downloads for non-displayable file types
- Provides a clean and responsive user interface

## Requirements

- Rust (latest stable version)
- Dependencies (as listed in `Cargo.toml`):
  - `std`
  - `walkdir`
  - `infer`
  - `url_escape`

## Usage

1. Clone the repository:
   ```
   git clone <repository-url>
   cd <repository-name>
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the server:
   ```
   cargo run --release
   ```

4. Open a web browser and navigate to `http://127.0.0.1:5500`

## How it works

- The server listens on `127.0.0.1:5500`
- It serves files from the current working directory
- Directories are displayed as an HTML page with clickable links
- Files are served with appropriate MIME types
- Non-displayable files are offered as downloads

## Security Considerations

- The server only serves files within or below the current working directory
- Attempts to access files outside this directory will result in a "Forbidden" error

## Customization

You can customize the server by modifying the `main.rs` file:

- Change the listening address/port in the `main()` function
- Modify the HTML template in the `handle_directory()` function
- Adjust MIME types or add new ones in the `get_mime_type()` function