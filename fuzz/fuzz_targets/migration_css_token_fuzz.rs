// Fuzz target for CssToken parser

#[fuzz_target(path = "fuzz/fuzz_targets/migration_css_token_fuzz.rs")]
fn fuzz_css_token(data: &[u8]) {
    let token = String::from_utf8_lossy(data).to_string();
    let approved_set: std::collections::HashSet<String> = std::collections::HashSet::from_iter(vec![
        "flex".to_string(),
        "items-center".to_string(),
        "h-screen".to_string(),
        "w-full".to_string(),
    ]);
    let _result = oya_frontend::migration::CssToken::from_string(&token, &approved_set);
    // No panics allowed
}
