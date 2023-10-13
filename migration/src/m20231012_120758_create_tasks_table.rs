use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BeerBrand::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BeerBrand::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BeerBrand::Name).string().not_null())
                    .col(ColumnDef::new(BeerBrand::City).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BeerBrand::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BeerBrand {
    Table,
    Id,
    Name,
    City,
}

// #[derive(DeriveIden)]
// enum Post {
//     Table,
//     Id,
//     Title,
//     Text,
// }
