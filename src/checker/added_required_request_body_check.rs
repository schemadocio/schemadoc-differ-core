use std::cell::RefCell;

use crate::checker::{ValidationIssue, ValidationIssuer};
use crate::core::DiffResult;
use crate::path_pointer::PathPointer;
use crate::schema_diff::{OperationDiff, RequestBodyDiff};

use crate::visitor::DiffVisitor;

pub struct AddedRequiredRequestBodyCheck {
    pointers: RefCell<Vec<PathPointer>>,
}

impl<'s> DiffVisitor<'s> for AddedRequiredRequestBodyCheck {
    fn visit_operation(
        &self,
        pointer: &PathPointer,
        _: &str,
        _: &'s DiffResult<OperationDiff>,
    ) -> bool {
        // Only check already existed operations
        pointer.is_updated()
    }

    fn visit_request_body(
        &self,
        pointer: &PathPointer,
        request_body_diff_result: &'s DiffResult<RequestBodyDiff>,
    ) -> bool {
        if !pointer.is_upserted() {
            return false;
        }

        let Some(request_body_diff) = request_body_diff_result.get() else {
            return false;
        };

        if !request_body_diff.required.is_upserted() {
            return false;
        }

        if let Some(required) = request_body_diff.required.get() {
            if *required {
                self.pointers.borrow_mut().push(pointer.clone())
            }
        }

        false
    }
}

impl Default for AddedRequiredRequestBodyCheck {
    fn default() -> Self {
        AddedRequiredRequestBodyCheck {
            pointers: RefCell::new(vec![]),
        }
    }
}

impl<'s> ValidationIssuer<'s> for AddedRequiredRequestBodyCheck {
    fn id(&self) -> &'static str {
        "added-required-request-body"
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
