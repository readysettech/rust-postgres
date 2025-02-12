use crate::connection::ConnectionRef;
use fallible_iterator::FallibleIterator;
use futures_util::StreamExt;
use std::pin::Pin;
use tokio_postgres::{Error, GenericResult, Row, RowStream};

/// The iterator returned by `query_raw`.
pub struct RowIter<'a> {
    connection: ConnectionRef<'a>,
    it: Pin<Box<RowStream>>,
}

impl<'a> RowIter<'a> {
    pub(crate) fn new(connection: ConnectionRef<'a>, stream: RowStream) -> RowIter<'a> {
        RowIter {
            connection,
            it: Box::pin(stream),
        }
    }

    /// Returns the number of rows affected by the query.
    ///
    /// This function will return `None` until the iterator has been exhausted.
    pub fn rows_affected(&self) -> Option<u64> {
        self.it.rows_affected()
    }
}

impl FallibleIterator for RowIter<'_> {
    type Item = Row;
    type Error = Error;

    fn next(&mut self) -> Result<Option<Row>, Error> {
        let it = &mut self.it;
        self.connection.block_on(async {
            loop {
                match it.next().await.transpose()? {
                    Some(GenericResult::Row(row)) => return Ok(Some(row)),
                    Some(GenericResult::Command(_, _)) => {}
                    None => return Ok(None),
                }
            }
        })
    }
}
