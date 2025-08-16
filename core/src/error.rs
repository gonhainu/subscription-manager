use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("年は1900-2200の範囲で指定してください: {0}")]
    InvalidYear(i32),

    #[error("月は1-12の範囲で指定してください: {0}")]
    InvalidMonth(u32),
}

pub type DomainResult<T> = Result<T, DomainError>;
