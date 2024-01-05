use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum LCError {
    #[error(transparent)]
    #[diagnostic(code(lc::io_error))]
    IoError(#[from] std::io::Error),
}
