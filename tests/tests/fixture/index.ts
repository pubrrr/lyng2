import { test } from '@playwright/test';
import ChatPage from './ChatPage';
import MainPage from './MainPage';

export const lyngTest = test.extend<{ chatPage: ChatPage; mainPage: MainPage }>({
    chatPage: async ({ page }, use) => {
        const chatPage = new ChatPage(page);
        await chatPage.goto();
        await use(chatPage);
    },
    mainPage: async ({ page }, use) => {
        const mainPage = new MainPage(page);
        await mainPage.goto();
        await use(mainPage);
    },
});
