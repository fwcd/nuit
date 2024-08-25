#[macro_export]

/// Clones values for capture in a closure.
macro_rules! clone {
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($i:ident),*| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$($i,)*| $body
        }
    );
}
