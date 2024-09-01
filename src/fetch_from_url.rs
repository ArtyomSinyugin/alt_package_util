use reqwest::{blocking::get, Error};
use serde_json::from_str;

use crate::{PackageInfo, ApiResponse};

pub fn fetch_packages(url: &str) -> Result<Vec<PackageInfo>, Error> {
    let response = get(url)?.text()?;
    let api_response: ApiResponse = from_str(&response).expect("Could not deserialize from request to ApiResponse struct");
    Ok(api_response.packages)
}