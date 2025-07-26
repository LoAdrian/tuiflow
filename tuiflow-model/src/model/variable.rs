use std::collections::HashSet;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub(crate) name: VariableName,
    pub(crate) value: String,
}

impl Variable {
    pub fn new(name: VariableName, value: String) -> Self {
        Self { name, value }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariableName(pub(crate) String);

impl Deref for VariableName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for VariableName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug)]
pub struct VariableSet(HashSet<Variable>);

impl Deref for VariableSet {
    type Target = HashSet<Variable>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromIterator<Variable> for VariableSet {
    fn from_iter<T: IntoIterator<Item = Variable>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}