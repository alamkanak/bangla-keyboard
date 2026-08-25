# Bangla Keyboard Input Method Editor (IME) for Mac + Windows

This is a production grade project for a bangla typing input method for mac + windows. Bangladesh supports many keyboard layouts:

- Bijoy
- Phonetic
- Avro Easy Keyboard Layout
- Bornona Keyboard Layout
- Munir Optima Uni Keyboard Layout
- National (Jatiya) Keyboard Layout
- Probhat Keyboard Layout
- UniBijoy Keyboard Layout
- Windows Default Keyboard Layout
- Windows Keyboard Layout

Most widely accepted ones are unibijoy and phonetic.
The goal of this project is to create a mac/windows app that supports unibijoy and phonetic keyboard layouts for typing in Bangla. The app will allow users to switch between these layouts easily and provide a seamless typing experience across different platforms. This should be well integrated with operating system's input method framework to ensure compatibility and performance.

The IME application code lives in the `app/` folder. Other top-level folders (website, docs, etc.) may exist alongside it.

To learn more about this project and bangla typing methods, have a look at

- `docs` folder
- `README.md` file

UX Design principles you must follow are in `docs/DESIGN.md`

## Features

- Support for unibijoy and phonetic keyboard layouts for typing in Bangla.
- Seamless switching between keyboard layouts.
- Integration with operating system's input method framework for compatibility and performance.
- Support for multiple platforms (Mac and Windows).
- Onboarding flow experience after fresh install to:
  - enable system-wide keyboard input method.
  - teach users how to switch between keyboard layouts and use the app
  - switch between bangla and english for the app language i18n support
  - select theme

## General Instructions

- Use #context7 to get latest docs about any library or framework. You must now use #context7 to retrieve relevant information before you implement the task.
- You must use TDD development approach. All features must have unit tests and integration tests.
- For any network calls, it should be off the main thread and properly handle loading and error states in the UI/backend
- All list apis and web pages should support pagination if the list is long
- Use best practice principles:
  - DRY (Don't Repeat Yourself)
  - KISS (Keep It Simple, Stupid)
  - SOLID principles for object-oriented design
- Must use dependency injection and inversion of control principles for modularity and testability
- You don't need to worry about migrations or breaking changes since the app is not released anywhere yet. Its only in dev phase.
- The entire app should be themified and in the UI codes, it should use theme variables so that the theme of the entire app can be switched easily.
- The code should never use hardcoded values for fonts, colors, etc. and it should be used from theme design tokens.
- It should do all ui things as reusable components. So components has to be generic enough to support all states. Also there should one component for one thing. Not more than that. E.g. button is only one component that supports all variants and states.
- All strings has to have i18n support. So all strings should be in i18n files and not hardcoded in the code. And all translations must be provided.
