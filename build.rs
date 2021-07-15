#[cfg(not(any(feature = "rustls_backend_marker", feature = "native_tls_backend_marker")))]
compile_error!(
    "You have not chosen a backend, which is required for HTTP or Gateway functionality.\n\
    Either of the `rustls_backend` or `native_tls_backend` features must be selected.\n\
    - `rustls_backend` uses Rustls, a pure Rust TLS-implemenation.\n\
    - `native_tls_backend` uses SChannel on Windows, Secure Transport on macOS, \
    and OpenSSL on other platforms.\n\
    If you are unsure, go with `rustls_backend`."
);

fn main() {}
