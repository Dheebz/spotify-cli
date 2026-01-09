use reqwest::StatusCode;

pub fn format_api_error(operation: &str, status: StatusCode, body: &str) -> String {
    let mut message = format!("{operation}: {} {}", status, body);

    if body.contains("Insufficient client scope") {
        message.push_str("; hint: missing scope, re-run `spotify auth login` and approve scopes");
    } else if status == StatusCode::UNAUTHORIZED {
        message.push_str("; hint: token expired or invalid, run `spotify auth login`");
    } else if status == StatusCode::FORBIDDEN {
        message.push_str("; hint: playlist may be read-only or missing modify scope, re-run `spotify auth login`");
    }

    message
}

#[cfg(test)]
mod tests {
    use super::format_api_error;
    use reqwest::StatusCode;

    #[test]
    fn adds_scope_hint() {
        let message = format_api_error(
            "spotify search failed",
            StatusCode::FORBIDDEN,
            r#"{"error":{"message":"Insufficient client scope"}}"#,
        );
        assert!(message.contains("missing scope"));
    }

    #[test]
    fn adds_unauthorized_hint() {
        let message = format_api_error("spotify request failed", StatusCode::UNAUTHORIZED, "{}");
        assert!(message.contains("token expired"));
    }
}
