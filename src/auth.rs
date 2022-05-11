const TOKEN_START_INDEX: usize = "Bearer ".len();

pub fn parse_bearer_token(header: &str) -> Option<uuid::Uuid> {
    if header.starts_with("Bearer ") {
        uuid::Uuid::parse_str(&header[TOKEN_START_INDEX..]).ok()
    } else {
        None
    }
}
