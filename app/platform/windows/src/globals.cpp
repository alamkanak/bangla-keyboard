#include "globals.h"

HINSTANCE g_hInstance = nullptr;
LONG g_cRefDll = 0;

void DllAddRef() {
    InterlockedIncrement(&g_cRefDll);
}

void DllRelease() {
    InterlockedDecrement(&g_cRefDll);
}
