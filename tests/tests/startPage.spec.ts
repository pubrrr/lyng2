import { expect } from '@playwright/test';
import { lyngTest } from './fixture';

lyngTest('Lyng main page shows links', async ({ mainPage }) => {
    await expect(mainPage.page).toHaveTitle(/Lyng/);
    await expect(mainPage.lyngLink).toBeVisible();
    await expect(mainPage.chatLink).toBeVisible();
});
