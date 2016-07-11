use winapi::*;

pub const SERVICE_AUTO_START: DWORD = 0x00000002;

pub const SERVICE_ERROR_NORMAL: DWORD = 0x00000001;

extern "system" {
    pub fn OpenSCManagerW(lpMachineName: LPCWSTR, lpDatabaseName: LPCWSTR, dwDesiredAccess: DWORD) -> SC_HANDLE;
    pub fn CreateServiceW(hSCManager: SC_HANDLE, lpServiceName: LPCWSTR, lpDisplayName: LPCWSTR, dwDesiredAccess: DWORD, dwServiceType: DWORD, dwStartType: DWORD, dwErrorControl: DWORD, lpBinaryPathName: LPCWSTR, lpLoadOrderGroup: LPCWSTR, lpdwTagId: LPDWORD, lpDependencies: LPCWSTR, lpServiceStartName: LPCWSTR, lpPassword: LPCWSTR) -> SC_HANDLE;
    pub fn CloseServiceHandle(hSCObject: SC_HANDLE) -> BOOL;
}
