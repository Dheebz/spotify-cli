//! User profile formatting functions

use serde_json::Value;

use crate::io::common::format_number;

pub fn format_user_profile(payload: &Value) {
    let display_name = payload
        .get("display_name")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");
    let email = payload.get("email").and_then(|v| v.as_str());
    let country = payload.get("country").and_then(|v| v.as_str());
    let product = payload
        .get("product")
        .and_then(|v| v.as_str())
        .unwrap_or("free");
    let followers = payload
        .get("followers")
        .and_then(|f| f.get("total"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let uri = payload.get("uri").and_then(|v| v.as_str()).unwrap_or("");

    println!("{}", display_name);
    if let Some(email) = email {
        println!("  Email: {}", email);
    }
    if let Some(country) = country {
        println!("  Country: {}", country);
    }
    println!("  Plan: {}", product);
    println!("  Followers: {}", format_number(followers));
    if !uri.is_empty() {
        println!("  URI: {}", uri);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_user_profile_full() {
        let payload = json!({
            "display_name": "Test User",
            "email": "test@example.com",
            "country": "US",
            "product": "premium",
            "followers": { "total": 1500 },
            "uri": "spotify:user:testuser"
        });
        format_user_profile(&payload);
    }

    #[test]
    fn format_user_profile_minimal() {
        let payload = json!({});
        format_user_profile(&payload);
    }

    #[test]
    fn format_user_profile_without_email() {
        let payload = json!({
            "display_name": "Test User",
            "country": "GB",
            "product": "free",
            "followers": { "total": 0 }
        });
        format_user_profile(&payload);
    }

    #[test]
    fn format_user_profile_without_country() {
        let payload = json!({
            "display_name": "Test User",
            "email": "test@example.com",
            "product": "premium"
        });
        format_user_profile(&payload);
    }

    #[test]
    fn format_user_profile_large_followers() {
        let payload = json!({
            "display_name": "Popular Artist",
            "followers": { "total": 5000000 }
        });
        format_user_profile(&payload);
    }
}
