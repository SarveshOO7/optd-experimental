//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "constraint")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub constraint_type: i32,
    pub table_id: i32,
    pub index_id: i32,
    pub foreign_ref_id: i32,
    pub check_src: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::constraint_attribute_junction::Entity")]
    ConstraintAttributeJunction,
    #[sea_orm(has_many = "super::foreign_constraint_ref_attribute_junction::Entity")]
    ForeignConstraintRefAttributeJunction,
    #[sea_orm(
        belongs_to = "super::index::Entity",
        from = "Column::IndexId",
        to = "super::index::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Index,
    #[sea_orm(
        belongs_to = "super::table_metadata::Entity",
        from = "Column::ForeignRefId",
        to = "super::table_metadata::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    TableMetadata2,
    #[sea_orm(
        belongs_to = "super::table_metadata::Entity",
        from = "Column::TableId",
        to = "super::table_metadata::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    TableMetadata1,
}

impl Related<super::constraint_attribute_junction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConstraintAttributeJunction.def()
    }
}

impl Related<super::foreign_constraint_ref_attribute_junction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ForeignConstraintRefAttributeJunction.def()
    }
}

impl Related<super::index::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Index.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}