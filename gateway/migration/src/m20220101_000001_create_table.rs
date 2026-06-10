use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum User {
    Table,
    UserId,
    Username,
    Email,
    PasswordHash,
    CreatedAt,
}

#[derive(DeriveIden)]
enum MaliciousPrompts {
    Table,
    PromptId,
    UserId,
    Prompt,
    AttackType,
    RequestedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::UserId).uuid().not_null().primary_key())
                    .col(ColumnDef::new(User::Username).string().unique_key().not_null())
                    .col(ColumnDef::new(User::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MaliciousPrompts::Table)
                    .if_not_exists()
                    // Set PromptId as the Primary Key
                    .col(
                        ColumnDef::new(MaliciousPrompts::PromptId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MaliciousPrompts::UserId).uuid().not_null())
                    .col(ColumnDef::new(MaliciousPrompts::Prompt).text().not_null())
                    .col(
                        ColumnDef::new(MaliciousPrompts::AttackType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MaliciousPrompts::RequestedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    // Set up the Foreign Key relationship to User::UserId
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-malicious-prompts-user-id")
                            .from(MaliciousPrompts::Table, MaliciousPrompts::UserId)
                            .to(User::Table, User::UserId)
                            .on_delete(ForeignKeyAction::Cascade) // Deletes prompts if the user is deleted
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-malicious-prompts-user-id")
                    .table(MaliciousPrompts::Table)
                    .col(MaliciousPrompts::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(MaliciousPrompts::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }

   
}
