#include "class_factory.h"
#include "text_service.h"

ClassFactory::ClassFactory(REFCLSID rclsid, HRESULT (*pfnCreateInstance)(IUnknown*, REFIID, void**))
    : m_cRef(1), m_clsid(rclsid), m_pfnCreateInstance(pfnCreateInstance) {
    DllAddRef();
}

STDMETHODIMP ClassFactory::QueryInterface(REFIID riid, void** ppvObj) {
    if (ppvObj == nullptr) return E_INVALIDARG;

    *ppvObj = nullptr;

    if (IsEqualIID(riid, IID_IUnknown) || IsEqualIID(riid, IID_IClassFactory)) {
        *ppvObj = static_cast<IClassFactory*>(this);
    }

    if (*ppvObj) {
        AddRef();
        return S_OK;
    }

    return E_NOINTERFACE;
}

STDMETHODIMP_(ULONG) ClassFactory::AddRef() {
    return InterlockedIncrement(&m_cRef);
}

STDMETHODIMP_(ULONG) ClassFactory::Release() {
    LONG cRef = InterlockedDecrement(&m_cRef);
    if (cRef == 0) {
        DllRelease();
        delete this;
    }
    return cRef;
}

STDMETHODIMP ClassFactory::CreateInstance(IUnknown* pUnkOuter, REFIID riid, void** ppvObj) {
    return m_pfnCreateInstance(pUnkOuter, riid, ppvObj);
}

STDMETHODIMP ClassFactory::LockServer(BOOL fLock) {
    if (fLock) {
        DllAddRef();
    } else {
        DllRelease();
    }
    return S_OK;
}
