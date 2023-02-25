import { test, expect } from "@playwright/test";

test("Lyng main page shows links", async ({ page }) => {
    await page.goto("/");

    await expect(page).toHaveTitle(/Lyng/);
    await expect(page.getByRole("link", { name: "Lyng" })).toBeVisible();
    await expect(page.getByRole("link", { name: "Chat" })).toBeVisible();
});
