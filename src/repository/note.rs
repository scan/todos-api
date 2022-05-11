use crate::model::Note;

use async_trait::async_trait;
use uuid::Uuid;
use super::Repository;

#[async_trait]
pub trait NoteRepository: Send + Sync + Clone + 'static {
    async fn create_note(&self, title: &str) -> anyhow::Result<Note>;

    async fn list_notes(&self, offset: usize, limit: usize) -> anyhow::Result<Vec<Note>>;

    async fn get_note(&self, id: Uuid) -> anyhow::Result<Option<Note>>;
    async fn remove_note(&self, id: Uuid) -> anyhow::Result<()>;
}

#[async_trait]
impl NoteRepository for Repository {
    async fn create_note(&self, title: &str) -> anyhow::Result<Note> {
        todo!()
    }

    async fn list_notes(&self, offset: usize, limit: usize) -> anyhow::Result<Vec<Note>> {
        todo!()
    }

    async fn get_note(&self, id: Uuid) -> anyhow::Result<Option<Note>> {
        todo!()
    }

    async fn remove_note(&self, id: Uuid) -> anyhow::Result<()> {
        todo!()
    }
}
