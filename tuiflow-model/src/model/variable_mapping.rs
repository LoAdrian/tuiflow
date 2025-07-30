mod variable_extractor;
mod variable_injector;

use crate::model::variable::VariableSet;
use mockall::mock;
pub use variable_extractor::RegexVariableExtractor;
pub use variable_injector::VariableInjector;

pub trait VariableExtractor: Clone {
    fn extract(&self, input: &str) -> Vec<VariableSet>;
}

mock! {
    pub VariableExtractor{}
    
    impl Clone for VariableExtractor {
        fn clone(&self) -> Self;
    }
    
    impl VariableExtractor for VariableExtractor {
        fn extract(&self, input: &str) -> Vec<VariableSet>;
    }
}