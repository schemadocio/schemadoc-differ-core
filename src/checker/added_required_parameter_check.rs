use std::cell::RefCell;

use crate::checker::{ValidationIssue, ValidationIssuer};
use crate::core::DiffResult;
use crate::path_pointer::PathPointer;
use crate::schema_diff::{OperationDiff, ParameterDiff};

use crate::visitor::DiffVisitor;

pub struct AddedRequiredParameterCheck {
    pointers: RefCell<Vec<PathPointer>>,
}

impl<'s> DiffVisitor<'s> for AddedRequiredParameterCheck {
    fn visit_operation(
        &self,
        pointer: &PathPointer,
        _: &str,
        _: &'s DiffResult<OperationDiff>,
    ) -> bool {
        // Operation must be UPDATED
        pointer.is_updated()
    }

    fn visit_parameter(
        &self,
        pointer: &PathPointer,
        parameter_diff_result: &'s DiffResult<ParameterDiff>,
    ) -> bool {
        // Parameter Must be UPSERTED
        if !pointer.is_upserted() {
            return false;
        }

        if let Some(parameter_diff) = parameter_diff_result.get() {
            if parameter_diff.required.is_upserted() {
                if let Some(required) = parameter_diff.required.get() {
                    if *required {
                        self.pointers.borrow_mut().push(pointer.clone())
                    }
                }
            }
        }

        false
    }
}

impl Default for AddedRequiredParameterCheck {
    fn default() -> Self {
        AddedRequiredParameterCheck {
            pointers: RefCell::new(vec![]),
        }
    }
}

impl<'s> ValidationIssuer<'s> for AddedRequiredParameterCheck {
    fn id(&self) -> &'static str {
        "added-required-parameter"
    }

    fn visitor(&self) -> &dyn DiffVisitor<'s> {
        self
    }

    fn issues(&self) -> Option<Vec<ValidationIssue>> {
        let pointers = std::mem::take(&mut *self.pointers.borrow_mut());

        let issues = pointers
            .into_iter()
            .map(|path| ValidationIssue::new(path, self.id(), true))
            .collect::<Vec<ValidationIssue>>();

        Some(issues)
    }
}
