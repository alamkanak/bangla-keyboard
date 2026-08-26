import { describe, it, expect } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import TryTyping from './TryTyping.svelte';

describe('TryTyping', () => {
    it('renders textarea with placeholder', () => {
        const { container } = render(TryTyping, { props: { layout: 'phonetic' } });
        const textarea = container.querySelector('textarea');
        expect(textarea).toBeInTheDocument();
    });

    it('shows success when Bengali text is typed', async () => {
        const { container } = render(TryTyping, { props: { layout: 'phonetic' } });
        const textarea = container.querySelector('textarea');

        await fireEvent.input(textarea, { target: { value: 'আমি বাংলায় লিখছি' } });

        const success = container.querySelector('.try-success');
        expect(success).toBeInTheDocument();
    });

    it('does not show success for English text', async () => {
        const { container } = render(TryTyping, { props: { layout: 'phonetic' } });
        const textarea = container.querySelector('textarea');

        await fireEvent.input(textarea, { target: { value: 'Hello world' } });

        const success = container.querySelector('.try-success');
        expect(success).not.toBeInTheDocument();
    });

    it('shows mac hint on macOS', () => {
        Object.defineProperty(navigator, 'platform', { value: 'MacIntel', configurable: true });
        const { container } = render(TryTyping, { props: { layout: 'phonetic' } });
        const hintText = container.querySelector('.hint-text');
        expect(hintText.textContent).toContain('Globe');
    });
});
