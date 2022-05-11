use crate::model::Note;

use super::Repository;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait NoteRepository: Send + Sync + Clone + 'static {
    async fn create_note(&self, title: &str) -> anyhow::Result<Note>;

    async fn list_notes(&self, offset: u32, limit: u32) -> anyhow::Result<Vec<Note>>;

    async fn get_note(&self, id: Uuid) -> anyhow::Result<Option<Note>>;
    async fn remove_note(&self, id: Uuid) -> anyhow::Result<()>;
}

#[async_trait]
impl NoteRepository for Repository {
    async fn create_note(&self, title: &str) -> anyhow::Result<Note> {
        let note = Note::new(title);

        sqlx::query!(
            r#"
INSERT INTO notes ( id, title, created_at ) VALUES ( ?, ?, ? )
            "#,
            note.id,
            note.title,
            note.created_at
        )
        .execute(&self.0)
        .await?;

        Ok(note)
    }

    async fn list_notes(&self, offset: u32, limit: u32) -> anyhow::Result<Vec<Note>> {
        let notes = sqlx::query_as!(
            Note,
            r#"
SELECT
    id as "id: _",
    title,
    contents,
    created_at as "created_at: _",
    modified_at as "modified_at: _"
FROM notes
ORDER BY created_at DESC
LIMIT ?
OFFSET ?
       "#,
            limit,
            offset,
        )
        .fetch_all(&self.0)
        .await?;

        Ok(notes)
    }

    async fn get_note(&self, id: Uuid) -> anyhow::Result<Option<Note>> {
        let res = sqlx::query_as!(
            Note,
            r#"
SELECT
    id as "id: _",
    title,
    contents,
    created_at as "created_at: _",
    modified_at as "modified_at: _"
FROM notes
WHERE id = ?
       "#,
            id,
        )
        .fetch_one(&self.0)
        .await;

        match res {
            Ok(note) => Ok(Some(note)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => anyhow::bail!(err),
        }
    }

    async fn remove_note(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
DELETE FROM notes WHERE id = ?
            "#,
            id
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }
}
