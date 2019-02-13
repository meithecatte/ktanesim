#[macro_export]
macro_rules! awaitt {
    ( $e:expr ) => {{
        use tokio_async_await::compat::forward::IntoAwaitable;
        await!($e.into_awaitable())
    }};
}
