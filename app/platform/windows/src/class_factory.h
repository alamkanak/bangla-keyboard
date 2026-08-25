#pragma once

#include <windows.h>
#include <msctf.h>
#include "globals.h"

class TextService;

class ClassFactory : public IClassFactory {
public:
    // IUnknown
    STDMETHODIMP QueryInterface(REFIID riid, void** ppvObj);
    STDMETHODIMP_(ULONG) AddRef();
    STDMETHODIMP_(ULONG) Release();

    // IClassFactory
    STDMETHODIMP CreateInstance(IUnknown* pUnkOuter, REFIID riid, void** ppvObj);
    STDMETHODIMP LockServer(BOOL fLock);

    ClassFactory(REFCLSID rclsid, HRESULT (*pfnCreateInstance)(IUnknown* pUnkOuter, REFIID riid, void** ppvObj));

private:
    LONG m_cRef;
    CLSID m_clsid;
    HRESULT (*m_pfnCreateInstance)(IUnknown* pUnkOuter, REFIID riid, void** ppvObj);
};
