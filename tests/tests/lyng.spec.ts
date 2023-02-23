import { test, expect } from "@playwright/test";

test.describe("Lyng", () => {
    test.beforeEach(async ({ page }) => {
        await page.goto("http://localhost:8080/");
        await page.getByRole("link", { name: "Lyng" }).click();
    });

    test("edit text in the editor", async ({ page }) => {
        await expect(page.getByText("1+2+3+4")).toBeVisible();

        await page.getByRole("textbox").filter({ hasText: "1+2+3+4" }).fill("1+2+3+4+5");

        await expect(page.getByText("((((1 + 2) + 3) + 4) + 5)")).toBeVisible();
    });
});
