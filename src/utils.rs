pub fn get_content_type(filename: &str) -> &'static str {
    let ext = filename.split('.').last().unwrap_or("");
    match ext {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        _ => "application/octet-stream",
    }
}