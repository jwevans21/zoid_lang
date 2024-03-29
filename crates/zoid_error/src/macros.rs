#[macro_export]
macro_rules! panic {
    () => {
        return Err($crate::ZoidError {
            severity: $crate::ZoidErrorSeverity::Fatal,
            error_code: $crate::ZoidErrorCode::Panicing,
            file: file!(),
            line: line!(),
            column: column!(),
            source: "",
        })
        .into()
    };
    ($source:expr) => {
        return Err($crate::ZoidError {
            severity: $crate::ZoidErrorSeverity::Fatal,
            error_code: $crate::ZoidErrorCode::Panicing,
            file: file!(),
            line: line!(),
            column: column!(),
            source: $source,
        })
        .into()
    };
}

#[macro_export]
macro_rules! unimplemented {
    () => {
        return Err($crate::ZoidError {
            severity: $crate::ZoidErrorSeverity::Error,
            error_code: $crate::ZoidErrorCode::NotImplemented,
            file: file!(),
            line: line!(),
            column: column!(),
            source: "",
        })
        .into()
    };
    ($source:expr) => {
        return Err($crate::ZoidError {
            severity: $crate::ZoidErrorSeverity::Error,
            error_code: $crate::ZoidErrorCode::NotImplemented,
            file: file!(),
            line: line!(),
            column: column!(),
            source: $source,
        })
        .into()
    };
}

#[macro_export]
macro_rules! todo {
    () => {
        return Err($crate::ZoidError {
            severity: $crate::ZoidErrorSeverity::Error,
            error_code: $crate::ZoidErrorCode::NotImplemented,
            file: file!(),
            line: line!(),
            column: column!(),
            source: "",
        })
        .into()
    };
    ($source:expr) => {
        return Err($crate::ZoidError {
            severity: $crate::ZoidErrorSeverity::Error,
            error_code: $crate::ZoidErrorCode::NotImplemented,
            file: file!(),
            line: line!(),
            column: column!(),
            source: $source,
        })
        .into()
    };
}
