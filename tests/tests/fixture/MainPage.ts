import { Locator, Page } from '@playwright/test';

export default class MainPage {
    readonly page: Page;
    readonly lyngLink: Locator;
    readonly chatLink: Locator;

    constructor(page: Page) {
        this.page = page;
        this.lyngLink = this.page.getByRole('link', { name: 'Lyng' });
        this.chatLink = this.page.getByRole('link', { name: 'Chat' });
    }

    async goto() {
        await this.page.goto('/');
    }
}
