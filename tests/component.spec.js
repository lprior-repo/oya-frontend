const { test, expect } = require('@playwright/test');

test.describe('Flow-WASM Component Parity', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8081');
    // Wait for the shell rendered by our Dioxus components
    await page.waitForSelector('.app-shell', { timeout: 15000 });
  });

  test('should open settings sidebar when node is clicked', async ({ page }) => {
    // Add node
    await page.click('text=HTTP Request');
    
    // Select node
    const node = page.locator('.node-card').first();
    await node.click();
    
    // Check for sidebar
    const sidebar = page.locator('aside >> text=Node Settings');
    await expect(sidebar).toBeVisible();
    
    // Check for expression badge
    await expect(page.locator('text=EXPRESSION')).toBeVisible();
  });

  test('should delete a node', async ({ page }) => {
    await page.click('text=HTTP Request');
    const nodeCount = await page.locator('.node-card').count();
    
    await page.hover('.node-card');
    await page.click('text=×');
    
    await expect(page.locator('.node-card')).toHaveCount(nodeCount - 1);
  });
});
