use crate::domain::RacketDomainError;
use crate::infra::RacketInfraError;
use crate::util::RacketUtilError;

#[derive(Debug)]
pub enum RacketError {}

impl From<RacketDomainError> for RacketError {
    fn from(_value: RacketDomainError) -> Self {
        todo!()
    }
}

impl From<RacketInfraError> for RacketError {
    fn from(_value: RacketInfraError) -> Self {
        todo!()
    }
}

impl From<RacketUtilError> for RacketError {
    fn from(_value: RacketUtilError) -> Self {
        todo!()
    }
}
