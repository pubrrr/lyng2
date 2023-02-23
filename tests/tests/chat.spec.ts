import { test, expect } from "@playwright/test";

test.describe("Chat", () => {
    test.beforeEach(async ({ page }) => {
        await page.goto("http://localhost:8080/");
        await page.getByRole("link", { name: "Chat" }).click();
    });

    test("should be on the chat page", async ({ page }) => {
        await expect(page.getByText("Lyng Chat")).toBeVisible();
    });

    test("go back to main page", async ({ page }) => {
        await page.getByRole("link").click();

        await expect(page.getByRole("link", { name: "Lyng" })).toBeVisible();
        await expect(page.getByRole("link", { name: "Chat" })).toBeVisible();
    });

    test("enter the name and send a message", async ({ page }) => {
        await page.getByLabel("Enter your name").click();
        await page.getByLabel("Enter your name").fill("test name");
        await page.getByLabel("Enter your name").press("Enter");

        await expect(page.getByText("Lyng Chat - test name")).toBeVisible();

        await page.getByPlaceholder("Enter your message").click();
        await page.getByPlaceholder("Enter your message").fill("hello world");
        await page.getByPlaceholder("Enter your message").press("Enter");

        await expect(page.getByRole("listitem").filter({ hasText: /hello world/ })).toBeVisible();
    });
});
