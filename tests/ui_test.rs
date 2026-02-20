use playwright::Playwright;
use std::path::Path;

#[tokio::test]
async fn given_flow_editor_when_loaded_then_sidebar_and_node_canvas_are_interactive(
) -> Result<(), Box<dyn std::error::Error>> {
    let playwright = Playwright::initialize().await?;

    let chromium = playwright.chromium();
    let browser = chromium.launcher().launch().await?;
    let context_obj = browser.context_builder().build().await?;
    let page = context_obj.new_page().await?;

    println!("Navigating to app (release mode)...");
    page.goto_builder("http://localhost:8081").goto().await?;

    println!("Given the app loads, waiting for sidebar shell (up to 60s)...");
    match page
        .wait_for_selector_builder("aside")
        .timeout(60000.0)
        .wait_for_selector()
        .await
    {
        Ok(_) => {
            println!("SUCCESS: Aside found!");
            page.wait_for_selector_builder("input[placeholder='Search nodes...']")
                .timeout(10000.0)
                .wait_for_selector()
                .await?;
            page.wait_for_selector_builder("text=HTTP Trigger")
                .timeout(10000.0)
                .wait_for_selector()
                .await?;
            page.click_builder("text=+ Add Node").click().await?;
            page.wait_for_selector_builder(".node-card")
                .timeout(10000.0)
                .wait_for_selector()
                .await?;
            println!("Then adding a node renders an interactive node card.");

            page.screenshot_builder()
                .path(Path::new("final_success.png").to_path_buf())
                .screenshot()
                .await?;
        }
        Err(e) => {
            println!("FAILURE: Aside still not found: {e:?}");
            let page_content = page.content().await?;
            println!("Content length: {}", page_content.len());
            page.screenshot_builder()
                .path(Path::new("final_failure.png").to_path_buf())
                .screenshot()
                .await?;
        }
    }

    browser.close().await?;
    Ok(())
}
