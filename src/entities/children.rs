//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "children"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub cid: i32,
    pub name: String,
    pub parent_id: i32,
    pub ability: Option<f64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Cid,
    Name,
    ParentId,
    Ability,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Cid,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AnsRecords,
    Parent,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Cid => ColumnType::Integer.def(),
            Self::Name => ColumnType::String(Some(255u32)).def(),
            Self::ParentId => ColumnType::Integer.def(),
            Self::Ability => ColumnType::Double.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AnsRecords => Entity::has_many(super::ans_records::Entity).into(),
            Self::Parent => Entity::belongs_to(super::parent::Entity)
                .from(Column::ParentId)
                .to(super::parent::Column::Pid)
                .into(),
        }
    }
}

impl Related<super::ans_records::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AnsRecords.def()
    }
}

impl Related<super::parent::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Parent.def()
    }
}

impl Related<super::quiz::Entity> for Entity {
    fn to() -> RelationDef {
        super::ans_records::Relation::Quiz.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::ans_records::Relation::Children.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}