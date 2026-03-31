// Fuzz targets for Bead oya-frontend-rb4
// Test parsers and deserializers with arbitrary input

// ============================================================================
// ClassList Fuzzer
// ============================================================================

#[cfg(feature = "fuzz")]
#[cfg(fuzzing)]
#[fuzz_target(path = "fuzz/fuzz_targets/classlist_fuzz.rs")]
fn fuzz_classlist(data: &[u8]) {
    let input = String::from_utf8_lossy(data).to_string();
    let _result = oya_frontend::migration::ClassList::from_string(&input);
    // No panics allowed - ClassList should handle any input gracefully
}

// ============================================================================
// CssToken Fuzzer
// ============================================================================

#[cfg(feature = "fuzz")]
#[cfg(fuzzing)]
#[fuzz_target(path = "fuzz/fuzz_targets/css_token_fuzz.rs")]
fn fuzz_css_token(data: &[u8]) {
    let token = String::from_utf8_lossy(data).to_string();
    let approved_set: std::collections::HashSet<String> =
        std::collections::HashSet::from_iter(vec![
            "flex".to_string(),
            "items-center".to_string(),
            "h-screen".to_string(),
            "w-full".to_string(),
        ]);
    let _result = oya_frontend::migration::CssToken::from_string(&token, &approved_set);
    // No panics allowed - CssToken should handle any input gracefully
}

// ============================================================================
// ZoomFactor Fuzzer
// ============================================================================

#[cfg(feature = "fuzz")]
#[cfg(fuzzing)]
#[fuzz_target(path = "fuzz/fuzz_targets/zoomfactor_fuzz.rs")]
fn fuzz_zoomfactor(data: &[u8]) {
    let input = String::from_utf8_lossy(data).to_string();
    if let Ok(value) = input.parse::<f32>() {
        let _result = oya_frontend::migration::ZoomFactor::from_f32(value);
    } else {
        // Invalid number - should not panic
        let _result = oya_frontend::migration::ZoomFactor::from_f32(f32::NAN);
    }
    // No panics allowed
}

// ============================================================================
// Px Fuzzer
// ============================================================================

#[cfg(feature = "fuzz")]
#[cfg(fuzzing)]
#[fuzz_target(path = "fuzz/fuzz_targets/px_fuzz.rs")]
fn fuzz_px(data: &[u8]) {
    let input = String::from_utf8_lossy(data).to_string();
    if let Ok(value) = input.parse::<f32>() {
        let _result = oya_frontend::migration::Px::new(value);
    } else {
        // Invalid number - should not panic
        let _result = oya_frontend::migration::Px::new(f32::NAN);
    }
    // No panics allowed
}

// ============================================================================
// FlowPosition Fuzzer
// ============================================================================

#[cfg(feature = "fuzz")]
#[cfg(fuzzing)]
#[fuzz_target(path = "fuzz/fuzz_targets/flowposition_fuzz.rs")]
fn fuzz_flowposition(data: &[u8]) {
    let input = String::from_utf8_lossy(data).to_string();
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() >= 2 {
        if let (Ok(x), Ok(y)) = (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
            let _result = oya_frontend::migration::FlowPosition::new(x, y);
        } else {
            let _result = oya_frontend::migration::FlowPosition::new(f32::NAN, f32::NAN);
        }
    } else {
        let _result = oya_frontend::migration::FlowPosition::new(f32::NAN, f32::NAN);
    }
    // No panics allowed
}

// ============================================================================
// NodeId Fuzzer
// ============================================================================

#[cfg(feature = "fuzz")]
#[cfg(fuzzing)]
#[fuzz_target(path = "fuzz/fuzz_targets/nodeid_fuzz.rs")]
fn fuzz_nodeid(data: &[u8]) {
    let uuid_str = String::from_utf8_lossy(data).to_string();
    let _result = oya_frontend::migration::NodeId::new(&uuid_str);
    // No panics allowed - NodeId should handle any string gracefully
}
