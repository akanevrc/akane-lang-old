
#[macro_export]
macro_rules! bail_info {
    ($info:expr, $msg:literal) => {
        {
            let info = $info;
            anyhow::bail!("({}, {}): {}", info.line, info.column, $msg)
        }
    };

    ($info:expr, $msg:expr, $($arg:tt)*) => {
        {
            let info = $info;
            anyhow::bail!("({}, {}): {}", info.line, info.column, format!($msg, $($arg)*))
        }
    };
}
