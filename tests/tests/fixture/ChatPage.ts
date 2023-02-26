import { Locator, Page } from '@playwright/test';

export default class ChatPage {
    readonly page: Page;
    readonly chatMessages: Locator;

    constructor(page: Page) {
        this.page = page;
        this.chatMessages = page.getByRole('listitem');
    }

    async goto() {
        await this.page.goto('/');
        await this.page.getByRole('link', { name: 'Chat' }).click();
    }

    async goBack() {
        await this.page.getByRole('link').click();
    }

    async registerWithUserName(name: string, mode: 'click' | 'pressEnter' = 'pressEnter') {
        await this.page.getByLabel('Enter your name').click();
        await this.page.getByLabel('Enter your name').fill(name);
        mode == 'pressEnter'
            ? await this.page.getByLabel('Enter your name').press('Enter')
            : await this.page.getByRole('button').click();
    }

    async sendMessage(message: string, mode: 'click' | 'pressEnter' = 'pressEnter') {
        await this.page.getByPlaceholder('Enter your message').click();
        await this.page.getByPlaceholder('Enter your message').fill(message);
        await this.page.getByPlaceholder('Enter your message').press('Enter');
        mode == 'pressEnter'
            ? await this.page.getByPlaceholder('Enter your message').press('Enter')
            : await this.page.getByRole('button').click();
    }
}
