use crate::model::variable::VariableSet;
use std::ops::Deref;
use tuiflow_model_contracts::error::VariableMappingError;

#[derive(Clone, Debug)]
pub struct VariableInjector {
    output_pattern: String,
}

impl VariableInjector {
    pub fn new(output_pattern: String) -> Self {
        Self {
            output_pattern,
        } 
    }
    
   pub(crate) fn fill(&self, variables: &VariableSet) -> Result<String, VariableMappingError> {
       let mut result = self.output_pattern.clone();
       variables.iter().for_each(|var| result = result.replace(format!("<{}>", var.name.deref()).as_str(), var.value.as_str()));
       if result.contains("<") {
           return Err(VariableMappingError);
       }
       Ok(result) 
   }
}