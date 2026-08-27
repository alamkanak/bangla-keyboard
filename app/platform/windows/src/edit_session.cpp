#include "edit_session.h"
#include "text_service.h"

EditSession::EditSession(Action action, TextService* pService, ITfContext* pContext, std::wstring text)
    : m_cRef(1), m_action(action), m_pService(pService), m_pContext(pContext), m_text(std::move(text)) {
    if (m_pService) m_pService->AddRef();
    if (m_pContext) m_pContext->AddRef();
}

EditSession::~EditSession() {
    if (m_pService) m_pService->Release();
    if (m_pContext) m_pContext->Release();
}

STDMETHODIMP EditSession::QueryInterface(REFIID riid, void** ppvObj) {
    if (!ppvObj) return E_INVALIDARG;
    if (IsEqualIID(riid, IID_IUnknown) || IsEqualIID(riid, IID_ITfEditSession)) {
        *ppvObj = static_cast<ITfEditSession*>(this);
        AddRef();
        return S_OK;
    }
    *ppvObj = nullptr;
    return E_NOINTERFACE;
}

STDMETHODIMP_(ULONG) EditSession::AddRef() {
    return InterlockedIncrement(&m_cRef);
}

STDMETHODIMP_(ULONG) EditSession::Release() {
    LONG cRef = InterlockedDecrement(&m_cRef);
    if (cRef == 0) delete this;
    return cRef;
}

STDMETHODIMP EditSession::DoEditSession(TfEditCookie ec) {
    if (!m_pService || !m_pContext) return E_UNEXPECTED;
    switch (m_action) {
        case Action::Start:  return m_pService->DoStartComposition(ec, m_pContext);
        case Action::Update: return m_pService->DoUpdateComposition(ec, m_pContext, m_text);
        case Action::End:    return m_pService->DoEndComposition(ec, m_pContext);
    }
    return E_UNEXPECTED;
}
