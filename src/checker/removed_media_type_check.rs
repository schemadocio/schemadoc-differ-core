use std::cell::RefCell;

use crate::checker::{ValidationIssue, ValidationIssuer};
use crate::core::{DiffResult, MapDiff};
use crate::path_pointer::PathPointer;
use crate::schema_diff::{
    MayBeRefDiff, MediaTypeDiff, OperationDiff, RequestBodyDiff, ResponseDiff,
};

use crate::visitor::DiffVisitor;

pub struct RemovedMediaTypeCheck {
    pointers: RefCell<Vec<PathPointer>>,
}

impl<'s> DiffVisitor<'s> for RemovedMediaTypeCheck {
    fn visit_operation(
        &self,
        pointer: &PathPointer,
        _: &str,
        _: &'s DiffResult<OperationDiff>,
    ) -> bool {
        pointer.is_updated()
    }

    fn visit_request_body(
        &self,
        pointer: &PathPointer,
        _: &'s DiffResult<RequestBodyDiff>,
    ) -> bool {
        pointer.is_updated()
    }

    fn visit_responses(
        &self,
        pointer: &PathPointer,
        _: &'s DiffResult<MapDiff<MayBeRefDiff<ResponseDiff>>>,
    ) -> bool {
        pointer.is_updated()
    }

    fn visit_media_types(
        &self,
        _: &PathPointer,
        _: &'s DiffResult<MapDiff<MediaTypeDiff>>,
    ) -> bool {
        true
    }

    fn visit_media_type(&self, pointer: &PathPointer, _: &'s DiffResult<MediaTypeDiff>) -> bool {
        if pointer.is_removed() {
            self.pointers.borrow_mut().push(pointer.clone());
            return false;
        }

        false
    }
}

impl Default for RemovedMediaTypeCheck {
    fn default() -> Self {
        RemovedMediaTypeCheck {
            pointers: RefCell::new(vec![]),
        }
    }
}

impl<'s> ValidationIssuer<'s> for RemovedMediaTypeCheck {
    fn id(&self) -> &'static str {
        "removed-media-type-check"
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
