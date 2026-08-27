const translations = {
  en: {
    // Sidebar
    'nav.layout': 'Layout',
    'nav.hotkeys': 'Hotkeys',
    'nav.theme': 'Theme',
    'nav.dictionary': 'Dictionary',
    'nav.about': 'About',
    'sidebar.title': 'Settings',

    // Layout page
    'layout.title': 'Layout',
    'layout.desc': 'Choose your preferred Bangla typing layout.',
    'layout.active': 'Active Layout',
    'layout.active.hint': 'Switch between Phonetic and UniBijoy input methods',
    'layout.phonetic': 'Phonetic (Avro)',
    'layout.unibijoy': 'UniBijoy',
    'layout.showKeyboard': 'Show on-screen keyboard',
    'layout.showShift': 'Show Shift layer',

    // Hotkeys page
    'hotkeys.title': 'Hotkeys',
    'hotkeys.desc': 'System keyboard shortcuts for switching input methods.',
    'hotkeys.switch.title': 'Switch between Bangla and English',
    'hotkeys.switch.desc': 'Use these system shortcuts to toggle between Bangla Keyboard and your default input method.',
    'hotkeys.mac.globe': 'Switch input source (recommended)',
    'hotkeys.mac.ctrlspace': 'Switch input source (if enabled)',
    'hotkeys.win.winspace': 'Cycle through input methods',
    'hotkeys.win.altshift': 'Switch input language',
    'hotkeys.customize.title': 'Customize shortcuts',
    'hotkeys.customize.mac': 'You can change these in System Settings → Keyboard → Keyboard Shortcuts → Input Sources.',
    'hotkeys.customize.win': 'You can change these in Settings → Time & Language → Typing → Advanced keyboard settings → Input language hot keys.',

    // Theme page
    'theme.title': 'Theme',
    'theme.desc': 'Choose your preferred appearance.',
    'theme.dark': 'Dark',
    'theme.dark.desc': 'Easy on the eyes, great for night use.',
    'theme.light': 'Light',
    'theme.light.desc': 'Clean and bright, great for daytime use.',

    // Dictionary page
    'dictionary.title': 'Dictionary',
    'dictionary.desc': 'Manage your custom words and autocorrect entries.',
    'dictionary.empty': 'Custom dictionary management coming soon.',

    // About page
    'about.title': 'About',
    'about.name': 'Bangla Keyboard',
    'about.desc': 'A macOS + Windows input method supporting UniBijoy and Phonetic keyboard layouts for typing in Bangla.',

    // Onboarding
    'onboarding.title': 'Welcome to Bangla Keyboard',
    'onboarding.subtitle': 'Let\u2019s get you set up in a few quick steps.',
    'onboarding.next': 'Continue',
    'onboarding.back': 'Back',
    'onboarding.finish': 'Get Started',
    'onboarding.step.language': 'Language',
    'onboarding.step.switching': 'Switching',
    'onboarding.step.layout': 'Layout',
    'onboarding.step.try': 'Try It',
    'onboarding.step.theme': 'Theme',

    'onboarding.language.title': 'Choose your language',
    'onboarding.language.desc': 'Select which language you\u2019d like for the app interface. You can change this later in settings.',

    'onboarding.switching.title': 'Switching between Bangla and English',
    'onboarding.switching.desc': 'Use these keyboard shortcuts to switch between Bangla Keyboard and your default input method.',
    'onboarding.switching.hotkeysTitle': 'Keyboard shortcuts',

    'onboarding.layout.title': 'Pick a layout',
    'onboarding.layout.desc': 'Choose how you want to type in Bangla. You can switch anytime.',
    'onboarding.layout.phonetic': 'Phonetic (Avro)',
    'onboarding.layout.phonetic.desc': 'Type romanized English and get Bangla automatically. Best for casual users.',
    'onboarding.layout.unibijoy': 'UniBijoy',
    'onboarding.layout.unibijoy.desc': 'Fixed key positions like the classic Bijoy layout. Best for professional typists.',
    'onboarding.layout.national': 'National (Jatiya)',
    'onboarding.layout.national.desc': 'Official Bangladesh standard (BDS 1738). Fixed layout with AltGr layer for rare characters.',

    'onboarding.try.title': 'Try it out',
    'onboarding.try.desc': 'Switch to Bangla Keyboard using the shortcut you just learned, then type something below.',
    'onboarding.try.hint.mac': 'Press the Globe key (🌐) or Ctrl+Space to switch to Bangla, then type in the box below.',
    'onboarding.try.hint.win': 'Press Win+Space to switch to Bangla, then type in the box below.',
    'onboarding.try.placeholder': 'Type something in Bangla here\u2026',
    'onboarding.try.success': 'You\u2019re typing in Bangla! You\u2019re all set.',

    'onboarding.theme.title': 'Select a theme',
    'onboarding.theme.desc': 'Choose your preferred appearance. You can change this later.',
    'onboarding.theme.dark': 'Dark',
    'onboarding.theme.dark.desc': 'Easy on the eyes, great for night use.',
    'onboarding.theme.light': 'Light',
    'onboarding.theme.light.desc': 'Clean and bright, great for daytime use.',
  },
  bn: {
    // Sidebar
    'nav.layout': 'লেআউট',
    'nav.hotkeys': 'হটকি',
    'nav.theme': 'থিম',
    'nav.dictionary': 'অভিধান',
    'nav.about': 'সম্পর্কে',
    'sidebar.title': 'সেটিংস',

    // Layout page
    'layout.title': 'লেআউট',
    'layout.desc': 'আপনার পছন্দের বাংলা টাইপিং লেআউট বেছে নিন।',
    'layout.active': 'সক্রিয় লেআউট',
    'layout.active.hint': 'ফোনেটিক এবং ইউনিবিজয় ইনপুট পদ্ধতির মধ্যে পরিবর্তন করুন',
    'layout.phonetic': 'ফোনেটিক (অভ্র)',
    'layout.unibijoy': 'ইউনিবিজয়',
    'layout.showKeyboard': 'অন-স্ক্রীন কীবোর্ড দেখান',
    'layout.showShift': 'শিফট স্তর দেখান',

    // Hotkeys page
    'hotkeys.title': 'হটকি',
    'hotkeys.desc': 'ইনপুট পদ্ধতি পরিবর্তনের জন্য সিস্টেম কীবোর্ড শর্টকাট।',
    'hotkeys.switch.title': 'বাংলা এবং ইংরেজির মধ্যে পরিবর্তন',
    'hotkeys.switch.desc': 'বাংলা কীবোর্ড এবং আপনার ডিফল্ট ইনপুট পদ্ধতির মধ্যে টগল করতে এই সিস্টেম শর্টকাটগুলি ব্যবহার করুন।',
    'hotkeys.mac.globe': 'ইনপুট সোর্স পরিবর্তন (প্রস্তাবিত)',
    'hotkeys.mac.ctrlspace': 'ইনপুট সোর্স পরিবর্তন (সক্রিয় থাকলে)',
    'hotkeys.win.winspace': 'ইনপুট পদ্ধতি চক্রাকারে পরিবর্তন',
    'hotkeys.win.altshift': 'ইনপুট ভাষা পরিবর্তন',
    'hotkeys.customize.title': 'শর্টকাট কাস্টমাইজ করুন',
    'hotkeys.customize.mac': 'আপনি System Settings → Keyboard → Keyboard Shortcuts → Input Sources এ এগুলি পরিবর্তন করতে পারেন।',
    'hotkeys.customize.win': 'আপনি Settings → Time & Language → Typing → Advanced keyboard settings → Input language hot keys এ এগুলি পরিবর্তন করতে পারেন।',

    // Theme page
    'theme.title': 'থিম',
    'theme.desc': 'আপনার পছন্দের চেহারা বেছে নিন।',
    'theme.dark': 'ডার্ক',
    'theme.dark.desc': 'চোখের জন্য আরামদায়ক, রাতে ব্যবহারের জন্য দারুণ।',
    'theme.light': 'লাইট',
    'theme.light.desc': 'পরিষ্কার এবং উজ্জ্বল, দিনের বেলা ব্যবহারের জন্য দারুণ।',

    // Dictionary page
    'dictionary.title': 'অভিধান',
    'dictionary.desc': 'আপনার কাস্টম শব্দ এবং স্বয়ংসংশোধন এন্ট্রি পরিচালনা করুন।',
    'dictionary.empty': 'কাস্টম অভিধান ব্যবস্থাপনা শীঘ্রই আসছে।',

    // About page
    'about.title': 'সম্পর্কে',
    'about.name': 'বাংলা কীবোর্ড',
    'about.desc': 'একটি macOS + Windows ইনপুট পদ্ধতি যা বাংলায় টাইপ করার জন্য ইউনিবিজয় এবং ফোনেটিক কীবোর্ড লেআউট সমর্থন করে।',

    // Onboarding
    'onboarding.title': 'বাংলা কীবোর্ডে স্বাগতম',
    'onboarding.subtitle': 'কয়েকটি সহজ ধাপে আপনাকে সেটআপ করা যাক।',
    'onboarding.next': 'পরবর্তী',
    'onboarding.back': 'পেছনে',
    'onboarding.finish': 'শুরু করুন',
    'onboarding.step.language': 'ভাষা',
    'onboarding.step.switching': 'পরিবর্তন',
    'onboarding.step.layout': 'লেআউট',
    'onboarding.step.try': 'পরীক্ষা',
    'onboarding.step.theme': 'থিম',

    'onboarding.language.title': 'আপনার ভাষা বেছে নিন',
    'onboarding.language.desc': 'অ্যাপ ইন্টারফেসের জন্য কোন ভাষা ব্যবহার করতে চান তা নির্বাচন করুন। আপনি পরে সেটিংসে এটি পরিবর্তন করতে পারবেন।',

    'onboarding.switching.title': 'বাংলা এবং ইংরেজির মধ্যে পরিবর্তন',
    'onboarding.switching.desc': 'বাংলা কীবোর্ড এবং আপনার ডিফল্ট ইনপুট পদ্ধতির মধ্যে পরিবর্তন করতে এই কীবোর্ড শর্টকাটগুলি ব্যবহার করুন।',
    'onboarding.switching.hotkeysTitle': 'কীবোর্ড শর্টকাট',

    'onboarding.layout.title': 'একটি লেআউট বেছে নিন',
    'onboarding.layout.desc': 'আপনি কীভাবে বাংলায় টাইপ করতে চান তা বেছে নিন। আপনি যেকোনো সময় পরিবর্তন করতে পারেন।',
    'onboarding.layout.phonetic': 'ফোনেটিক (অভ্র)',
    'onboarding.layout.phonetic.desc': 'রোমানাইজড ইংরেজি টাইপ করুন এবং স্বয়ংক্রিয়ভাবে বাংলা পান। সাধারণ ব্যবহারকারীদের জন্য সেরা।',
    'onboarding.layout.unibijoy': 'ইউনিবিজয়',
    'onboarding.layout.unibijoy.desc': 'ক্লাসিক বিজয় লেআউটের মতো নির্দিষ্ট কী পজিশন। পেশাদার টাইপিস্টদের জন্য সেরা।',
    'onboarding.layout.national': 'জাতীয় (ন্যাশনাল)',
    'onboarding.layout.national.desc': 'বাংলাদেশের সরকারি মান (BDS 1738)। AltGr লেয়ার সহ নির্দিষ্ট কী লেআউট।',

    'onboarding.try.title': 'পরীক্ষা করুন',
    'onboarding.try.desc': 'আপনি যে শর্টকাট শিখলেন তা ব্যবহার করে বাংলা কীবোর্ডে পরিবর্তন করুন, তারপর নিচে কিছু টাইপ করুন।',
    'onboarding.try.hint.mac': 'বাংলায় পরিবর্তন করতে Globe কী (🌐) বা Ctrl+Space চাপুন, তারপর নিচের বক্সে টাইপ করুন।',
    'onboarding.try.hint.win': 'বাংলায় পরিবর্তন করতে Win+Space চাপুন, তারপর নিচের বক্সে টাইপ করুন।',
    'onboarding.try.placeholder': 'এখানে বাংলায় কিছু টাইপ করুন\u2026',
    'onboarding.try.success': 'আপনি বাংলায় টাইপ করছেন! আপনি প্রস্তুত।',

    'onboarding.theme.title': 'একটি থিম নির্বাচন করুন',
    'onboarding.theme.desc': 'আপনার পছন্দের চেহারা বেছে নিন। আপনি পরে এটি পরিবর্তন করতে পারেন।',
    'onboarding.theme.dark': 'ডার্ক',
    'onboarding.theme.dark.desc': 'চোখের জন্য আরামদায়ক, রাতে ব্যবহারের জন্য দারুণ।',
    'onboarding.theme.light': 'লাইট',
    'onboarding.theme.light.desc': 'পরিষ্কার এবং উজ্জ্বল, দিনের বেলা ব্যবহারের জন্য দারুণ।',
  },
};

let currentLocale = 'en';

export function setLocale(locale) {
  if (translations[locale]) {
    currentLocale = locale;
  }
}

export function getLocale() {
  return currentLocale;
}

export function t(key) {
  return translations[currentLocale]?.[key] ?? translations['en']?.[key] ?? key;
}

export function getAvailableLocales() {
  return Object.keys(translations);
}
