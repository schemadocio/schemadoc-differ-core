use std::cell::RefCell;

use crate::core::{DiffResult, MapDiff, VecDiff};
use crate::path_pointer::PathPointer;

use crate::checker::{ValidationIssue, ValidationIssuer};
use crate::schema_diff::{
    MayBeRefDiff, MediaTypeDiff, OperationDiff, ParameterDiff, RequestBodyDiff, ResponseDiff,
    SchemaDiff,
};
use crate::visitor::DiffVisitor;

pub struct SchemaEnumValueRemovedCheck {
    pointers: RefCell<Vec<PathPointer>>,
}

impl<'s> DiffVisitor<'s> for SchemaEnumValueRemovedCheck {
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
        pointer: &PathPointer,
        _: &'s DiffResult<MapDiff<MediaTypeDiff>>,
    ) -> bool {
        pointer.is_updated()
    }

    fn visit_media_type(&self, pointer: &PathPointer, _: &'s DiffResult<MediaTypeDiff>) -> bool {
        pointer.is_updated()
    }

    fn visit_parameters(
        &self,
        pointer: &PathPointer,
        _: &'s DiffResult<VecDiff<MayBeRefDiff<ParameterDiff>>>,
    ) -> bool {
        pointer.is_updated()
    }

    fn visit_parameter(&self, pointer: &PathPointer, _: &'s DiffResult<ParameterDiff>) -> bool {
        pointer.is_updated()
    }

    fn visit_schema(
        &self,
        pointer: &PathPointer,
        schema_diff_result: &'s DiffResult<SchemaDiff>,
    ) -> bool {
        if !pointer.is_updated() {
            return false;
        }

        let Some(schema) = schema_diff_result.get() else {
            return false;
        };

        if schema.r#enum.is_updated() {
            let has_removed = match schema.r#enum.get() {
                None => false,
                Some(values) => values.iter().any(|v| v.is_removed()),
            };
            if has_removed {
                self.pointers.borrow_mut().push(pointer.clone())
            }
        }

        true
    }
}

impl Default for SchemaEnumValueRemovedCheck {
    fn default() -> Self {
        Self {
            pointers: RefCell::new(vec![]),
        }
    }
}

impl<'s> ValidationIssuer<'s> for SchemaEnumValueRemovedCheck {
    fn id(&self) -> &'static str {
        "schema-enum-value-removed"
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
