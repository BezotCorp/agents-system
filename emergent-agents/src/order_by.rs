use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct OrderBy {
    formula: String,
    values: BTreeMap<String, f64>,
}

impl OrderBy {
    pub(crate) fn new(formula: impl Into<String>, values: BTreeMap<String, f64>) -> Self {
        Self {
            formula: formula.into(),
            values,
        }
    }

    pub(crate) fn formula(&self) -> &str {
        &self.formula
    }

    pub(crate) fn values(&self) -> &BTreeMap<String, f64> {
        &self.values
    }

    pub(crate) fn set_value(&mut self, name: impl Into<String>, value: f64) {
        self.values.insert(name.into(), value);
    }

    pub(crate) fn next_index(&self, current_index: usize) -> Result<usize, String> {
        let mut namespace = fasteval::StringToF64Namespace::new();

        for (name, value) in &self.values {
            namespace.insert(name.clone(), *value);
        }

        namespace.insert("x".to_owned(), current_index as f64);

        let result = fasteval::ez_eval(&self.formula, &mut namespace).map_err(|error| {
            format!(
                "failed to evaluate order formula {:?}: {error}",
                self.formula,
            )
        })?;

        if !result.is_finite() {
            return Err(format!(
                "order formula {:?} produced non-finite value {result}",
                self.formula,
            ));
        }

        if result.fract() != 0.0 {
            return Err(format!(
                "order formula {:?} produced non-integer index {result}",
                self.formula,
            ));
        }

        if result < 0.0 || result > usize::MAX as f64 {
            return Err(format!(
                "order formula {:?} produced invalid index {result}",
                self.formula,
            ));
        }

        Ok(result as usize)
    }
}
