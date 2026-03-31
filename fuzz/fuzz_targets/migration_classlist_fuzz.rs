// Fuzz target for ClassList parser

#[fuzz_target(path = "fuzz/fuzz_targets/migration_classlist_fuzz.rs")]
fn fuzz_classlist(data: &[u8]) {
    let input = String::from_utf8_lossy(data).to_string();
    let _result = oya_frontend::migration::ClassList::from_string(&input);
    // No panics allowed - ClassList should handle any input gracefully
}
