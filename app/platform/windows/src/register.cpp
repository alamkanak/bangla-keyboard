#include "register.h"
#include "globals.h"
#include <msctf.h>
#include <string>

static const wchar_t* CLSID_KEY = L"CLSID\\{A1B2C3D4-E5F6-7890-ABCD-EF1234567890}";
static const wchar_t* INPROC_KEY = L"CLSID\\{A1B2C3D4-E5F6-7890-ABCD-EF1234567890}\\InProcServer32";

BOOL RegisterServer() {
    wchar_t dllPath[MAX_PATH];
    if (!GetModuleFileNameW(g_hInstance, dllPath, MAX_PATH)) return FALSE;

    HKEY hKey;
    DWORD dwDisposition;

    // Register CLSID
    if (RegCreateKeyExW(HKEY_CLASSES_ROOT, CLSID_KEY, 0, nullptr,
                        REG_OPTION_NON_VOLATILE, KEY_WRITE, nullptr,
                        &hKey, &dwDisposition) != ERROR_SUCCESS) {
        return FALSE;
    }
    RegSetValueExW(hKey, nullptr, 0, REG_SZ,
                   (const BYTE*)DISPLAY_NAME, (DWORD)((wcslen(DISPLAY_NAME) + 1) * sizeof(wchar_t)));
    RegCloseKey(hKey);

    // Register InProcServer32
    if (RegCreateKeyExW(HKEY_CLASSES_ROOT, INPROC_KEY, 0, nullptr,
                        REG_OPTION_NON_VOLATILE, KEY_WRITE, nullptr,
                        &hKey, &dwDisposition) != ERROR_SUCCESS) {
        return FALSE;
    }
    RegSetValueExW(hKey, nullptr, 0, REG_SZ,
                   (const BYTE*)dllPath, (DWORD)((wcslen(dllPath) + 1) * sizeof(wchar_t)));
    const wchar_t* threadingModel = L"Apartment";
    RegSetValueExW(hKey, L"ThreadingModel", 0, REG_SZ,
                   (const BYTE*)threadingModel, (DWORD)((wcslen(threadingModel) + 1) * sizeof(wchar_t)));
    RegCloseKey(hKey);

    return TRUE;
}

void UnregisterServer() {
    RegDeleteTreeW(HKEY_CLASSES_ROOT, CLSID_KEY);
}

BOOL RegisterProfiles() {
    ITfInputProcessorProfiles* pInputProcessProfiles = nullptr;
    HRESULT hr = CoCreateInstance(CLSID_TF_InputProcessorProfiles, nullptr,
                                  CLSCTX_INPROC_SERVER, IID_ITfInputProcessorProfiles,
                                  (void**)&pInputProcessProfiles);
    if (FAILED(hr)) return FALSE;

    hr = pInputProcessProfiles->Register(CLSID_BanglaKeyboard);
    if (FAILED(hr)) {
        pInputProcessProfiles->Release();
        return FALSE;
    }

    wchar_t dllPath[MAX_PATH];
    GetModuleFileNameW(g_hInstance, dllPath, MAX_PATH);

    hr = pInputProcessProfiles->AddLanguageProfile(
        CLSID_BanglaKeyboard,
        LANG_BENGALI,
        GUID_Profile,
        DISPLAY_NAME, (ULONG)wcslen(DISPLAY_NAME),
        dllPath, (ULONG)wcslen(dllPath),
        0 // icon index
    );

    pInputProcessProfiles->Release();
    return SUCCEEDED(hr);
}

void UnregisterProfiles() {
    ITfInputProcessorProfiles* pInputProcessProfiles = nullptr;
    HRESULT hr = CoCreateInstance(CLSID_TF_InputProcessorProfiles, nullptr,
                                  CLSCTX_INPROC_SERVER, IID_ITfInputProcessorProfiles,
                                  (void**)&pInputProcessProfiles);
    if (SUCCEEDED(hr)) {
        pInputProcessProfiles->Unregister(CLSID_BanglaKeyboard);
        pInputProcessProfiles->Release();
    }
}

BOOL RegisterCategories() {
    ITfCategoryMgr* pCategoryMgr = nullptr;
    HRESULT hr = CoCreateInstance(CLSID_TF_CategoryMgr, nullptr,
                                  CLSCTX_INPROC_SERVER, IID_ITfCategoryMgr,
                                  (void**)&pCategoryMgr);
    if (FAILED(hr)) return FALSE;

    hr = pCategoryMgr->RegisterCategory(CLSID_BanglaKeyboard,
                                         GUID_TFCAT_TIP_KEYBOARD,
                                         CLSID_BanglaKeyboard);

    pCategoryMgr->Release();
    return SUCCEEDED(hr);
}

void UnregisterCategories() {
    ITfCategoryMgr* pCategoryMgr = nullptr;
    HRESULT hr = CoCreateInstance(CLSID_TF_CategoryMgr, nullptr,
                                  CLSCTX_INPROC_SERVER, IID_ITfCategoryMgr,
                                  (void**)&pCategoryMgr);
    if (SUCCEEDED(hr)) {
        pCategoryMgr->UnregisterCategory(CLSID_BanglaKeyboard,
                                          GUID_TFCAT_TIP_KEYBOARD,
                                          CLSID_BanglaKeyboard);
        pCategoryMgr->Release();
    }
}
