#pragma once

#include <windows.h>
#include <msctf.h>
#include <string>

// Engine CLSID - {A1B2C3D4-E5F6-7890-ABCD-EF1234567890}
// {generated unique GUID for this IME}
static const CLSID CLSID_BanglaKeyboard = {
    0xa1b2c3d4, 0xe5f6, 0x7890,
    {0xab, 0xcd, 0xef, 0x12, 0x34, 0x56, 0x78, 0x90}
};

// Profile GUID
static const GUID GUID_Profile = {
    0xb2c3d4e5, 0xf6a7, 0x8901,
    {0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x91}
};

// LANG_BENGALI / SUBLANG_BENGALI_BANGLADESH are macros in winnt.h — name our
// composed LANGID differently so the macro doesn't clobber the identifier.
static const LANGID BANGLA_LANG_ID = MAKELANGID(LANG_BENGALI, SUBLANG_BENGALI_BANGLADESH);

// Display name
static const wchar_t* DISPLAY_NAME = L"Bangla Keyboard";
static const wchar_t* TOOLTIP = L"Bangla Keyboard - UniBijoy & Phonetic Input Method";

// DLL globals
extern HINSTANCE g_hInstance;
extern LONG g_cRefDll;

void DllAddRef();
void DllRelease();
