import { expect, Page, test } from '@playwright/test';

test.describe('Chat', () => {
    test.describe('single user', () => {
        test.beforeEach(async ({ page }) => {
            await page.goto('/');
            await page.getByRole('link', { name: 'Chat' }).click();
        });

        test('should be on the chat page', async ({ page }) => {
            await expect(page.getByText('Lyng Chat')).toBeVisible();
        });

        test('go back to main page', async ({ page }) => {
            await page.getByRole('link').click();

            await expect(page.getByRole('link', { name: 'Lyng' })).toBeVisible();
            await expect(page.getByRole('link', { name: 'Chat' })).toBeVisible();
        });

        test('enter the name and send a message', async ({ page }) => {
            await registerWithUserName('test name', page);

            await expect(page.getByText('Lyng Chat - test name')).toBeVisible();

            await page.getByPlaceholder('Enter your message').click();
            await page.getByPlaceholder('Enter your message').fill('hello world');
            await page.getByPlaceholder('Enter your message').press('Enter');

            await expect(
                page.getByRole('listitem').filter({ hasText: /hello world/ })
            ).toBeVisible();
        });

        test('enter the name and send a message via clicking on buttons', async ({ page }) => {
            await page.getByLabel('Enter your name').click();
            await page.getByLabel('Enter your name').fill('test name');
            await page.getByRole('button').click();

            await expect(page.getByText('Lyng Chat - test name')).toBeVisible();

            await sendMessage(page, 'hello world');

            await expect(
                page.getByRole('listitem').filter({ hasText: /hello world/ })
            ).toBeVisible();
        });
    });

    test.describe('user interaction', () => {
        test('send message to other user', async ({ browser }) => {
            const senderPage = await browser.newContext().then((context) => context.newPage());
            const receiverPage = await browser.newContext().then((context) => context.newPage());
            await senderPage.goto('/');
            await receiverPage.goto('/');
            await senderPage.getByRole('link', { name: 'Chat' }).click();
            await receiverPage.getByRole('link', { name: 'Chat' }).click();

            await registerWithUserName('sender name', senderPage);
            await registerWithUserName('receiver name', receiverPage);

            await sendMessage(senderPage, 'hello receiver');

            let messageOnSenderPage = senderPage
                .getByRole('listitem')
                .filter({ hasText: /hello receiver/ });
            await expect(messageOnSenderPage).toBeVisible();
            let messageOnReceiverPage = receiverPage
                .getByRole('listitem')
                .filter({ hasText: /hello receiver/ });
            await expect(messageOnReceiverPage).toBeVisible();
        });

        test('show username of other user entering', async ({ browser, browserName }) => {
            const page = await browser.newContext().then((context) => context.newPage());
            await page.goto('/');
            await page.getByRole('link', { name: 'Chat' }).click();

            await registerWithUserName('my name', page);

            let otherUsersName = "other user's name-" + browserName;
            const otherUsersPage = await browser.newContext().then((context) => context.newPage());
            await otherUsersPage.goto('/');
            await otherUsersPage.getByRole('link', { name: 'Chat' }).click();
            await registerWithUserName(otherUsersName, otherUsersPage);

            await expect(page.getByText(otherUsersName)).toBeVisible();
        });
    });

    async function sendMessage(page: Page, message: string) {
        await page.getByPlaceholder('Enter your message').click();
        await page.getByPlaceholder('Enter your message').fill(message);
        await page.getByRole('button').click();
    }

    async function registerWithUserName(name: string, page: Page) {
        await page.getByLabel('Enter your name').click();
        await page.getByLabel('Enter your name').fill(name);
        await page.getByLabel('Enter your name').press('Enter');
    }
});
