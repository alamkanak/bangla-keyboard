const translations = {
  en: {
    // Sidebar
    'nav.layout': 'Layout',
    'nav.hotkeys': 'Hotkeys',
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
    'hotkeys.desc': 'Configure keyboard shortcuts.',
    'hotkeys.toggle': 'Toggle Bangla/English',
    'hotkeys.toggle.hint': 'Shortcut to switch between Bangla and system keyboard',

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
    'onboarding.step.enable': 'Enable',
    'onboarding.step.layout': 'Layout',
    'onboarding.step.theme': 'Theme',

    'onboarding.language.title': 'Choose your language',
    'onboarding.language.desc': 'Select which language you\u2019d like for the app interface. You can change this later in settings.',

    'onboarding.enable.title': 'Enable the input method',
    'onboarding.enable.desc': 'Bangla Keyboard needs to be registered as a system input source. Click the button below to enable it, then verify it appears in your input sources.',
    'onboarding.enable.button': 'Enable Bangla Keyboard',
    'onboarding.enable.enabled': 'Enabled!',
    'onboarding.enable.manual': 'To verify or add manually:',
    'onboarding.enable.mac.step1': 'Open System Settings \u2192 Keyboard',
    'onboarding.enable.mac.step2': 'Click "Edit" next to Input Sources',
    'onboarding.enable.mac.step3': 'Click "+", select "Bangla" in the left list',
    'onboarding.enable.mac.step4': 'Look for "Bangla Keyboard" on the right and click "Add"',
    'onboarding.enable.win.step1': 'Open Settings \u2192 Time & Language \u2192 Language & Region',
    'onboarding.enable.win.step2': 'Under "Preferred languages", click the "\u22EF" next to Bengali',
    'onboarding.enable.win.step3': 'Click "Language options", then "Add a keyboard"',
    'onboarding.enable.win.step4': 'Select "Bangla Keyboard" from the list',
    'onboarding.enable.switchHint': 'Once enabled, press Ctrl+Space (or your configured hotkey) to switch between Bangla and English while typing in any app.',

    'onboarding.layout.title': 'Pick a layout',
    'onboarding.layout.desc': 'Choose how you want to type in Bangla. You can switch anytime.',
    'onboarding.layout.phonetic': 'Phonetic (Avro)',
    'onboarding.layout.phonetic.desc': 'Type romanized English and get Bangla automatically. Best for casual users.',
    'onboarding.layout.unibijoy': 'UniBijoy',
    'onboarding.layout.unibijoy.desc': 'Fixed key positions like the classic Bijoy layout. Best for professional typists.',
    'onboarding.layout.national': 'National (Jatiya)',
    'onboarding.layout.national.desc': 'Official Bangladesh standard (BDS 1738). Fixed layout with AltGr layer for rare characters.',

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
    'hotkeys.desc': 'কীবোর্ড শর্টকাট কনফিগার করুন।',
    'hotkeys.toggle': 'বাংলা/ইংরেজি টগল',
    'hotkeys.toggle.hint': 'বাংলা এবং সিস্টেম কীবোর্ডের মধ্যে পরিবর্তন করার শর্টকাট',

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
    'onboarding.step.enable': 'সক্রিয়',
    'onboarding.step.layout': 'লেআউট',
    'onboarding.step.theme': 'থিম',

    'onboarding.language.title': 'আপনার ভাষা বেছে নিন',
    'onboarding.language.desc': 'অ্যাপ ইন্টারফেসের জন্য কোন ভাষা ব্যবহার করতে চান তা নির্বাচন করুন। আপনি পরে সেটিংসে এটি পরিবর্তন করতে পারবেন।',

    'onboarding.enable.title': 'ইনপুট পদ্ধতি সক্রিয় করুন',
    'onboarding.enable.desc': 'বাংলা কীবোর্ডকে সিস্টেম ইনপুট সোর্স হিসেবে নিবন্ধন করতে হবে। নিচের বোতামে ক্লিক করুন, তারপর যাচাই করুন।',
    'onboarding.enable.button': 'বাংলা কীবোর্ড সক্রিয় করুন',
    'onboarding.enable.enabled': 'সক্রিয় হয়েছে!',
    'onboarding.enable.manual': 'যাচাই করতে বা ম্যানুয়ালি যোগ করতে:',
    'onboarding.enable.mac.step1': 'System Settings → Keyboard খুলুন',
    'onboarding.enable.mac.step2': 'Input Sources এর পাশে "Edit" ক্লিক করুন',
    'onboarding.enable.mac.step3': '"+" ক্লিক করুন, বাম তালিকায় "Bangla" নির্বাচন করুন',
    'onboarding.enable.mac.step4': 'ডানদিকে "Bangla Keyboard" খুঁজুন এবং "Add" ক্লিক করুন',
    'onboarding.enable.win.step1': 'Settings → Time & Language → Language & Region খুলুন',
    'onboarding.enable.win.step2': '"Preferred languages" এর অধীনে Bengali এর পাশে "⋯" ক্লিক করুন',
    'onboarding.enable.win.step3': '"Language options" ক্লিক করুন, তারপর "Add a keyboard"',
    'onboarding.enable.win.step4': 'তালিকা থেকে "Bangla Keyboard" নির্বাচন করুন',
    'onboarding.enable.switchHint': 'সক্রিয় করার পর, যেকোনো অ্যাপে টাইপ করার সময় বাংলা এবং ইংরেজির মধ্যে পরিবর্তন করতে Ctrl+Space (বা আপনার কনফিগার করা হটকি) চাপুন।',

    'onboarding.layout.title': 'একটি লেআউট বেছে নিন',
    'onboarding.layout.desc': 'আপনি কীভাবে বাংলায় টাইপ করতে চান তা বেছে নিন। আপনি যেকোনো সময় পরিবর্তন করতে পারেন।',
    'onboarding.layout.phonetic': 'ফোনেটিক (অভ্র)',
    'onboarding.layout.phonetic.desc': 'রোমানাইজড ইংরেজি টাইপ করুন এবং স্বয়ংক্রিয়ভাবে বাংলা পান। সাধারণ ব্যবহারকারীদের জন্য সেরা।',
    'onboarding.layout.unibijoy': 'ইউনিবিজয়',
    'onboarding.layout.unibijoy.desc': 'ক্লাসিক বিজয় লেআউটের মতো নির্দিষ্ট কী পজিশন। পেশাদার টাইপিস্টদের জন্য সেরা।',
    'onboarding.layout.national': 'জাতীয় (ন্যাশনাল)',
    'onboarding.layout.national.desc': 'বাংলাদেশের সরকারি মান (BDS 1738)। AltGr লেয়ার সহ নির্দিষ্ট কী লেআউট।',

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
