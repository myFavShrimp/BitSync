use std::ops::{Deref, DerefMut};

use sqlx::{Database, Postgres, Transaction as SqlxTransaction};

pub struct Transaction(pub(crate) SqlxTransaction<'static, Postgres>);

#[derive(thiserror::Error, Debug)]
#[error("Database transaction commit failed")]
pub struct TransactionCommitError(#[from] sqlx::Error);

#[derive(thiserror::Error, Debug)]
#[error("Database transaction rollback failed")]
pub struct TransactionRollbackError(#[from] sqlx::Error);

impl Transaction {
    pub async fn commit(self) -> Result<(), TransactionCommitError> {
        Ok(self.0.commit().await?)
    }

    pub async fn rollback(self) -> Result<(), TransactionRollbackError> {
        Ok(self.0.rollback().await?)
    }
}

impl Deref for Transaction {
    type Target = <Postgres as Database>::Connection;

    fn deref(&self) -> &Self::Target {
        &*(self.0)
    }
}

impl DerefMut for Transaction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *(self.0)
    }
}
