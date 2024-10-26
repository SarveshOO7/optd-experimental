//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "fingerprint")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub inner: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::logical_expression::Entity")]
    LogicalExpression,
    #[sea_orm(has_many = "super::physical_expression::Entity")]
    PhysicalExpression,
}

impl Related<super::logical_expression::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LogicalExpression.def()
    }
}

impl Related<super::physical_expression::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PhysicalExpression.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
