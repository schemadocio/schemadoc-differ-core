use std::cell::RefCell;

use crate::checker::{ValidationIssue, ValidationIssuer};
use crate::core::{DiffResult, MapDiff};
use crate::path_pointer::PathPointer;
use crate::schema_diff::{MediaTypeDiff, RequestBodyDiff, SchemaDiff};

use crate::visitor::DiffVisitor;

pub struct AddedRequiredBodyPropertyCheck {
    pointers: RefCell<Vec<PathPointer>>,
}

impl<'s> DiffVisitor<'s> for AddedRequiredBodyPropertyCheck {
    fn visit_request_body(
        &self,
        pointer: &PathPointer,
        request_body_diff_result: &'s DiffResult<RequestBodyDiff>,
    ) -> bool {
        // Continue only if request body is required
        if let Some(request_body_diff) = request_body_diff_result.get() {
            request_body_diff
                .required
                .get()
                .map(|v| *v && pointer.is_updated())
                .unwrap_or(false)
        } else {
            false
        }
    }

    fn visit_media_types(
        &self,
        pointer: &PathPointer,
        _: &'s DiffResult<MapDiff<MediaTypeDiff>>,
    ) -> bool {
        pointer.is_updated()
    }

    fn visit_media_type(&self, pointer: &PathPointer, _: &'s DiffResult<MediaTypeDiff>) -> bool {
        pointer.is_updated()
    }

    fn visit_schema(
        &self,
        pointer: &PathPointer,
        _schema_diff_result: &'s DiffResult<SchemaDiff>,
    ) -> bool {
        if pointer.is_added() {
            self.pointers.borrow_mut().push(pointer.clone());
            return false;
        }

        !(pointer.is_removed() || pointer.is_same())
    }
}

impl Default for AddedRequiredBodyPropertyCheck {
    fn default() -> Self {
        AddedRequiredBodyPropertyCheck {
            pointers: RefCell::new(vec![]),
        }
    }
}

impl<'s> ValidationIssuer<'s> for AddedRequiredBodyPropertyCheck {
    fn id(&self) -> &'static str {
        "added-required-body-property"
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
