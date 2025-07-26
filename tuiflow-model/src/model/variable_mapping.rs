mod variable_extractor;
mod variable_injector;

use crate::model::variable::VariableSet;
pub use variable_extractor::RegexVariableExtractor;
pub use variable_injector::VariableInjector;

pub trait VariableExtractor: Clone {
    fn parse(&self, input: &str) -> Vec<VariableSet>;
}