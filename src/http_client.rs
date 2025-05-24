use curl::easy::Easy;

pub struct HttpClient;

pub struct Response {
    pub status_code: u32,
    pub body: String,
}

impl HttpClient {
    /// Fetches the content of the given URL and returns it as a `String`.
    pub fn fetch_url(url: &str) -> Result<Response, String> {
        let mut data = Vec::new();
        let mut easy = Easy::new();

        easy.url(url).map_err(|e| e.to_string())?;
        {
            let mut transfer = easy.transfer();
            transfer
                .write_function(|new_data| {
                    data.extend_from_slice(new_data);
                    Ok(new_data.len())
                })
                .map_err(|e| e.to_string())?;
            transfer.perform().map_err(|e| e.to_string())?;
        }

        let status_code = easy.response_code().map_err(|e| e.to_string());
        let body = String::from_utf8(data).map_err(|e| e.to_string());
        if status_code.is_err() || body.is_err() {
            return Err("Failed to fetch URL".to_string());
        }

        Ok(Response {
            status_code: status_code.unwrap(),
            body: body.unwrap(),
        })
    }
}
