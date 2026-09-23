/*
 * Accessors for bitfields of nginx structures.
 *
 * bindgen computes bitfield positions itself rather than asking the C
 * compiler, and its allocation does not match the AArch64 rule that a
 * bitfield may not straddle its container.  The generated accessors are
 * therefore off by two bits on that platform, which silently breaks
 * `header_only` (see issue #338).
 *
 * Letting the C compiler perform the access removes the guesswork: it
 * knows the layout by definition, on every target.
 */

#include <ngx_config.h>
#include <ngx_core.h>

#if (NGX_RS_FEATURE_HTTP)

#include <ngx_http.h>

ngx_uint_t
ngx_rs_http_request_get_header_only(const ngx_http_request_t *r)
{
    return r->header_only;
}

void
ngx_rs_http_request_set_header_only(ngx_http_request_t *r, ngx_uint_t value)
{
    r->header_only = value ? 1 : 0;
}

#endif
