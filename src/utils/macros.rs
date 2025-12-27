#[macro_export]
macro_rules! mybox {
    ($val: expr) => {
        Box::new($val)
    }
}
#[macro_export]
macro_rules! make_error {
    ($sentence:expr, $label:expr, $span:expr) => {
        LabeledError::new($sentence.to_string())
        .with_label($label, $span)
    };
}

#[macro_export]
macro_rules! return_error {
    ($sentence:expr, $label:expr, $span:expr) => {
        return Err(make_error!($sentence, $label, $span))
    };
}
