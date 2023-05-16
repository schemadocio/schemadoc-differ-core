use std::cell::RefCell;

use crate::core::DiffResult;
use crate::path_pointer::PathPointer;

use crate::checker::{ValidationIssue, ValidationIssuer};
use crate::schema_diff::OperationDiff;
use crate::visitor::DiffVisitor;

pub struct RemovedOperationCheck {
    pointers: RefCell<Vec<PathPointer>>,
}

impl<'s> DiffVisitor<'s> for RemovedOperationCheck {
    fn visit_operation(
        &self,
        pointer: &PathPointer,
        _method: &str,
        operation_diff_result: &'s DiffResult<OperationDiff>,
    ) -> bool {
        if let DiffResult::Removed(_) = operation_diff_result {
            self.pointers.borrow_mut().push(pointer.clone())
        }
        false
    }
}

impl Default for RemovedOperationCheck {
    fn default() -> Self {
        Self {
            pointers: RefCell::new(vec![]),
        }
    }
}

impl<'s> ValidationIssuer<'s> for RemovedOperationCheck {
    fn id(&self) -> &'static str {
        "removed-operation"
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
