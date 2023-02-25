import { expect } from '@playwright/test';
import { lyngTest } from './fixture';
import ChatPage from './fixture/ChatPage';

lyngTest.describe('Chat', () => {
    lyngTest.describe('single user', () => {
        lyngTest('should be on the chat page', async ({ chatPage }) => {
            await expect(chatPage.page.getByText('Lyng Chat')).toBeVisible();
        });

        lyngTest('go back to main page', async ({ chatPage, mainPage }) => {
            await chatPage.goto();
            await chatPage.goBack();

            await expect(mainPage.lyngLink).toBeVisible();
            await expect(mainPage.chatLink).toBeVisible();
        });

        for (const confirmMode of ['click', 'pressEnter'] as const) {
            lyngTest(`enter the name and send a message for confirmation mode ${confirmMode}`, async ({ chatPage }) => {
                await chatPage.registerWithUserName('test name', confirmMode);

                await expect(chatPage.page.getByText('Lyng Chat - test name')).toBeVisible();

                await chatPage.sendMessage('hello world', confirmMode);

                await expect(chatPage.chatMessages.filter({ hasText: /hello world/ }).first()).toBeVisible();
            });
        }
    });

    lyngTest.describe('user interaction', () => {
        lyngTest('send message to other user', async ({ browser }) => {
            const senderPage = new ChatPage(await browser.newContext().then((context) => context.newPage()));
            const receiverPage = new ChatPage(await browser.newContext().then((context) => context.newPage()));

            await senderPage.goto();
            await senderPage.registerWithUserName('sender name');
            await receiverPage.goto();
            await receiverPage.registerWithUserName('receiver name');

            await senderPage.sendMessage('hello receiver');

            await expect(senderPage.chatMessages.filter({ hasText: /hello receiver/ }).first()).toBeVisible();
            await expect(receiverPage.chatMessages.filter({ hasText: /hello receiver/ }).first()).toBeVisible();
        });

        lyngTest('show username of other user entering', async ({ browser, browserName }) => {
            const chatPage = new ChatPage(await browser.newContext().then((context) => context.newPage()));
            await chatPage.goto();
            await chatPage.registerWithUserName('my name');

            let otherUsersName = `user${Math.random()}-${browserName}`;
            const otherUsersPage = new ChatPage(await browser.newContext().then((context) => context.newPage()));
            await otherUsersPage.goto();
            await otherUsersPage.registerWithUserName(otherUsersName);

            await expect(chatPage.page.getByText(otherUsersName)).toBeVisible();
        });
    });
});
