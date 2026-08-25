# National (Jatiya) Keyboard Layout — Full Specification

## Overview

The National (Jatiya) keyboard layout is the official Bangla keyboard standard for Bangladesh, designated as **BDS 1738:2018** by the Bangladesh Computer Council (BCC). It is a Unicode-compliant fixed keyboard layout designed for professional Bangla typing.

- **Developer**: Bangladesh Computer Council (BCC) — www.bccbd.org
- **Standard**: BDS 1738:2018 (Bangladesh Standard)
- **Type**: Fixed positional keyboard layout
- **Encoding**: Unicode (Bengali block U+0980–U+09FF)
- **Layers**: 4 (Normal, Shift, AltGr, Shift+AltGr)

---

## Design Philosophy

The National layout shares many similarities with the Bijoy/UniBijoy layout but has key differences:

1. **Consonants** are mostly on the right hand and top row
2. **Vowel signs (kars)** are on the left hand (home and bottom rows)
3. **Shift produces the aspirated/heavier variant** (ক→খ, গ→ঘ, ত→থ, etc.)
4. **AltGr layer** provides full vowel forms (ই, ঈ, উ, ঊ, etc.) and rare characters
5. **Hasanta (্)** is on the G key — the conjunct-forming key
6. **Backtick key** produces ZWNJ/ZWJ for controlling joining behavior

### Key Differences from UniBijoy

| Feature | UniBijoy | National (Jatiya) |
|---------|----------|-------------------|
| `F` key (normal) | া (aa-kar) | ব |
| `H` key (normal) | ব | া (aa-kar) |
| `F` key (shift) | অ | ভ |
| `H` key (shift) | ভ | অ |
| `A` key (shift) | © | ৗ (ou-kar) |
| `Z` key (normal) | ª | ঁ (chandrabindu) |
| `Z` key (shift) | ¨ | ঃ (visarga) |
| `X` key (normal) | ও | ো (o-kar) |
| `X` key (shift) | ৌ (ou-kar) | ৌ (ou-kar) |
| AltGr layer | Not present | Full vowels and rare chars |
| Backtick key | N/A | ZWNJ / ZWJ |

---

## Complete Key Mapping

### Alphabet Keys

| Physical Key | Normal | Shift | AltGr | Shift+AltGr |
|--------------|--------|-------|-------|-------------|
| Q | ঙ | ং | ৢ | ৣ |
| W | য | য় | — | — |
| E | ড | ঢ | ৄ | — |
| R | প | ফ | — | — |
| T | ট | ঠ | — | — |
| Y | চ | ছ | — | — |
| U | জ | ঝ | — | — |
| I | হ | ঞ | ঽ (avagraha) | — |
| O | গ | ঘ | — | — |
| P | ড় | ঢ় | — | — |
| A | ৃ (rri-kar) | ৗ (ou-kar) | ঋ | ৠ |
| S | ু (u-kar) | ূ (uu-kar) | উ | ঊ |
| D | ি (i-kar) | ী (ii-kar) | ই | ঈ |
| F | ব | ভ | ৰ | ৱ |
| G | ্ (hasanta) | । (dari) | ॥ (double dari) | — |
| H | া (aa-kar) | অ | আ | — |
| J | ক | খ | — | — |
| K | ত | থ | — | — |
| L | দ | ধ | ঌ | ৡ |
| Z | ঁ (chandrabindu) | ঃ (visarga) | ৺ | — |
| X | ো (o-kar) | ৌ (ou-kar) | ও | ঔ |
| C | ে (e-kar) | ৈ (oi-kar) | এ | ঐ |
| V | র | ল | — | — |
| B | ন | ণ | — | — |
| N | স | ষ | — | — |
| M | ম | শ | — | — |

### Number Row

| Physical Key | Normal | Shift | AltGr | Shift+AltGr |
|--------------|--------|-------|-------|-------------|
| `` ` `` (backtick) | ‌ (ZWNJ U+200C) | ‍ (ZWJ U+200D) | ‌ (ZWNJ) | ‍ (ZWJ) |
| 1 | ১ | ! | ৴ | — |
| 2 | ২ | @ | ৵ | — |
| 3 | ৩ | # | ৶ | — |
| 4 | ৪ | $ | ৳ (Taka sign) | ৲ |
| 5 | ৫ | % | ৷ | — |
| 6 | ৬ | ^ | ৸ | ৎ (khanda-ta) |
| 7 | ৭ | & | — | — |
| 8 | ৮ | * | — | — |
| 9 | ৯ | ( | — | — |
| 0 | ০ | ) | — | — |
| - | - | _ | — | — |
| = | = | + | — | — |

### Punctuation / OEM Keys

| Physical Key | Normal | Shift | AltGr | Shift+AltGr |
|--------------|--------|-------|-------|-------------|
| [ | [ | { | — | — |
| ] | ] | } | — | — |
| \\ | \\ | \| | — | — |
| ; | ; | : | — | — |
| ' | ' | " | — | — |
| , | , | < | — | — |
| . | . | > | — | — |
| / | / | ? | — | — |

### Numpad

| Key | Output |
|-----|--------|
| Num0 | ০ |
| Num1 | ১ |
| Num2 | ২ |
| Num3 | ৩ |
| Num4 | ৪ |
| Num5 | ৫ |
| Num6 | ৬ |
| Num7 | ৭ |
| Num8 | ৮ |
| Num9 | ৯ |
| Num+ | + |
| Num- | - |
| Num* | * |
| Num/ | / |
| Num. | . |

---

## Typing Mechanics

### Conjuncts (যুক্তাক্ষর)

Conjunct consonants are formed using the **hasanta** key (G) between two consonants:

- ক্ত: `J` (ক) → `G` (্) → `K` (ত)
- ন্ত: `B` (ন) → `G` (্) → `K` (ত)
- স্ত: `N` (স) → `G` (্) → `K` (ত)
- ক্ষ: `J` (ক) → `G` (্) → `N+Shift` (ষ)

### Vowel Signs (কার)

Vowel signs are typed **after** the consonant:

- কা: `J` (ক) → `H` (া)
- কি: `J` (ক) → `D` (ি)
- কী: `J` (ক) → `Shift+D` (ী)
- কু: `J` (ক) → `S` (ু)
- কূ: `J` (ক) → `Shift+S` (ূ)
- কে: `J` (ক) → `C` (ে)
- কৈ: `J` (ক) → `Shift+C` (ৈ)
- কো: `J` (ক) → `X` (ো)
- কৌ: `J` (ক) → `Shift+X` (ৌ)
- কৃ: `J` (ক) → `A` (ৃ)

### Full Vowels (স্বরবর্ণ)

Full vowel forms are accessed via AltGr + the corresponding kar key:

| Kar Key | Normal (kar) | AltGr (full vowel) | Shift+AltGr |
|---------|-------------|---------------------|-------------|
| H | া | আ | — |
| D | ি | ই | ঈ |
| S | ু | উ | ঊ |
| C | ে | এ | ঐ |
| X | ো | ও | ঔ |
| A | ৃ | ঋ | ৠ |

### ZWNJ / ZWJ (Joining Control)

The backtick (`` ` ``) key outputs **Zero Width Non-Joiner (ZWNJ U+200C)** and Shift+backtick outputs **Zero Width Joiner (ZWJ U+200D)**. These control ligature formation:

- **ZWNJ**: Prevents two characters from forming a conjunct (shows them with visible hasanta instead)
- **ZWJ**: Forces joining behavior where it would not normally occur

---

## Unicode Code Points Reference

### Consonants (ব্যঞ্জনবর্ণ)

| Character | Unicode | Name |
|-----------|---------|------|
| ক | U+0995 | KA |
| খ | U+0996 | KHA |
| গ | U+0997 | GA |
| ঘ | U+0998 | GHA |
| ঙ | U+0999 | NGA |
| চ | U+099A | CA |
| ছ | U+099B | CHA |
| জ | U+099C | JA |
| ঝ | U+099D | JHA |
| ঞ | U+099E | NYA |
| ট | U+099F | TTA |
| ঠ | U+09A0 | TTHA |
| ড | U+09A1 | DDA |
| ঢ | U+09A2 | DDHA |
| ণ | U+09A3 | NNA |
| ত | U+09A4 | TA |
| থ | U+09A5 | THA |
| দ | U+09A6 | DA |
| ধ | U+09A7 | DHA |
| ন | U+09A8 | NA |
| প | U+09AA | PA |
| ফ | U+09AB | PHA |
| ব | U+09AC | BA |
| ভ | U+09AD | BHA |
| ম | U+09AE | MA |
| য | U+09AF | YA |
| র | U+09B0 | RA |
| ল | U+09B2 | LA |
| শ | U+09B6 | SHA |
| ষ | U+09B7 | SSA |
| স | U+09B8 | SA |
| হ | U+09B9 | HA |
| ড় | U+09DC | RRA |
| ঢ় | U+09DD | RHA |
| য় | U+09DF | YYA |

### Vowels (স্বরবর্ণ)

| Character | Unicode | Name |
|-----------|---------|------|
| অ | U+0985 | A |
| আ | U+0986 | AA |
| ই | U+0987 | I |
| ঈ | U+0988 | II |
| উ | U+0989 | U |
| ঊ | U+098A | UU |
| ঋ | U+098B | VOCALIC R |
| এ | U+098F | E |
| ঐ | U+0990 | AI |
| ও | U+0993 | O |
| ঔ | U+0994 | AU |

### Vowel Signs (কার)

| Character | Unicode | Name |
|-----------|---------|------|
| া | U+09BE | AA SIGN |
| ি | U+09BF | I SIGN |
| ী | U+09C0 | II SIGN |
| ু | U+09C1 | U SIGN |
| ূ | U+09C2 | UU SIGN |
| ৃ | U+09C3 | VOCALIC R SIGN |
| ে | U+09C7 | E SIGN |
| ৈ | U+09C8 | AI SIGN |
| ো | U+09CB | O SIGN |
| ৌ | U+09CC | AU SIGN |
| ৗ | U+09D7 | AU LENGTH MARK |

### Special Characters

| Character | Unicode | Name |
|-----------|---------|------|
| ্ | U+09CD | HASANTA (virama) |
| ং | U+0982 | ANUSVARA |
| ঃ | U+0983 | VISARGA |
| ঁ | U+0981 | CHANDRABINDU |
| ৎ | U+09CE | KHANDA TA |
| ় | U+09BC | NUKTA |
| ঽ | U+09BD | AVAGRAHA |
| । | U+0964 | DANDA |
| ॥ | U+0965 | DOUBLE DANDA |

### Digits

| Character | Unicode |
|-----------|---------|
| ০ | U+09E6 |
| ১ | U+09E7 |
| ২ | U+09E8 |
| ৩ | U+09E9 |
| ৪ | U+09EA |
| ৫ | U+09EB |
| ৬ | U+09EC |
| ৭ | U+09ED |
| ৮ | U+09EE |
| ৯ | U+09EF |

### Currency & Rare Characters (AltGr layer)

| Character | Unicode | Name |
|-----------|---------|------|
| ৳ | U+09F3 | TAKA SIGN |
| ৲ | U+09F2 | RUPEE MARK |
| ৴ | U+09F4 | NUMERATOR ONE |
| ৵ | U+09F5 | NUMERATOR TWO |
| ৶ | U+09F6 | NUMERATOR THREE |
| ৷ | U+09F7 | CURRENCY NUMERATOR FOUR |
| ৸ | U+09F8 | CURRENCY NUMERATOR ONE LESS THAN DENOMINATOR |
| ৺ | U+09FA | ISSHAR |
| ৰ | U+09F0 | ASSAMESE RA |
| ৱ | U+09F1 | ASSAMESE WA |
| ঌ | U+098C | VOCALIC L |
| ৡ | U+09E1 | VOCALIC LL |
| ৢ | U+09E2 | VOCALIC L SIGN |
| ৣ | U+09E3 | VOCALIC LL SIGN |
| ৄ | U+09C4 | VOCALIC RR SIGN |
| ৠ | U+09E0 | VOCALIC RR |

---

## Layout Processing Rules

Based on the Avro Keyboard reference implementation, the National layout processor should support:

### Modern Mode (Default)

1. **Dead-key vowel logic**: After a hasanta (্) or at word start, typing a vowel kar key produces the full vowel form instead of the kar
2. **Automatic Reph**: র্ (ra + hasanta) before a consonant should be reordered as reph above the following conjunct
3. **Chandrabindu interaction**: Correct positioning of ঁ when combined with vowel signs
4. **Context tracking**: Maintain a buffer of recent characters for contextual processing

### Old/Traditional Mode (Optional)

1. **Pre-base kar typing**: Allows typing ে-কার, ি-কার, ৈ-কার before the consonant (old typewriter convention)
2. The processor then reorders them to correct Unicode order internally

### Capslock Behavior

- **Alpha keys** (A-Z): Capslock acts as a shift toggle (capslock-aware / "logical shift")
- **Number/OEM keys**: Capslock has no effect (uses "true shift" only)

### Backspace Behavior

Backspace must operate on **logical units**, not raw code points. The engine maintains an internal buffer of recently composed characters to enable intelligent deletion.

#### Rules

1. **Single character**: If the last input produced a single character (e.g., pressing `J` → ক), backspace deletes one character.

2. **Conjunct decomposition**: If the user formed a conjunct (e.g., ক + ্ + ত → ক্ত), backspace removes the **last logical input step**:
   - First backspace: removes ত and the hasanta → leaves ক
   - This means the engine sends enough `VK_BACK` keystrokes to delete the rendered conjunct glyph, then re-emits the remaining characters.

3. **Vowel kar deletion**: If a vowel sign was the last input (e.g., কা), backspace removes only the kar → leaves ক.

4. **Dead-key state**: If the engine is in dead-key state (hasanta pending), backspace cancels the dead-key state and removes the hasanta from the buffer without deleting the preceding consonant.

5. **Reph recomposition**: If reph (র্) was reordered above a conjunct, backspace should undo the reordering — remove the reph and restore the previous state.

6. **Multi-codepoint composed characters**: Some outputs span multiple code points (e.g., ো = ে + া as U+09CB, or composed via ে + া). Backspace must delete the entire logical unit.

#### Implementation Strategy (from Avro reference)

```
InternalBackspace(count):
    Remove `count` characters from the end of the internal tracking buffer
    Send `count` physical backspace keystrokes to delete from the application
    Re-emit any remaining composed text if needed

DoBackspace():
    If no tracked characters remain:
        Send enough backspaces to delete the entire last composed output
        Reset dead-key state
    Else:
        Remove last logical character from buffer
        Recompose and re-render the remaining buffer
```

The physical `Backspace()` function should send keystrokes with a small delay or synchronization mechanism between each to avoid race conditions with the host application's input handling.

---

## Implementation Notes

### JSON Layout Format

The layout data file should follow the same format as existing layouts in `app/data/layouts/`. The National layout requires an extended format to support the AltGr and Shift+AltGr layers:

```json
{
    "name": "National (Jatiya)",
    "version": "1.0",
    "type": "fixed",
    "standard": "BDS 1738:2018",
    "developer": "Bangladesh Computer Council (BCC)",
    "layers": 4,
    "key_mapping": {
        "normal": { ... },
        "shift": { ... },
        "altgr": { ... },
        "shift_altgr": { ... }
    },
    "numpad": { ... }
}
```

### Source Reference

The layout was extracted from the Avro Keyboard open-source project:
- File: `assets/keyboard-layouts/National (Jatiya).avrolayout` (XML format)
- Layout version: 5 (Avro Keyboard format)
- Processing: `clsGenericLayoutModern.pas` (modern mode), `clsGenericLayoutOld.pas` (traditional mode)
- Loader: `KeyboardLayoutLoader.pas`
