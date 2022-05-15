use async_graphql::{Object, Context};

use crate::{repository::NoteRepository, model::Note};

use super::get_repo_from_ctx;

#[derive(Debug, Default, Clone, Copy)]
pub struct Mutation;

#[Object]
impl Mutation {
    pub async fn create_note<'ctx>(&self, ctx: &Context<'ctx>, title: String) -> anyhow::Result<Note> {
        let repo = get_repo_from_ctx(ctx)?;
        let note = repo.create_note(title.as_str()).await?;

        Ok(note)
    }

    pub async fn delete_note<'ctx>(&self, ctx: &Context<'ctx>, id: uuid::Uuid) -> anyhow::Result<bool> {
        let repo = get_repo_from_ctx(ctx)?;
        repo.remove_note(id).await?;

        Ok(true)
    }
}
