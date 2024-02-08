//! GenericResult

use crate::Row;

#[derive(Debug)]
pub enum GenericResult {
    Row(Row),
    Command(u64, String),
}
