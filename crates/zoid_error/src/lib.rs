use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Write,
    process::{ExitCode, Termination},
};

pub mod macros;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZoidError<'a, 'b> {
    pub severity: ZoidErrorSeverity,
    pub error_code: ZoidErrorCode,
    pub file: &'b str,
    pub line: u32,
    pub column: u32,
    pub source: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ZoidErrorSeverity {
    Fatal,
    Error,
    Warning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ZoidErrorCode {
    Panicing,
    NotImplemented,
    InvalidArgument,
    SyntaxError,
}

impl Error for ZoidError<'_, '_> {}

impl Termination for ZoidError<'_, '_> {
    fn report(self) -> ExitCode {
        let _ = std::io::stderr()
            .lock()
            .write_fmt(format_args!("{}\n", self));
        ExitCode::FAILURE
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ZoidResult<T>(pub Result<T, ZoidError<'static, 'static>>);

impl<T: Termination> Termination for ZoidResult<T> {
    fn report(self) -> ExitCode {
        match self.0 {
            Ok(value) => value.report(),
            Err(error) => error.report(),
        }
    }
}

impl<T> From<Result<T, ZoidError<'static, 'static>>> for ZoidResult<T> {
    fn from(result: Result<T, ZoidError<'static, 'static>>) -> Self {
        Self(result)
    }
}

impl Display for ZoidError<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.severity {
            ZoidErrorSeverity::Fatal => write!(f, "fatal: ")?,
            ZoidErrorSeverity::Error => write!(f, "error: ")?,
            ZoidErrorSeverity::Warning => write!(f, "warning: ")?,
        }
        writeln!(f, "{}:{}:{}", self.file, self.line, self.column)?;

        match self.error_code {
            ZoidErrorCode::Panicing => write!(f, "\tpanicing: {}", self.source)?,
            ZoidErrorCode::NotImplemented => write!(f, "\tnot implemented: {}", self.source)?,
            ZoidErrorCode::InvalidArgument => write!(f, "\tinvalid argument: {}", self.source)?,
            ZoidErrorCode::SyntaxError => {
                std::todo!("implement syntax error pretty printing");
            }
        }

        Ok(())
    }
}
