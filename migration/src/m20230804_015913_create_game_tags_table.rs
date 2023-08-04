use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GameTags::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GameTags::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(GameTags::GameId).integer().not_null())
                    .col(ColumnDef::new(GameTags::TagId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(GameTags::Table, GameTags::GameId)
                            .to(Games::Table, Games::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GameTags::Table, GameTags::TagId)
                            .to(Tags::Table, Tags::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GameTags::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum GameTags {
    Table,
    Id,
    GameId,
    TagId,
}

#[derive(Iden)]
enum Games {
    Table,
    Id,
}

#[derive(Iden)]
enum Tags {
    Table,
    Id,
}
