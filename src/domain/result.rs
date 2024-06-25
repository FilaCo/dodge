use crate::domain::RacketDomainError;

#[allow(dead_code)]
pub(crate) type RacketDomainResult<T> = Result<T, RacketDomainError>;
