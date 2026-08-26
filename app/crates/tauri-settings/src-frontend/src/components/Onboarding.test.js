import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent, screen } from '@testing-library/svelte';
import Onboarding from './Onboarding.svelte';

beforeEach(() => {
    // Mock Tauri internals
    window.__TAURI_INTERNALS__ = {
        invoke: vi.fn().mockImplementation((cmd) => {
            if (cmd === 'check_ime_status') return Promise.resolve('enabled');
            if (cmd === 'enable_ime') return Promise.resolve('enabled');
            return Promise.resolve('');
        }),
    };
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

    it('navigates to setup step', async () => {
        const { container } = render(Onboarding, { props: { oncomplete: vi.fn() } });
        const next = getNextButton(container);
        expect(next).toBeTruthy();

        await fireEvent.click(next);

        const title = container.querySelector('.step-title');
        expect(title.textContent).toContain('almost ready');
    });

    it('shows hotkeys on setup step', async () => {
        const { container } = render(Onboarding, { props: { oncomplete: vi.fn() } });
        const next = getNextButton(container);

        await fireEvent.click(next);

        const hotkeyCards = container.querySelectorAll('.hotkey-card');
        expect(hotkeyCards.length).toBeGreaterThan(0);
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

    it('checks IME status when entering setup step', async () => {
        const { container } = render(Onboarding, { props: { oncomplete: vi.fn() } });
        const next = getNextButton(container);
        expect(next).toBeTruthy();

        await fireEvent.click(next);

        expect(window.__TAURI_INTERNALS__.invoke).toHaveBeenCalledWith('check_ime_status');
    });
});
