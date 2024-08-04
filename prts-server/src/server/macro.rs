#[macro_export]
macro_rules! route {
    () => {
        axum::Router::new()
    };
    ($(post $path:expr => $handler:ident), +) => {
        route!().route($path, post($handler))
    };
    ($(get $path:expr => $handler:ident), +) => {
        route!().route($path, get($handler))
    };
}
