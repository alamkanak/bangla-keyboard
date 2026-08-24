# Bangla Typing Methods: Bijoy/UniBijoy & Phonetic

## Overview

There are two fundamental approaches to typing Bangla on a computer:

1. **Fixed Layout (Bijoy / UniBijoy)** — Each physical key is mapped to a specific Bangla character. Users must memorize the layout.
2. **Phonetic / Transliteration (Avro Phonetic)** — Users type romanized English approximations and the software converts them to Bangla in real-time.

Both methods are widely used in Bangladesh. Bijoy dominates among professional typists and government offices; Phonetic dominates among casual users and the younger generation.

---

## 1. Bijoy Keyboard (Fixed Layout)

### History & Status

- Created by **Mustafa Jabbar**, first published for Macintosh in December 1988 and for Windows in March 1993 by **Ananda Computers**.
- **Proprietary** and licensed under the Bangladesh Copyright Act 2005.
- In 2017, the Bangladesh Computer Council declared the Bijoy layout as the **national keyboard layout** (BDS 1738:2018).
- Was the most widely used Bangla keyboard in Bangladesh until Avro Keyboard gained popularity.
- Requires purchasing a license for each computer.

### How It Works (UX)

Bijoy is a **fixed, positional keyboard layout**. Each key on a standard QWERTY keyboard is mapped to one or two Bangla characters (normal and shifted). The user presses physical keys to produce specific Bangla letters, vowel signs (কার/kar), and conjuncts.

#### Key Mapping (Bijoy Layout on QWERTY)

| Key | Normal       | Shift       |
| --- | ------------ | ----------- |
| Q   | ঙ            | ং           |
| W   | য            | য়          |
| E   | ড            | ঢ           |
| R   | প            | ফ           |
| T   | ট            | ঠ           |
| Y   | চ            | ছ           |
| U   | জ            | ঝ           |
| I   | হ            | ঞ           |
| O   | গ            | ঘ           |
| P   | ড়           | ঢ়          |
| A   | ‍ৃ (rri-kar) | ©           |
| S   | ‍ু (u-kar)   | ‍ূ (uu-kar) |
| D   | ি (i-kar)    | ‍ী (ii-kar) |
| F   | া (aa-kar)   | অ           |
| G   | ` (hasanta)  | ।           |
| H   | ব            | ভ           |
| J   | ক            | খ           |
| K   | ত            | থ           |
| L   | দ            | ধ           |
| Z   | ª            | ¨           |
| X   | ও            | ৗ (ou-kar)  |
| C   | ে (e-kar)    | ৈ (oi-kar)  |
| V   | র            | ল           |
| B   | ন            | ণ           |
| N   | স            | ষ           |
| M   | ম            | শ           |

**Number row:** Numbers produce Bangla digits (০-৯), with `Shift+4` producing ৳ (Taka sign), `Shift+7` producing ঁ (chandrabindu), and `\` producing ৎ (khanda-ta) / ঃ (visarga).

#### Typing Conjuncts (যুক্তাক্ষর)

Conjunct consonants are formed by pressing the **hasanta** key (G key) between two consonants:

- To type ক্ত (kto): press `J` (ক) → `G` (্ hasanta) → `K` (ত)
- To type ন্ত (nto): press `B` (ন) → `G` (্ hasanta) → `K` (ত)
- To type স্ত (sto): press `N` (স) → `G` (্ hasanta) → `K` (ত)

The hasanta (্) acts as a joiner that signals "combine the previous consonant with the next one."

#### Vowel Signs (কার)

Vowel signs are typed **after** the consonant they modify (matching the visual rendering order for most signs):

- To type কা (ka): press `J` (ক) → `F` (া aa-kar)
- To type কি (ki): press `J` (ক) → `D` (ি i-kar)
- To type কু (ku): press `J` (ক) → `S` (ু u-kar)

**Exception:** For ে-কার (e-kar) and ৈ-কার (oi-kar), which visually appear before the consonant, the user still types the consonant first, then the vowel sign. The rendering engine handles the visual placement.

#### UX Characteristics

| Aspect                 | Description                                                                     |
| ---------------------- | ------------------------------------------------------------------------------- |
| **Learning curve**     | Steep — requires memorizing the entire layout                                   |
| **Speed potential**    | Very high for trained typists (touch-typing possible)                           |
| **Target users**       | Professional typists, government offices, publishing houses                     |
| **Feedback model**     | Direct — each key press produces a character immediately                        |
| **Mode switching**     | Toggle between Bangla and English mode (typically Ctrl+Alt+B or a function key) |
| **Error handling**     | Incorrect key = wrong character; must delete and retype                         |
| **Conjunct discovery** | Users must know which consonant combinations form valid conjuncts               |

### Typing Automation (in Avro/OpenBangla implementations)

When Bijoy-style fixed layouts are used in modern software (Avro, OpenBangla), additional automation is available:

- **Automatic vowel forming** — converts standalone vowel letters appropriately
- **Old-style Reph (রেফ)** — automatic positioning of র্ above conjuncts
- **Traditional কার joining** — handles edge cases in vowel sign attachment
- **Automatic চন্দ্রবিন্দু positioning** — fixes chandrabindu placement

---

## 2. UniBijoy (Unicode Bijoy)

### What It Is

UniBijoy is a **Unicode-compliant version** of the Bijoy keyboard layout, standardized by the **Ekushey** project. It preserves the same key positions as Bijoy but outputs proper **Unicode characters** instead of ANSI codes.

### Bijoy vs UniBijoy

| Feature                 | Bijoy (Original)                                | UniBijoy                                       |
| ----------------------- | ----------------------------------------------- | ---------------------------------------------- |
| **Encoding**            | ASCII/ANSI (custom code points)                 | Unicode (U+0980–U+09FF)                        |
| **Font dependency**     | Requires Bijoy-specific fonts (e.g., SutonnyMJ) | Works with any Unicode Bangla font             |
| **Cross-platform text** | Text appears garbled without correct font       | Text renders correctly everywhere              |
| **Copy/paste**          | Broken across different systems                 | Works universally                              |
| **Search/sort**         | Unreliable                                      | Standards-compliant                            |
| **Web compatibility**   | Poor                                            | Full                                           |
| **License**             | Proprietary (Mustafa Jabbar)                    | Was distributed freely until copyright dispute |

### Legal History

UniBijoy was included in Avro Keyboard v4.5.1 but was removed in v4.5.2 (August 2010) after Mustafa Jabbar filed a copyright violation claim with the Bangladesh Copyright Office, arguing the layout was a copy of his Bijoy layout. A settlement was reached and UniBijoy was dropped from Avro.

---

## 3. Phonetic Typing (Avro Phonetic)

### History & Status

- Developed by **Mehdi Hasan Khan** (OmicronLab), first released **26 March 2003** (Bangladesh Independence Day).
- **Free and open-source** under Mozilla Public License 1.1.
- Available on Windows (Avro), Linux (ibus-avro, OpenBangla Keyboard), macOS (iAvro), and web (avro.im).
- The phonetic layout has been adopted by Ridmik Keyboard (Android/iOS), Borno, OpenBangla Keyboard, Bengali Wikipedia, and Firefox OS.
- Received the **Ekushey Padak** (second-highest civilian award in Bangladesh) in 2025.

### How It Works (UX)

The user types **romanized English text** on a standard QWERTY keyboard, and the software **transliterates** it to Bangla in real-time. No layout memorization is needed — if you can spell the Bangla word phonetically in English, you can type it.

#### Core Transliteration Rules

**Vowels:**

| Type | Bangla | Romanized   |
| ---- | ------ | ----------- |
| অ    | অ      | `o`         |
| আ    | আ      | `a`         |
| ই    | ই      | `i`         |
| ঈ    | ঈ      | `ee` / `ii` |
| উ    | উ      | `u`         |
| ঊ    | ঊ      | `oo` / `uu` |
| এ    | এ      | `e`         |
| ঐ    | ঐ      | `OI` / `oi` |
| ও    | ও      | `O`         |
| ঔ    | ঔ      | `OU` / `ou` |

**Common Consonants:**

| Type | Bangla | Romanized     |
| ---- | ------ | ------------- |
| ক    | ক      | `k`           |
| খ    | খ      | `kh`          |
| গ    | গ      | `g`           |
| ঘ    | ঘ      | `gh`          |
| চ    | চ      | `c` / `ch`    |
| ছ    | ছ      | `chh`         |
| জ    | জ      | `j`           |
| ঝ    | ঝ      | `jh`          |
| ট    | ট      | `T`           |
| ঠ    | ঠ      | `Th`          |
| ড    | ড      | `D`           |
| ঢ    | ঢ      | `Dh`          |
| ণ    | ণ      | `N` (capital) |
| ত    | ত      | `t`           |
| থ    | থ      | `th`          |
| দ    | দ      | `d`           |
| ধ    | ধ      | `dh`          |
| ন    | ন      | `n`           |
| প    | প      | `p`           |
| ফ    | ফ      | `ph` / `f`    |
| ব    | ব      | `b`           |
| ভ    | ভ      | `bh` / `v`    |
| ম    | ম      | `m`           |
| র    | র      | `r`           |
| ল    | ল      | `l`           |
| শ    | শ      | `sh` / `S`    |
| ষ    | ষ      | `Sh`          |
| স    | স      | `s`           |
| হ    | হ      | `h`           |
| ং    | ং      | `ng`          |
| ঃ    | ঃ      | `H` (capital) |
| ঁ    | ঁ      | `^`           |

**Example:**

Typing `ami banglay gan gai` produces: **আমি বাংলায় গান গাই**

#### Floating Preview Window

A small preview window appears near the cursor showing the Bangla text being composed as the user types romanized text. The user can see the conversion happening in real-time before committing.

#### Dictionary & Auto-Correct

- Built-in dictionary with ~150,000 Bangla words.
- As the user types, the system suggests phonetically similar words with correct spelling.
- Auto-correct handles commonly mistyped words.
- Common English words (Facebook, download, etc.) are auto-corrected to their Bangla equivalents.
- Users can add custom words and edit the auto-correct dictionary.

#### UX Characteristics

| Aspect              | Description                                                            |
| ------------------- | ---------------------------------------------------------------------- |
| **Learning curve**  | Very low — if you can type English, you can type Bangla                |
| **Speed potential** | Moderate for casual use; can be fast with practice                     |
| **Target users**    | General population, students, casual writers, anyone who knows English |
| **Feedback model**  | Delayed/preview — text is composed, then committed                     |
| **Mode switching**  | Toggle between Bangla and English (Ctrl+Space, or F1-F12 configurable) |
| **Error handling**  | May produce wrong word from ambiguous romanization; dictionary helps   |
| **Discoverability** | High — intuitive guessing works for most words                         |

---

## 4. Fonts

### ANSI Fonts (Legacy — Bijoy Era)

The original Bijoy keyboard used **ANSI encoding** where Bangla glyphs were mapped to ASCII code points. This required specific fonts to display correctly.

| Font            | Notes                                                                                  |
| --------------- | -------------------------------------------------------------------------------------- |
| **SutonnyMJ**   | The most popular ANSI Bangla font, widely used in print media and government documents |
| **SutonnyOMJ**  | OpenType version of SutonnyMJ                                                          |
| **BijoyMJ**     | Bundled with Bijoy software                                                            |
| **AdorshoLipi** | Common ANSI font                                                                       |
| **BanglaMJ**    | Another popular ANSI choice                                                            |

**Problem with ANSI fonts:** Text typed in ANSI encoding looks like garbled English characters when the correct font is not installed. Text cannot be searched, sorted, or used on the web without conversion.

### Unicode Fonts (Modern — Standard)

Unicode Bangla fonts use the standard Bengali Unicode block (U+0980–U+09FF) and work universally across all platforms and applications.

| Font                   | Type                 | Notes                                                                                        |
| ---------------------- | -------------------- | -------------------------------------------------------------------------------------------- |
| **Noto Sans Bengali**  | Sans-serif, variable | Google's font — multiple weights (100-900) and widths, 695 glyphs. Excellent coverage. Free. |
| **Noto Serif Bengali** | Serif, variable      | Serif counterpart from Google. Free.                                                         |
| **SolaimanLipi**       | Sans-serif           | Most widely used Unicode Bangla font in Bangladesh. Free.                                    |
| **Kalpurush**          | Sans-serif           | Popular free Unicode font from Ekushey                                                       |
| **Nikosh**             | Serif                | Free Unicode font, common in government documents                                            |
| **Bangla MN**          | Sans-serif           | Ships with macOS                                                                             |
| **Vrinda**             | Sans-serif           | Ships with Windows                                                                           |
| **Shonar Bangla**      | Serif                | Ships with Windows                                                                           |
| **Akaash**             | Sans-serif           | Free, Ekushey project                                                                        |
| **Mukti**              | Sans-serif           | Free, Ekushey project                                                                        |
| **Lohit Bengali**      | Sans-serif           | Ships with many Linux distributions                                                          |

### OpenType Font Benefits

- Type Bangla and English in the **same font** — just change keyboard mode
- Proper **conjunct rendering** via OpenType GSUB/GPOS tables
- Correct **vowel sign reordering** handled by the text shaping engine (HarfBuzz, CoreText, DirectWrite)
- Full **search and sort** support
- Works universally on **web, email, documents, databases**

### Font Rendering Pipeline

For Bangla text to display correctly, the system needs:

1. **Unicode-encoded text** — proper code points in the Bengali block
2. **A Unicode Bangla font** with OpenType tables for complex script shaping
3. **A text shaping engine** (HarfBuzz, CoreText, DirectWrite, Uniscribe) that applies:
   - Conjunct formation (via GSUB lookup tables)
   - Vowel sign reordering (e.g., ি appears left of the consonant)
   - Reph and Ya-phala positioning
   - Mark positioning (chandrabindu, nukta)

---

## 5. Comparison: Bijoy vs Phonetic

| Feature                      | Bijoy (Fixed)                       | Phonetic (Avro)                   |
| ---------------------------- | ----------------------------------- | --------------------------------- |
| **Input method**             | Direct key-to-character mapping     | English-to-Bangla transliteration |
| **Layout memorization**      | Required                            | Not needed                        |
| **Typing speed (trained)**   | Faster                              | Slightly slower                   |
| **Typing speed (untrained)** | Very slow                           | Immediate productivity            |
| **Conjunct typing**          | Manual (hasanta between consonants) | Automatic from romanization       |
| **Error types**              | Wrong character                     | Wrong word/spelling               |
| **Professional use**         | Standard in publishing, govt        | Growing adoption                  |
| **Encoding**                 | Originally ANSI, now also Unicode   | Unicode from the start            |
| **License**                  | Proprietary (layout)                | Open source                       |
| **Platform support**         | Windows primarily                   | Windows, macOS, Linux, Web        |
| **Offline capability**       | Full                                | Full                              |
| **Dictionary needed**        | No                                  | Yes (for best results)            |

---

## 6. Implementation Considerations for Our App

### Must Support

- **Bijoy/UniBijoy layout** — fixed key mapping for professional users
- **Avro-compatible phonetic** — transliteration with dictionary for casual users
- **Unicode output** — all text output must be Unicode (Bengali block U+0980–U+09FF)
- **Mode switching** — easy toggle between Bangla and system keyboard (configurable hotkey)
- **Layout viewer** — on-screen keyboard showing current layout (critical for Bijoy learners)

### OS Integration Points

- **macOS:** Input Method Kit (IMK) framework — register as a system input source
- **Windows:** Text Services Framework (TSF) — register as a text input processor

### Font Considerations

- Ship with or recommend **SolaimanLipi** and/or **Noto Sans Bengali** as default fonts
- Ensure the app works with any Unicode Bangla font installed on the system
- No ANSI font support needed (legacy concern only)

### Phonetic Engine Requirements

- Transliteration rule engine (pattern matching from Roman to Bangla)
- Dictionary for word prediction and disambiguation
- Auto-correct database (user-editable)
- Real-time preview window near cursor
- Support for case-sensitive mappings (e.g., `T` vs `t`, `N` vs `n`, `D` vs `d`)
