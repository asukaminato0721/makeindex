#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

pub mod qsort;
pub mod scanid;
pub mod scanst;
pub mod sortid;
// 重新导出主要函数
pub use qsort::*;
pub use scanid::*;
pub use scanst::*;
pub use sortid::*;
