#include "text_service.h"
#include <string>
#include <shlobj.h>

TextService::TextService()
    : m_cRef(1), m_pThreadMgr(nullptr), m_tfClientId(TF_CLIENTID_NULL),
      m_pComposition(nullptr), m_dwFlags(0), m_engineInitialized(false) {
    DllAddRef();
}

TextService::~TextService() {
    if (m_engineInitialized) {
        bk_engine_shutdown();
    }
    DllRelease();
}

HRESULT TextService::CreateInstance(IUnknown* pUnkOuter, REFIID riid, void** ppvObj) {
    if (pUnkOuter != nullptr) return CLASS_E_NOAGGREGATION;

    TextService* pService = new (std::nothrow) TextService();
    if (pService == nullptr) return E_OUTOFMEMORY;

    HRESULT hr = pService->QueryInterface(riid, ppvObj);
    pService->Release();
    return hr;
}

// IUnknown
STDMETHODIMP TextService::QueryInterface(REFIID riid, void** ppvObj) {
    if (ppvObj == nullptr) return E_INVALIDARG;

    *ppvObj = nullptr;

    if (IsEqualIID(riid, IID_IUnknown) || IsEqualIID(riid, IID_ITfTextInputProcessor)) {
        *ppvObj = static_cast<ITfTextInputProcessorEx*>(this);
    } else if (IsEqualIID(riid, IID_ITfTextInputProcessorEx)) {
        *ppvObj = static_cast<ITfTextInputProcessorEx*>(this);
    } else if (IsEqualIID(riid, IID_ITfKeyEventSink)) {
        *ppvObj = static_cast<ITfKeyEventSink*>(this);
    } else if (IsEqualIID(riid, IID_ITfCompositionSink)) {
        *ppvObj = static_cast<ITfCompositionSink*>(this);
    } else if (IsEqualIID(riid, IID_ITfDisplayAttributeProvider)) {
        *ppvObj = static_cast<ITfDisplayAttributeProvider*>(this);
    }

    if (*ppvObj) {
        AddRef();
        return S_OK;
    }

    return E_NOINTERFACE;
}

STDMETHODIMP_(ULONG) TextService::AddRef() {
    return InterlockedIncrement(&m_cRef);
}

STDMETHODIMP_(ULONG) TextService::Release() {
    LONG cRef = InterlockedDecrement(&m_cRef);
    if (cRef == 0) {
        delete this;
    }
    return cRef;
}

// ITfTextInputProcessor
STDMETHODIMP TextService::Activate(ITfThreadMgr* pThreadMgr, TfClientId tfClientId) {
    return ActivateEx(pThreadMgr, tfClientId, 0);
}

STDMETHODIMP TextService::Deactivate() {
    UninitKeystrokeSink();

    if (m_pThreadMgr) {
        m_pThreadMgr->Release();
        m_pThreadMgr = nullptr;
    }
    m_tfClientId = TF_CLIENTID_NULL;

    return S_OK;
}

// ITfTextInputProcessorEx
STDMETHODIMP TextService::ActivateEx(ITfThreadMgr* pThreadMgr, TfClientId tfClientId, DWORD dwFlags) {
    m_pThreadMgr = pThreadMgr;
    m_pThreadMgr->AddRef();
    m_tfClientId = tfClientId;
    m_dwFlags = dwFlags;

    // Initialize Rust engine
    if (!m_engineInitialized) {
        std::wstring dataPath = GetDataPath();
        int wLen = WideCharToMultiByte(CP_UTF8, 0, dataPath.c_str(), -1, nullptr, 0, nullptr, nullptr);
        std::string utf8Path(wLen - 1, '\0');
        WideCharToMultiByte(CP_UTF8, 0, dataPath.c_str(), -1, utf8Path.data(), wLen, nullptr, nullptr);

        if (bk_engine_init(utf8Path.c_str()) == 0) {
            m_engineInitialized = true;
        }
    }

    HRESULT hr = InitKeystrokeSink();
    if (FAILED(hr)) {
        Deactivate();
        return hr;
    }

    return S_OK;
}

std::wstring TextService::GetDataPath() {
    wchar_t dllPath[MAX_PATH];
    GetModuleFileNameW(g_hInstance, dllPath, MAX_PATH);

    std::wstring path(dllPath);
    size_t lastSlash = path.find_last_of(L'\\');
    if (lastSlash != std::wstring::npos) {
        path = path.substr(0, lastSlash);
    }
    path += L"\\data";
    return path;
}

// ITfCompositionSink
STDMETHODIMP TextService::OnCompositionTerminated(TfEditCookie ecWrite, ITfComposition* pComposition) {
    bk_reset();
    if (m_pComposition) {
        m_pComposition->Release();
        m_pComposition = nullptr;
    }
    return S_OK;
}

// ITfDisplayAttributeProvider
STDMETHODIMP TextService::EnumDisplayAttributeInfo(IEnumTfDisplayAttributeInfo** ppEnum) {
    if (ppEnum == nullptr) return E_INVALIDARG;
    *ppEnum = nullptr;
    return E_NOTIMPL;
}

STDMETHODIMP TextService::GetDisplayAttributeInfo(REFGUID guid, ITfDisplayAttributeInfo** ppInfo) {
    if (ppInfo == nullptr) return E_INVALIDARG;
    *ppInfo = nullptr;
    return E_NOTIMPL;
}
