use std::path::Path;

fn hash(message: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(message);
    let result = hasher.result_str();
    debug!("Hash of '{}' is: {}", message, result);
    result
}
