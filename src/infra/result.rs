use crate::infra::RacketInfraError;

#[allow(dead_code)]
pub(crate) type RacketInfraResult<T> = Result<T, RacketInfraError>;
