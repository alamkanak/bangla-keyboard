#include "text_service.h"
#include "edit_session.h"

// The public StartComposition / UpdateComposition / EndComposition methods
// just queue an EditSession; the actual work happens inside DoEditSession
// once TSF hands us a write cookie.

namespace {

// Send an edit session request, preferring synchronous edit (which lets us
// commit within the same key event) but falling back to asynchronous mode
// for hosts (e.g. Chrome / WPF) that refuse sync read/write edits.
HRESULT RequestEdit(ITfContext* pContext, TfClientId clientId, EditSession* pSession, HRESULT& hrSession) {
    HRESULT hr = pContext->RequestEditSession(clientId, pSession,
                                              TF_ES_SYNC | TF_ES_READWRITE,
                                              &hrSession);
    if (hr == TF_E_SYNCHRONOUS) {
        hr = pContext->RequestEditSession(clientId, pSession,
                                          TF_ES_ASYNCDONTCARE | TF_ES_READWRITE,
                                          &hrSession);
    }
    return hr;
}

} // namespace

HRESULT TextService::StartComposition(ITfContext* pContext) {
    if (m_pComposition) return S_OK;

    EditSession* pSession = new (std::nothrow) EditSession(EditSession::Action::Start, this, pContext, L"");
    if (!pSession) return E_OUTOFMEMORY;

    HRESULT hrSession = S_OK;
    HRESULT hr = RequestEdit(pContext, m_tfClientId, pSession, hrSession);
    pSession->Release();
    return FAILED(hr) ? hr : hrSession;
}

HRESULT TextService::UpdateComposition(ITfContext* pContext, const wchar_t* text) {
    std::wstring buf = text ? std::wstring(text) : std::wstring();
    EditSession* pSession = new (std::nothrow) EditSession(EditSession::Action::Update, this, pContext, std::move(buf));
    if (!pSession) return E_OUTOFMEMORY;

    HRESULT hrSession = S_OK;
    HRESULT hr = RequestEdit(pContext, m_tfClientId, pSession, hrSession);
    pSession->Release();
    return FAILED(hr) ? hr : hrSession;
}

HRESULT TextService::EndComposition(ITfContext* pContext) {
    if (!m_pComposition) return S_OK;

    EditSession* pSession = new (std::nothrow) EditSession(EditSession::Action::End, this, pContext, L"");
    if (!pSession) return E_OUTOFMEMORY;

    HRESULT hrSession = S_OK;
    HRESULT hr = RequestEdit(pContext, m_tfClientId, pSession, hrSession);
    pSession->Release();
    return FAILED(hr) ? hr : hrSession;
}

HRESULT TextService::DoStartComposition(TfEditCookie ec, ITfContext* pContext) {
    if (m_pComposition) return S_OK;

    ITfInsertAtSelection* pInsertAtSelection = nullptr;
    HRESULT hr = pContext->QueryInterface(IID_ITfInsertAtSelection, (void**)&pInsertAtSelection);
    if (FAILED(hr)) return hr;

    ITfRange* pRangeInsert = nullptr;
    hr = pInsertAtSelection->InsertTextAtSelection(ec, TF_IAS_QUERYONLY, L"", 0, &pRangeInsert);
    pInsertAtSelection->Release();
    if (FAILED(hr) || !pRangeInsert) return hr;

    ITfContextComposition* pContextComposition = nullptr;
    hr = pContext->QueryInterface(IID_ITfContextComposition, (void**)&pContextComposition);
    if (SUCCEEDED(hr)) {
        hr = pContextComposition->StartComposition(ec, pRangeInsert,
                                                    static_cast<ITfCompositionSink*>(this),
                                                    &m_pComposition);
        pContextComposition->Release();
    }
    pRangeInsert->Release();
    return hr;
}

HRESULT TextService::DoUpdateComposition(TfEditCookie ec, ITfContext* pContext, const std::wstring& text) {
    if (!m_pComposition) return S_OK;

    ITfRange* pRange = nullptr;
    HRESULT hr = m_pComposition->GetRange(&pRange);
    if (FAILED(hr) || !pRange) return hr;

    hr = pRange->SetText(ec, 0, text.c_str(), (LONG)text.length());
    if (SUCCEEDED(hr)) {
        ITfRange* pRangeSel = nullptr;
        if (SUCCEEDED(pRange->Clone(&pRangeSel)) && pRangeSel) {
            pRangeSel->Collapse(ec, TF_ANCHOR_END);
            TF_SELECTION tfSel = {};
            tfSel.range = pRangeSel;
            tfSel.style.ase = TF_AE_END;
            tfSel.style.fInterimChar = FALSE;
            pContext->SetSelection(ec, 1, &tfSel);
            pRangeSel->Release();
        }
    }
    pRange->Release();
    return hr;
}

HRESULT TextService::DoEndComposition(TfEditCookie ec, ITfContext* pContext) {
    if (!m_pComposition) return S_OK;

    // Collapse selection to end so the caret sits after the committed text.
    ITfRange* pRange = nullptr;
    if (SUCCEEDED(m_pComposition->GetRange(&pRange)) && pRange) {
        pRange->Collapse(ec, TF_ANCHOR_END);
        TF_SELECTION tfSel = {};
        tfSel.range = pRange;
        tfSel.style.ase = TF_AE_END;
        tfSel.style.fInterimChar = FALSE;
        pContext->SetSelection(ec, 1, &tfSel);
        pRange->Release();
    }

    m_pComposition->EndComposition(ec);
    m_pComposition->Release();
    m_pComposition = nullptr;
    return S_OK;
}
