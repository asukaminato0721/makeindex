use crate::page::{PageFields, PageType};

pub const FIELD_MAX: usize = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Group {
    Numeric(i32),
    Symbol,
    Alpha,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeOp {
    None,
    Open,
    Close,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub sort_keys: [String; FIELD_MAX],
    pub actual_keys: [String; FIELD_MAX],
    pub group: Group,
    pub literal_page: String,
    pub page_fields: PageFields,
    pub page_type: PageType,
    pub encap: String,
    pub range_op: RangeOp,
    pub file: String,
    pub line: usize,
}

impl Entry {
    pub fn new(file: String, line: usize) -> Self {
        Self {
            sort_keys: Default::default(),
            actual_keys: Default::default(),
            group: Group::Alpha,
            literal_page: String::new(),
            page_fields: PageFields::default(),
            page_type: PageType::Empty,
            encap: String::new(),
            range_op: RangeOp::None,
            file,
            line,
        }
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self::new(String::new(), 0)
    }
}
