use super::sql_session::SqlSession;
use color_eyre::eyre::{Result, eyre};
use std::str::FromStr;
use substring::Substring;

pub const SELECT_BATCH_SIZE: u32 = 500;

pub struct Selection<'a> {
    session: &'a SqlSession,
    query: String,
    selection_size: u32,
    page_index: u32,
    page_count: u32,
}

impl<'a> Selection<'a> {
    pub fn new(session: &'a SqlSession, query: String) -> Result<Self> {
        let selection_size = session.get_selection_size(query.clone())?;

        let page_count = selection_size / SELECT_BATCH_SIZE + 1;

        Ok(Selection {
            session,
            query,
            selection_size,
            page_index: 0,
            page_count,
        })
    }

    fn get_page(&mut self, num: u32) -> Result<Vec<Vec<String>>> {
        if num > self.page_count {
            return Err(eyre!("Requested page exceeds page count."));
        }

        // limit the selection_size for the last page
        let mut query_limit: u32;

        if num == self.page_index {
            query_limit = self.selection_size - ((self.page_count - 1) * SELECT_BATCH_SIZE);
        } else {
            query_limit = SELECT_BATCH_SIZE;
        }

        let limit_key_word = "LIMIT";

        // if there is an existing limit remove it and modify query to use new one
        let mut limit_index = 0;

        for i in 0..(self.query.len() - limit_key_word.len()) {
            if self
                .query
                .substring(i, limit_key_word.len())
                .eq_ignore_ascii_case(limit_key_word)
            {
                limit_index = i + limit_key_word.len();
            }
        }

        if limit_index == 0 {
            limit_index = self.query.len() - 1;
        }

        // if we have an existing offset in the user query remove it and extract the number
        // and add it to our offset
        let mut offset = SELECT_BATCH_SIZE * num;

        let offset_key_word = "LIMIT";

        let mut offset_index = 0;

        for i in 0..(self.query.len() - offset_key_word.len()) {
            if self
                .query
                .substring(i, offset_key_word.len())
                .eq_ignore_ascii_case(offset_key_word)
            {
                offset_index = i + offset_key_word.len();
            }
        }

        if offset_index != 0 {
            let mut digits_length = 1;

            while let Some(c) = self.query.chars().nth(offset_index + digits_length) {
                if c.is_ascii_digit() {
                    digits_length += 1;
                } else {
                    break;
                }
            }

            let digits = self
                .query
                .substring(offset_index + 1, offset_index + digits_length);

            match u32::from_str(digits) {
                Ok(added_offset) => {
                    offset += added_offset;
                }
                Err(e) => {
                    return Err(eyre!(
                        "Failed converting to an int while attempt to extract user defined offset: {}",
                        e
                    ));
                }
            }
        }

        self.page_index = num;

        let modified_query = format!(
            "{} LIMIT {} OFFSET {}",
            self.query.substring(0, limit_index),
            query_limit,
            offset
        );

        self.session.select(&modified_query)
    }
}
