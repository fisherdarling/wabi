use crate::types::{
    index::{FuncIdx, GlobalIdx, MemIdx, TableIdx, TypeIdx},
    memory::{ElemType, MemType},
    types::GlobalType,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    module: String,
    name: String,
    desc: ImportDesc,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportDesc {
    Func(TypeIdx),
    Table(ElemType),
    Mem(MemType),
    Global(GlobalType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Export {
    name: String,
    desc: ExportDesc,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}