use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, Object,
};

use crate::{model::Note, repository::NoteRepository};

use super::get_repo_from_ctx;

#[derive(Debug, Default, Clone, Copy)]
pub struct Query;

#[Object]
impl Query {
    pub async fn version<'ctx>(&self, _ctx: &Context<'ctx>) -> &str {
        "0.0.1"
    }

    pub async fn note<'ctx>(&self, ctx: &Context<'ctx>, id: uuid::Uuid) -> anyhow::Result<Option<Note>> {
        let repo = get_repo_from_ctx(ctx)?;
        let note = repo.get_note(id).await?;

        Ok(note)
    }

    pub async fn notes<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> anyhow::Result<Connection<usize, Note>> {
        let repo = get_repo_from_ctx(ctx)?;

        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = after.map(|after| after + 1).unwrap_or(0);
                let mut end = before.unwrap_or(10000);

                if let Some(first) = first {
                    end = (start + first).min(end);
                }
                if let Some(last) = last {
                    start = if last > end - start { end } else { end - last };
                }

                let notes = repo.list_notes(start as u32, (end - start) as u32).await?;

                let mut connection = Connection::new(start > 0, end < 10000);

                connection.append(
                    (notes)
                        .iter()
                        .enumerate()
                        .map(|(i, n)| Edge::with_additional_fields(i, n.clone(), EmptyFields)),
                );

                Ok::<_, anyhow::Error>(connection)
            },
        )
        .await
        .map_err(|err| anyhow::format_err!(err.message))
    }
}
