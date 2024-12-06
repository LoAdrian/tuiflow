mod error;
mod variable_mapper;

pub(crate) use error::{VariableMapperCompilationError, VariableMappingError};
use mockall::mock;
pub(crate) use variable_mapper::RegexVariableMapper;

pub trait VariableMapper: Clone {
    fn map(&self, input: &str) -> impl Iterator<Item = Result<String, VariableMappingError>>;
}

mock! {
    #[derive(Clone)]
    pub(crate) VariableMapper{}

    impl Clone for VariableMapper{
        fn clone(&self) -> Self;
    }

    impl VariableMapper for VariableMapper{
        fn map(&self, input: &str) -> impl Iterator<Item = Result<String, VariableMappingError>>;
    }
}
