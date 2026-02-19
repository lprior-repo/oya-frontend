use playwright::Playwright;
use tokio;
use std::path::Path;

#[tokio::test]
async fn test_flow_wasm_ui() -> Result<(), Box<dyn std::error::Error>> {
    let playwright = Playwright::initialize().await?;
    
    let chromium = playwright.chromium();
    let browser = chromium.launcher().launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;

    // Serve via Rust/Dioxus CLI (handled in bash)
    page.goto_builder("http://localhost:8081").goto().await?;

    // Wait for the app to load - using query_selector as it is more direct in this crate version
    let _ = page.query_selector("aside").await?.expect("Sidebar should be visible");

    // Click "HTTP Request"
    page.click_builder("text=HTTP Request").click().await?;

    // Verify node card exists
    let node_card = page.query_selector(".node-card").await?.expect("Node card should be created");
    
    // Click node card to open settings
    node_card.click_builder().click().await?;

    // Verify settings sidebar appears
    let _ = page.query_selector("text=Node Settings").await?.expect("Settings sidebar should open");

    // Capture screenshot
    page.screenshot_builder()
        .path(Path::new("screenshot_rust.png").to_path_buf())
        .screenshot()
        .await?;

    println!("UI Validation Successful! Final check: RUST ONLY.");

    browser.close().await?;
    Ok(())
}
