use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Finding {
    pub subdomain: String,
    pub sources: HashSet<String>,
}
