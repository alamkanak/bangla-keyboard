import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent, screen } from '@testing-library/svelte';
import Onboarding from './Onboarding.svelte';

beforeEach(() => {
    Object.defineProperty(navigator, 'platform', { value: 'MacIntel', configurable: true });
});

function getNextButton(container) {
    const buttons = container.querySelectorAll('button');
    // The next/finish button is always the last button in the footer
    const allBtns = [...buttons];
    return allBtns[allBtns.length - 1];
}

function getFinishButton(container) {
    return getNextButton(container);
}

describe('Onboarding', () => {
    it('starts at language selection step', () => {
        const { container } = render(Onboarding, { props: { oncomplete: vi.fn() } });
        const title = container.querySelector('.step-title');
        expect(title.textContent).toContain('Choose your language');
    });

    it('navigates to switching step', async () => {
        const { container } = render(Onboarding, { props: { oncomplete: vi.fn() } });
        const next = getNextButton(container);
        expect(next).toBeTruthy();

        await fireEvent.click(next);

        const title = container.querySelector('.step-title');
        expect(title.textContent).toContain('Switching between');
    });

    it('shows hotkeys on switching step without enable button', async () => {
        const { container } = render(Onboarding, { props: { oncomplete: vi.fn() } });
        const next = getNextButton(container);

        await fireEvent.click(next);

        const hotkeyCards = container.querySelectorAll('.hotkey-card');
        expect(hotkeyCards.length).toBeGreaterThan(0);

        // No enable button or status card should exist
        const statusCard = container.querySelector('.status-card');
        expect(statusCard).toBeNull();
    });

    it('calls oncomplete with preferences on finish', async () => {
        const oncomplete = vi.fn();
        const { container } = render(Onboarding, { props: { oncomplete } });

        // Navigate through all 5 steps
        for (let i = 0; i < 4; i++) {
            const next = getNextButton(container);
            expect(next).toBeTruthy();
            await fireEvent.click(next);
        }

        // Click finish on the last step
        const finish = getFinishButton(container);
        expect(finish).toBeTruthy();
        await fireEvent.click(finish);

        expect(oncomplete).toHaveBeenCalledWith(
            expect.objectContaining({
                language: 'en',
                layout: 'phonetic',
                theme: 'dark',
            })
        );
    });

    it('does not invoke check_ime_status on switching step', async () => {
        const mockInvoke = vi.fn();
        window.__TAURI_INTERNALS__ = { invoke: mockInvoke };

        const { container } = render(Onboarding, { props: { oncomplete: vi.fn() } });
        const next = getNextButton(container);
        await fireEvent.click(next);

        expect(mockInvoke).not.toHaveBeenCalledWith('check_ime_status');

        delete window.__TAURI_INTERNALS__;
    });

    it('persists layout preference immediately when selected', async () => {
        const mockInvoke = vi.fn().mockResolvedValue('');
        window.__TAURI_INTERNALS__ = { invoke: mockInvoke };

        const { container } = render(Onboarding, { props: { oncomplete: vi.fn() } });
        // Navigate to layout step (step 2)
        await fireEvent.click(getNextButton(container)); // → switching
        await fireEvent.click(getNextButton(container)); // → layout

        // Select 'unibijoy' radio card
        const radioCards = container.querySelectorAll('[role="radio"], .radio-card');
        const unibijoyCard = [...radioCards].find(el => el.textContent.includes('UniBijoy'));
        if (unibijoyCard) {
            await fireEvent.click(unibijoyCard);
            expect(mockInvoke).toHaveBeenCalledWith('update_preference', { key: 'layout', value: 'unibijoy' });
        }

        delete window.__TAURI_INTERNALS__;
    });
});
