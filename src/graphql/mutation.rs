use async_graphql::{Context, InputObject, Object};

use crate::{model::Note, repository::NoteRepository};

use super::get_repo_from_ctx;

#[derive(Debug, Default, Clone, Copy)]
pub struct Mutation;

#[derive(Debug, Default, Clone, PartialEq, Eq, InputObject)]
pub struct EditNoteInput {
    id: uuid::Uuid,
    title: String,
    contents: Option<String>,
}

#[Object]
impl Mutation {
    pub async fn create_note<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        title: String,
    ) -> anyhow::Result<Note> {
        let repo = get_repo_from_ctx(ctx)?;
        let note = repo.create_note(title.as_str()).await?;

        Ok(note)
    }

    pub async fn delete_note<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: uuid::Uuid,
    ) -> anyhow::Result<bool> {
        let repo = get_repo_from_ctx(ctx)?;
        repo.remove_note(id).await?;

        Ok(true)
    }

    pub async fn edit_note<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        note: EditNoteInput,
    ) -> anyhow::Result<bool> {
        let repo = get_repo_from_ctx(ctx)?;
        repo.update_note(note.id, note.title, note.contents).await?;

        Ok(true)
    }
}
