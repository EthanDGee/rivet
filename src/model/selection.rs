use super::sql_session::SqlSession;
use color_eyre::eyre::Result;

pub const SELECT_BATCH_SIZE: usize = 500;

pub struct Selection<'a> {
    session: &'a SqlSession,
    query: String,
    selection_size: u32,
    page_index: usize,
}

impl<'a> Selection<'a> {
    pub fn new(session: &'a SqlSession, query: String) -> Result<Self> {
        let selection_size = session.get_selection_size(query.clone())?;

        Ok(Selection {
            session,
            query,
            selection_size,
            page_index: 0,
        })
    }
}
