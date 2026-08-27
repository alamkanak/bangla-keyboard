#pragma once

#include <windows.h>
#include <msctf.h>
#include <string>

class TextService;

// One-shot ITfEditSession that dispatches into TextService for the specific
// composition action requested. TSF requires all text mutation to happen
// inside an edit session callback that owns a TfEditCookie write token.
class EditSession : public ITfEditSession {
public:
    enum class Action {
        Start,
        Update,
        End,
    };

    EditSession(Action action, TextService* pService, ITfContext* pContext, std::wstring text);
    virtual ~EditSession();

    // IUnknown
    STDMETHODIMP QueryInterface(REFIID riid, void** ppvObj) override;
    STDMETHODIMP_(ULONG) AddRef() override;
    STDMETHODIMP_(ULONG) Release() override;

    // ITfEditSession
    STDMETHODIMP DoEditSession(TfEditCookie ec) override;

private:
    LONG m_cRef;
    Action m_action;
    TextService* m_pService;
    ITfContext* m_pContext;
    std::wstring m_text;
};
