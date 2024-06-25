use crate::domain::RacketDomainError;

pub(crate) type RacketDomainResult<T> = Result<T, RacketDomainError>;
