#[macro_export]
macro_rules! s {
    ($shead:tt $($sseg:tt)*) => {
        $shead.to_string() $(+&$sseg.to_string())*
    }
}