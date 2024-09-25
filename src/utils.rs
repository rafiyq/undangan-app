pub fn get_content_type(filename: &str) -> &'static str {
    let ext = filename.split('.').last().unwrap_or("");
    match ext {
        "css" => "text/css",
        "html" => "text/html",
        "js" => "application/javascript",
        "json" => "application/json",
        "ico" => "image/x-icon",
        "xml" => "application/xml",
        _ => "application/octet-stream",
    }
}