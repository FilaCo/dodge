use crate::infra::RacketInfraError;

pub(crate) type RacketInfraResult<T> = Result<T, RacketInfraError>;
