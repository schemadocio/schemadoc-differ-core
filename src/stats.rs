use serde_json::{Map, Number, Value};
use std::iter::FromIterator;

use crate::schema_diff::{HttpSchemaDiff, MayBeRefDiff};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DiffStats {
    pub paths_all: usize,
    pub paths_added: usize,
    pub paths_updated: usize,
    pub paths_removed: usize,
}

impl DiffStats {
    pub fn as_map(&self) -> Map<String, Value> {
        Map::from_iter([
            (
                "paths_all".to_string(),
                Value::Number(Number::from(self.paths_all)),
            ),
            (
                "paths_added".to_string(),
                Value::Number(Number::from(self.paths_added)),
            ),
            (
                "paths_updated".to_string(),
                Value::Number(Number::from(self.paths_updated)),
            ),
            (
                "paths_removed".to_string(),
                Value::Number(Number::from(self.paths_removed)),
            ),
        ])
    }
}

#[allow(dead_code)]
impl HttpSchemaDiff {
    pub fn stats(&self) -> DiffStats {
        let mut paths_all = 0;
        let mut paths_added = 0;
        let mut paths_updated = 0;
        let mut paths_removed = 0;

        if let Some(paths) = self.paths.get() {
            for (_real_path, may_be_path_diff_result) in paths.iter() {
                if let Some(MayBeRefDiff::Value(path_diff_result)) = may_be_path_diff_result.get() {
                    if let Some(path) = path_diff_result.get() {
                        for operation in [
                            &path.get,
                            &path.post,
                            &path.put,
                            &path.patch,
                            &path.delete,
                            &path.head,
                            &path.options,
                            &path.trace,
                        ]
                        .iter()
                        {
                            if operation.exists() {
                                paths_all += 1;
                            }

                            if operation.is_none() || operation.is_same() {
                                continue;
                            }

                            if operation.is_added() {
                                paths_added += 1;
                            } else if operation.is_updated() {
                                paths_updated += 1;
                            } else if operation.is_removed() {
                                paths_removed += 1;
                            }
                        }
                    }
                }
            }

            DiffStats {
                paths_all,
                paths_added,
                paths_updated,
                paths_removed,
            }
        } else {
            DiffStats {
                paths_all: 0,
                paths_added: 0,
                paths_updated: 0,
                paths_removed: 0,
            }
        }
    }

    pub fn version(&self) -> String {
        match self.info.get() {
            Some(info) => match info.version.get() {
                Some(version) => version.to_owned(),
                None => "".to_owned(),
            },
            None => "".to_owned(),
        }
    }
}
