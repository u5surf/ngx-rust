use core::mem::offset_of;

use crate::bindings::ngx_http_conf_ctx_t;

/// The offset of the `main_conf` field in the `ngx_http_conf_ctx_t` struct.
///
/// This is used to access the main configuration context for an HTTP module.
pub const NGX_HTTP_MAIN_CONF_OFFSET: usize = offset_of!(ngx_http_conf_ctx_t, main_conf);

/// The offset of the `srv_conf` field in the `ngx_http_conf_ctx_t` struct.
///
/// This is used to access the server configuration context for an HTTP module.
pub const NGX_HTTP_SRV_CONF_OFFSET: usize = offset_of!(ngx_http_conf_ctx_t, srv_conf);

/// The offset of the `loc_conf` field in the `ngx_http_conf_ctx_t` struct.
///
/// This is used to access the location configuration context for an HTTP module.
pub const NGX_HTTP_LOC_CONF_OFFSET: usize = offset_of!(ngx_http_conf_ctx_t, loc_conf);

use crate::bindings::{ngx_http_request_t, ngx_uint_t};

unsafe extern "C" {
    /// Reads `r->header_only`.
    ///
    /// bindgen derives bitfield positions on its own rather than asking the C
    /// compiler, and its allocation does not match the AArch64 rule that a
    /// bitfield may not straddle its container, so the generated accessor
    /// reads the wrong bit there. This goes through the C compiler instead,
    /// which knows the layout on every target.
    ///
    /// # Safety
    ///
    /// `r` must point at a valid [`ngx_http_request_t`].
    #[link_name = "ngx_rs_http_request_get_header_only"]
    pub fn ngx_http_request_get_header_only(r: *const ngx_http_request_t) -> ngx_uint_t;

    /// Sets `r->header_only`. See [`ngx_http_request_get_header_only`].
    ///
    /// # Safety
    ///
    /// `r` must point at a valid [`ngx_http_request_t`].
    #[link_name = "ngx_rs_http_request_set_header_only"]
    pub fn ngx_http_request_set_header_only(r: *mut ngx_http_request_t, value: ngx_uint_t);
}
