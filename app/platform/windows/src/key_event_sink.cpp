#include "text_service.h"

HRESULT TextService::InitKeystrokeSink() {
    ITfKeystrokeMgr* pKeystrokeMgr = nullptr;
    HRESULT hr = m_pThreadMgr->QueryInterface(IID_ITfKeystrokeMgr, (void**)&pKeystrokeMgr);
    if (FAILED(hr)) return hr;

    hr = pKeystrokeMgr->AdviseKeyEventSink(m_tfClientId, static_cast<ITfKeyEventSink*>(this), TRUE);
    pKeystrokeMgr->Release();
    return hr;
}

HRESULT TextService::UninitKeystrokeSink() {
    ITfKeystrokeMgr* pKeystrokeMgr = nullptr;
    HRESULT hr = m_pThreadMgr->QueryInterface(IID_ITfKeystrokeMgr, (void**)&pKeystrokeMgr);
    if (FAILED(hr)) return hr;

    hr = pKeystrokeMgr->UnadviseKeyEventSink(m_tfClientId);
    pKeystrokeMgr->Release();
    return hr;
}

STDMETHODIMP TextService::OnSetFocus(BOOL fForeground) {
    return S_OK;
}

STDMETHODIMP TextService::OnTestKeyDown(ITfContext* pContext, WPARAM wParam, LPARAM lParam, BOOL* pfEaten) {
    *pfEaten = FALSE;

    // Eat keys when composing or for normal character input
    if (bk_is_composing()) {
        switch (wParam) {
        case VK_RETURN:
        case VK_SPACE:
        case VK_BACK:
        case VK_ESCAPE:
            *pfEaten = TRUE;
            return S_OK;
        }
    }

    // Eat printable character keys
    if (wParam >= 0x20 && wParam <= 0x7E) {
        *pfEaten = TRUE;
    }

    return S_OK;
}

STDMETHODIMP TextService::OnTestKeyUp(ITfContext* pContext, WPARAM wParam, LPARAM lParam, BOOL* pfEaten) {
    *pfEaten = FALSE;
    return S_OK;
}

STDMETHODIMP TextService::OnKeyDown(ITfContext* pContext, WPARAM wParam, LPARAM lParam, BOOL* pfEaten) {
    *pfEaten = FALSE;

    if (!m_engineInitialized) return S_OK;

    // Handle special keys
    switch (wParam) {
    case VK_RETURN:
        if (bk_is_composing()) {
            char* committed = bk_handle_enter();
            if (committed) {
                int wLen = MultiByteToWideChar(CP_UTF8, 0, committed, -1, nullptr, 0);
                std::wstring wtext(wLen - 1, L'\0');
                MultiByteToWideChar(CP_UTF8, 0, committed, -1, wtext.data(), wLen);
                bk_free_string(committed);
                UpdateComposition(pContext, wtext.c_str());
            }
            EndComposition(pContext);
            *pfEaten = TRUE;
        }
        return S_OK;

    case VK_SPACE:
        if (bk_is_composing()) {
            char* committed = bk_handle_space();
            if (committed) {
                int wLen = MultiByteToWideChar(CP_UTF8, 0, committed, -1, nullptr, 0);
                std::wstring wtext(wLen - 1, L'\0');
                MultiByteToWideChar(CP_UTF8, 0, committed, -1, wtext.data(), wLen);
                bk_free_string(committed);
                UpdateComposition(pContext, wtext.c_str());
            }
            EndComposition(pContext);
            *pfEaten = TRUE;
        }
        return S_OK;

    case VK_BACK:
        if (bk_is_composing()) {
            int result = bk_handle_backspace();
            if (result == 0) {
                EndComposition(pContext);
            } else if (result == 1) {
                char* preview = bk_get_preview();
                if (preview) {
                    int wLen = MultiByteToWideChar(CP_UTF8, 0, preview, -1, nullptr, 0);
                    std::wstring wtext(wLen - 1, L'\0');
                    MultiByteToWideChar(CP_UTF8, 0, preview, -1, wtext.data(), wLen);
                    bk_free_string(preview);
                    UpdateComposition(pContext, wtext.c_str());
                }
            }
            *pfEaten = TRUE;
        }
        return S_OK;

    case VK_ESCAPE:
        if (bk_is_composing()) {
            bk_reset();
            EndComposition(pContext);
            *pfEaten = TRUE;
        }
        return S_OK;
    }

    // Get the character from the key
    BYTE keyState[256];
    GetKeyboardState(keyState);
    wchar_t wch[2] = { 0 };
    int result = ToUnicode((UINT)wParam, (UINT)((lParam >> 16) & 0xFF), keyState, wch, 2, 0);
    if (result != 1) return S_OK;

    char ch = (char)wch[0];
    if (ch < 0x20 || ch > 0x7E) return S_OK;

    bool shift = (GetKeyState(VK_SHIFT) & 0x8000) != 0;

    int action = bk_handle_key(ch, shift);
    if (action == 1) { // UpdatePreview
        if (!m_pComposition) {
            StartComposition(pContext);
        }
        char* preview = bk_get_preview();
        if (preview) {
            int wLen = MultiByteToWideChar(CP_UTF8, 0, preview, -1, nullptr, 0);
            std::wstring wtext(wLen - 1, L'\0');
            MultiByteToWideChar(CP_UTF8, 0, preview, -1, wtext.data(), wLen);
            bk_free_string(preview);
            UpdateComposition(pContext, wtext.c_str());
        }
        *pfEaten = TRUE;
    } else if (action == 0) { // Commit
        EndComposition(pContext);
        *pfEaten = TRUE;
    }

    return S_OK;
}

STDMETHODIMP TextService::OnKeyUp(ITfContext* pContext, WPARAM wParam, LPARAM lParam, BOOL* pfEaten) {
    *pfEaten = FALSE;
    return S_OK;
}

STDMETHODIMP TextService::OnPreservedKey(ITfContext* pContext, REFGUID rguid, BOOL* pfEaten) {
    *pfEaten = FALSE;
    return S_OK;
}
