#[macro_export]
macro_rules! awaitt {
    ( $e:expr ) => {
        await!($e.into_awaitable())
    };
}
