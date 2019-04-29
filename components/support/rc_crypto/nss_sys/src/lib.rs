/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

#[cfg_attr(feature = "cargo-clippy", allow(clippy::all))]
mod bindings;

pub use bindings::*;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_void};

// Remap some constants.
pub const SECSuccess: SECStatus = _SECStatus_SECSuccess;
pub const SECFailure: SECStatus = _SECStatus_SECFailure;
pub const PR_FALSE: PRBool = 0;
pub const PR_TRUE: PRBool = 1;
pub const CK_FALSE: CK_BBOOL = 0;
pub const CK_TRUE: CK_BBOOL = 1;

// This is the version this crate is claiming to be compatible with.
// We check it at runtime using `NSS_VersionCheck`.
pub const COMPATIBLE_NSS_VERSION: &str = "3.26";

// Code adapted from https://stackoverflow.com/a/35591693. I am not this kind of smart.
macro_rules! nss_exports {
    ($(unsafe fn $fn_name:ident($($arg:ident: $argty:ty),*)$( -> $ret:ty)?;)*) => {$(
        #[cfg(not(target_os = "ios"))]
        lazy_static::lazy_static! {
            pub static ref $fn_name: libloading::Symbol<'static, unsafe extern fn($($arg: $argty),*)$( -> $ret)?> = {
                unsafe {
                    LIBNSS3.get(stringify!($fn_name).as_bytes()).expect(stringify!(Could not get $fn_name handle))
                }
            };
        }
        #[cfg(target_os = "ios")]
        extern "C" {
            pub fn $fn_name($($arg: $argty),*)$( -> $ret)?;
        }
    )*};
}

#[cfg(not(target_os = "ios"))]
lazy_static::lazy_static! {
    // Lib handle.
    static ref LIBNSS3: libloading::Library = {
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        const LIB_NAME: &str = "libnss3.dylib";
        #[cfg(any(target_os = "linux", target_os = "android"))]
        const LIB_NAME: &str = "libnss3.so";
        #[cfg(target_os = "windows")]
        const LIB_NAME: &str = "nss3.dll";
        libloading::Library::new(LIB_NAME).expect("Cannot load libnss3.")
    };
}

nss_exports! {
    unsafe fn PR_GetError() -> PRErrorCode;
    unsafe fn PR_GetErrorTextLength() -> PRInt32;
    unsafe fn PR_GetErrorText(out: *mut c_uchar) -> PRInt32;
    unsafe fn NSS_NoDB_Init(configdir: *const c_char) -> SECStatus;
    unsafe fn NSS_InitContext(configdir: *const c_char, certPrefix: *const c_char, keyPrefix: *const c_char, secmodName: *const c_char, initParams: *mut NSSInitParameters, flags: PRUint32) -> *mut NSSInitContext;
    unsafe fn NSS_IsInitialized() -> PRBool;
    unsafe fn NSS_GetVersion() -> *const c_char;
    unsafe fn NSS_VersionCheck(importedVersion: *const c_char) -> PRBool;
    unsafe fn NSS_SecureMemcmp(ia: *const c_void, ib: *const c_void, n: usize) -> c_int;
    unsafe fn PK11_HashBuf(hashAlg: SECOidTag::Type, out: *mut c_uchar, r#in: *const c_uchar, len: PRInt32) -> SECStatus;
    unsafe fn PK11_FreeSlot(slot: *mut PK11SlotInfo);
    unsafe fn PK11_FreeSymKey(symKey: *mut PK11SymKey);
    unsafe fn PK11_DestroyContext(context: *mut PK11Context, freeit: PRBool);
    unsafe fn PK11_GetInternalSlot() -> *mut PK11SlotInfo;
    unsafe fn PK11_ImportSymKey(slot: *mut PK11SlotInfo, r#type: CK_MECHANISM_TYPE, origin: PK11Origin::Type, operation: CK_ATTRIBUTE_TYPE, key: *mut SECItem, wincx: *mut c_void) -> *mut PK11SymKey;
    unsafe fn PK11_CreateContextBySymKey(r#type: CK_MECHANISM_TYPE, operation: CK_ATTRIBUTE_TYPE, symKey: *mut PK11SymKey, param: *mut SECItem) -> *mut PK11Context;
    unsafe fn PK11_DigestBegin(cx: *mut PK11Context) -> SECStatus;
    unsafe fn PK11_DigestOp(context: *mut PK11Context, r#in: *const c_uchar, len: c_uint) -> SECStatus;
    unsafe fn PK11_DigestFinal(context: *mut PK11Context, data: *mut c_uchar, outLen: *mut c_uint, len: c_uint) -> SECStatus;
    unsafe fn PK11_GenerateRandom(data: *mut c_uchar, len: c_int) -> SECStatus;
    unsafe fn PK11_Derive(baseKey: *mut PK11SymKey, mechanism: CK_MECHANISM_TYPE, param: *mut SECItem, target: CK_MECHANISM_TYPE, operation: CK_ATTRIBUTE_TYPE, keySize: c_int) -> *mut PK11SymKey;
    unsafe fn PK11_ExtractKeyValue(symKey: *mut PK11SymKey) -> SECStatus;
    unsafe fn PK11_GetKeyData(symKey: *mut PK11SymKey) -> *mut SECItem;
}
