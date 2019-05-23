use crate::types::{
    expression::Expr,
    index::{FuncIdx, MemIdx, TableIdx},
};

// TODO: ConstExpr
#[derive(Debug, Clone, PartialEq)]
pub struct DataSegment {
    tableidx: TableIdx,
    offset: Expr,
    init: Vec<FuncIdx>,
}

// TODO: ConstExpr
#[derive(Debug, Clone, PartialEq)]
pub struct ElementSegment {
    data: MemIdx,
    offset: Expr,
    init: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Memory {
    memtype: MemType,
    memory: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableType {
    kind: ElemType,
    limits: Limit,
}

impl TableType {
    pub fn new(kind: ElemType, limits: Limit) -> TableType {
        Self { kind, limits }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    kind: ElemType,
    limits: Limit,
    // elems: Vec<
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElemType {
    FuncRef,
}

pub type MemType = Limit;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Limit {
    min: u32,
    max: Option<u32>,
}

impl Limit {
    pub fn new(min: u32, max: Option<u32>) -> Self {
        Limit { min, max }
    }
}