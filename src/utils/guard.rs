#[macro_export]
macro_rules! guard {
    ($condition:expr) => {
        if !$condition {
            return;
        }
    };
    ($condition:expr, $ret:expr) => {
        if !$condition {
            return $ret;
        }
    };
}
