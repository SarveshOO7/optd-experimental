use super::cascades_group::CascadesGroup;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum PhysicalExpression {
    Table,
    Id,
    Fingerprint,
    Data,
    GroupId,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241026_000001_create_physical_expression_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // TODO add expression / root operator variant identifier
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PhysicalExpression::Table)
                    .if_not_exists()
                    .col(pk_auto(PhysicalExpression::Id))
                    .col(big_unsigned(PhysicalExpression::Fingerprint))
                    .col(json(PhysicalExpression::Data))
                    .col(integer(PhysicalExpression::GroupId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-physical_expression-group_id")
                            .from(PhysicalExpression::Table, PhysicalExpression::GroupId)
                            .to(CascadesGroup::Table, CascadesGroup::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PhysicalExpression::Table).to_owned())
            .await
    }
}