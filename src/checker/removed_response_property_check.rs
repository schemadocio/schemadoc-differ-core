use std::cell::RefCell;

use crate::checker::{ValidationIssue, ValidationIssuer};
use crate::core::{DiffResult, MapDiff};
use crate::path_pointer::PathPointer;
use crate::schema_diff::{MayBeRefDiff, MediaTypeDiff, OperationDiff, ResponseDiff, SchemaDiff};
use crate::visitor::DiffVisitor;

pub struct RemovedResponsePropertyCheck {
    pointers: RefCell<Vec<PathPointer>>,
}

impl<'s> DiffVisitor<'s> for RemovedResponsePropertyCheck {
    fn visit_operation(
        &self,
        pointer: &PathPointer,
        _: &str,
        _: &'s DiffResult<OperationDiff>,
    ) -> bool {
        !pointer.is_removed()
    }

    fn visit_responses(
        &self,
        pointer: &PathPointer,
        _: &'s DiffResult<MapDiff<MayBeRefDiff<ResponseDiff>>>,
    ) -> bool {
        !pointer.is_removed()
    }

    fn visit_media_types(
        &self,
        pointer: &PathPointer,
        _: &'s DiffResult<MapDiff<MediaTypeDiff>>,
    ) -> bool {
        !pointer.is_removed()
    }

    fn visit_media_type(&self, pointer: &PathPointer, _: &'s DiffResult<MediaTypeDiff>) -> bool {
        !pointer.is_removed()
    }

    fn visit_schema(
        &self,
        pointer: &PathPointer,
        schema_diff_result: &'s DiffResult<SchemaDiff>,
    ) -> bool {
        if pointer.parent().is_removed() {
            return false;
        }

        if let DiffResult::Removed(_) = schema_diff_result {
            self.pointers.borrow_mut().push(pointer.clone())
        }

        true
    }
}

impl Default for RemovedResponsePropertyCheck {
    fn default() -> Self {
        Self {
            pointers: RefCell::new(vec![]),
        }
    }
}

impl<'s> ValidationIssuer<'s> for RemovedResponsePropertyCheck {
    fn id(&self) -> &'static str {
        "remove-response-property"
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
