use shared::error::Result;

pub(crate) trait ExtractConn {
    fn extract_conn(&self) -> Result<infrastructure::Connection>;
}

impl<'ctx> ExtractConn for async_graphql::Context<'ctx> {
    fn extract_conn(&self) -> Result<infrastructure::Connection> {
        // SAFETY: this should always work
        let pool = self.data::<infrastructure::ConnectionPool>()?;
        Ok(pool.pool.get()?)
    }
}
