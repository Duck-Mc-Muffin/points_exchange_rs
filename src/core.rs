use clap::ValueEnum;
use data_sqlite::DataSQLite;
use persistance_layer::*;
use tokio::{spawn, task::JoinHandle};

mod data_sqlite;
mod persistance_layer;

pub type UserID = DbPk;
pub type TokenID = DbPk;
pub type TokenAmount = i32;

/// Order output in ascending (asc) or descending (desc) order
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Order
{
    Asc,
    Desc,
}

// Various ordering "by column" options (depending on the query)
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OrderBySenderOrAmount
{
    Sender,
    Amount,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OrderByTokenOrSenderOrAmount
{
    Token,
    Sender,
    Amount,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OrderByReceiverOrSenderOrAmount
{
    Receiver,
    Sender,
    Amount,
}

// Query options
pub enum UserQueryModeStrict<'a>
{
    ById(UserID),
    ByName(&'a str),
}
pub enum UserQueryModeWithCreation<'a>
{
    ById(UserID),
    ByName(&'a str),
    ByNameOrCreate(&'a str),
}
pub enum TokenQueryModeStrict<'a>
{
    ById(TokenID),
    ByName(&'a str),
}
pub enum TokenQueryModeWithCreation<'a>
{
    ById(TokenID),
    ByName(&'a str),
    ByNameOrCreate(&'a str),
}

// =================================================================================================================
pub struct Core
{
    db: DbImplementors,
}

#[derive(Debug)]
struct RelativeUserAmountEntry
{
    pub receiver:         User,
    pub amount_by_sender: Vec<RelativeUserTokenAmountEntry>,
}

#[derive(Debug)]
struct RelativeTokenAmountEntry
{
    pub token:            Token,
    pub amount_by_sender: Vec<RelativeUserTokenAmountEntry>,
}

#[derive(Debug)]
struct RelativeUserTokenAmountEntry
{
    pub sender: User,
    pub amount: TokenAmount,
}

// TODO {CustomErrorType}
#[derive(Debug)]
enum DeferredUserId
{
    Immediate(DbPk),
    Deferred(JoinHandle<Result<User, sqlx::Error>>),
}

// TODO {CustomErrorType}
#[derive(Debug)]
enum DeferredTokenId
{
    Immediate(DbPk),
    Deferred(JoinHandle<Result<Token, sqlx::Error>>),
}

impl Core
{
    // TODO: Make PersistanceLayer configurable
    pub async fn new() -> Core
    {
        Core {
            db: DbImplementors::SQLite(DataSQLite::new().await), // TODO
        }
    }

    // =========================================== Workaround helper methods ===========================================
    // WORKAROUND {AsyncPersistanceLayerTraitObject}:
    //      Since async in trait objects is not a thing yet in rust, these helper methods will hide the enum for now.
    async fn db_create_user(&self, name: &str) -> Result<User, sqlx::Error>
    {
        match &self.db
        {
            DbImplementors::SQLite(db) => db.create_user(name).await,
        }
    }
    async fn db_get_all_users(&self) -> Result<Vec<User>, sqlx::Error>
    {
        match &self.db
        {
            DbImplementors::SQLite(db) => db.get_all_users().await,
        }
    }
    async fn db_query_user(&self, name: &str) -> Result<Vec<User>, sqlx::Error>
    {
        match &self.db
        {
            DbImplementors::SQLite(db) => db.query_user(name).await,
        }
    }
    async fn db_create_token(&self, name: &str) -> Result<Token, sqlx::Error>
    {
        match &self.db
        {
            DbImplementors::SQLite(db) => db.create_token(name).await,
        }
    }
    async fn db_query_token(&self, name: &str) -> Result<Vec<Token>, sqlx::Error>
    {
        match &self.db
        {
            DbImplementors::SQLite(db) => db.query_token(name).await,
        }
    }
    async fn db_get_all_tokens(&self) -> Result<Vec<Token>, sqlx::Error>
    {
        match &self.db
        {
            DbImplementors::SQLite(db) => db.get_all_tokens().await,
        }
    }
    async fn db_get_current_total(
        &self,
        sender: UserQueryModeStrict<'_>,
        receiver: UserQueryModeStrict<'_>,
        token: TokenQueryModeStrict<'_>,
    ) -> Result<Option<TokenAmount>, sqlx::Error>
    {
        match &self.db
        {
            DbImplementors::SQLite(db) => db.get_current_total(sender, receiver, token).await,
        }
    }
    async fn db_transaction(
        &self,
        sender: UserQueryModeStrict<'_>,
        receiver: UserQueryModeStrict<'_>,
        token: TokenQueryModeStrict<'_>,
        amount: TokenAmount,
    ) -> Result<TokenAmount, sqlx::Error>
    {
        match &self.db
        {
            DbImplementors::SQLite(db) => db.transaction(sender, receiver, token, amount).await,
        }
    }

    // ================================================ User Management ================================================
    pub async fn create_user(&self, name: &str) -> Result<User, sqlx::Error>
    {
        self.db_create_user(name).await
    }

    pub async fn query_all_users(&self) -> Result<Vec<User>, sqlx::Error>
    {
        self.db_get_all_users().await
    }

    pub async fn query_user(&self, name: &str) -> Result<Vec<User>, sqlx::Error>
    {
        self.db_query_user(name).await
    }

    // ================================================ Token Management ================================================
    pub async fn create_token(&self, name: &str) -> Result<Token, sqlx::Error>
    {
        self.db_create_token(name).await
    }

    pub async fn query_all_tokens(&self) -> Result<Vec<Token>, sqlx::Error>
    {
        self.db_get_all_tokens().await
    }

    pub async fn query_token(&self, name: &str) -> Result<Vec<Token>, sqlx::Error>
    {
        self.db_query_token(name).await
    }

    // ================================================ Transactions ================================================
    // TODO {CustomErrorType}: Define error type

    async fn get_or_create_user_by_name(&self, name: &str, create_if_missing: bool) -> Result<User, sqlx::Error>
    {
        match self.db_query_user(name).await?.into_iter().next()
        {
            Some(user) => Ok(user),
            None =>
            {
                if create_if_missing
                {
                    Ok(self.db_create_user(name).await?)
                }
                else
                {
                    Err(sqlx::Error::RowNotFound)
                }
            }
        }
    }

    async fn get_or_create_token_by_name(&self, name: &str, create_if_missing: bool) -> Result<Token, sqlx::Error>
    {
        match self.db_query_token(name).await?.into_iter().next()
        {
            Some(token) => Ok(token),
            None =>
            {
                if create_if_missing
                {
                    Ok(self.db_create_token(name).await?)
                }
                else
                {
                    Err(sqlx::Error::RowNotFound)
                }
            }
        }
    }

    /// Will execute a transaction and, if necessary, will create all users and tokens on the fly (opt-in).
    pub async fn transaction(
        &self,
        _sender: UserQueryModeWithCreation<'_>,
        _receiver: UserQueryModeWithCreation<'_>,
        _token: TokenQueryModeWithCreation<'_>,
        _amount: TokenAmount,
    ) -> Result<TokenAmount, sqlx::Error>
    {
        // TODO: Refactor DB-Layer state to support ASYNC
        // This code block is just a simple test, that needs to compile
        let name = "test";
        let handle = spawn(self.get_or_create_user_by_name(name, false));
        let _ = handle.await;

        // TODO: CONTINUE HERE (compiler error below)
        // // Spawn all necessary SQL query tasks
        // let deferred_sender_id = match sender
        // {
        //     UserQueryModeWithCreation::ById(id) => DeferredUserId::Immediate(id),
        //     UserQueryModeWithCreation::ByName(name) => DeferredUserId::Deferred(spawn(self.get_or_create_user_by_name(name, false))),
        //     UserQueryModeWithCreation::ByNameOrCreate(name) => DeferredUserId::Deferred(spawn(self.get_or_create_user_by_name(name, true))),
        // };
        // let deferred_receiver_id = match receiver
        // {
        //     UserQueryModeWithCreation::ById(id) => DeferredUserId::Immediate(id),
        //     UserQueryModeWithCreation::ByName(name) => DeferredUserId::Deferred(spawn(self.get_or_create_user_by_name(name, false))),
        //     UserQueryModeWithCreation::ByNameOrCreate(name) => DeferredUserId::Deferred(spawn(self.get_or_create_user_by_name(name, true))),
        // };
        // let deferred_token_id = match token
        // {
        //     TokenQueryModeWithCreation::ById(id) => DeferredTokenId::Immediate(id),
        //     TokenQueryModeWithCreation::ByName(name) => DeferredTokenId::Deferred(spawn(self.get_or_create_token_by_name(name, false))),
        //     TokenQueryModeWithCreation::ByNameOrCreate(name) =>
        //     {
        //         DeferredTokenId::Deferred(spawn(self.get_or_create_token_by_name(name, true)))
        //     }
        // };
        //
        // // Join all queries into current async task
        // let sender_id = match deferred_sender_id
        // {
        //     DeferredUserId::Immediate(id) => id,
        //     DeferredUserId::Deferred(handle) => handle.await.unwrap()?.id,
        // };
        // let receiver_id = match deferred_receiver_id
        // {
        //     DeferredUserId::Immediate(id) => id,
        //     DeferredUserId::Deferred(handle) => handle.await.unwrap()?.id,
        // };
        // let token_id = match deferred_token_id
        // {
        //     DeferredTokenId::Immediate(id) => id,
        //     DeferredTokenId::Deferred(handle) => handle.await.unwrap()?.id,
        // };
        //
        // // Retain previous total before the actual transaction takes place.
        // // Reason:
        // //      Doing another query after the transaction, to check the new total, could include other transactions
        // //      that might have happend in the brief time window while this function executes. This would be unintuitive.
        // //      So the return value should always be the "last known total" + "the transaction amount."
        // //      Even if it's not the most recent total.
        // let previous_total = self
        //     .db_get_current_total(
        //         UserQueryModeStrict::ById(sender_id),
        //         UserQueryModeStrict::ById(receiver_id),
        //         TokenQueryModeStrict::ById(token_id),
        //     )
        //     .await?
        //     .unwrap_or_default();
        //
        // // Transaction
        // let transaction_amount = self
        //     .db_transaction(
        //         UserQueryModeStrict::ById(sender_id),
        //         UserQueryModeStrict::ById(receiver_id),
        //         TokenQueryModeStrict::ById(token_id),
        //         amount,
        //     )
        //     .await?;
        //
        // // Return the the new total, including the new transaction
        // Ok(previous_total + transaction_amount)
        Ok(1337)
    }

    // ================================================ List Tokens =================================================
    pub async fn list_user_token(
        &mut self,
        receiver: UserQueryModeStrict<'_>,
        token: TokenQueryModeStrict<'_>,
        order: Order,
        order_by: Option<OrderBySenderOrAmount>,
    ) -> Result<Vec<RelativeUserTokenAmountEntry>, sqlx::Error>
    {
        let _receiver_id: DbPk = match receiver
        {
            UserQueryModeStrict::ById(id) => id,
            UserQueryModeStrict::ByName(name) =>
            {
                self.db_query_user(name)
                    .await?
                    .into_iter()
                    .next()
                    .ok_or(sqlx::Error::RowNotFound)?
                    .id
            }
        };

        let _token_id: DbPk = match token
        {
            TokenQueryModeStrict::ById(id) => id,
            TokenQueryModeStrict::ByName(name) => self.query_token(name).await?.into_iter().next().ok_or(sqlx::Error::RowNotFound)?.id,
        };

        self.db_list_user_token(receiver, token, order, order_by)
    }

    pub async fn list_tokens_by_user(
        &mut self,
        receiver: UserQueryModeStrict<'_>,
        order: Order,
        order_by: Option<OrderByTokenOrSenderOrAmount>,
    ) -> Result<Vec<RelativeTokenAmountEntry>, sqlx::Error>
    {
        let _receiver_id: UserID = match receiver
        {
            // TODO: pop-ing the >last< match is not optimal
            UserQueryModeStrict::ById(id) => id,
            UserQueryModeStrict::ByName(name) =>
            {
                self.db_query_user(name)
                    .await?
                    .into_iter()
                    .next()
                    .ok_or(sqlx::Error::RowNotFound)?
                    .id
            }
        };

        self.db_list_tokens_by_user(receiver, order, order_by)
    }

    pub async fn list_users_by_token(
        &mut self,
        token: TokenQueryModeStrict<'_>,
        order: Order,
        order_by: Option<OrderByReceiverOrSenderOrAmount>,
    ) -> Result<Vec<RelativeUserAmountEntry>, sqlx::Error>
    {
        let _token_id: UserID = match token
        {
            // TODO: pop-ing the >last< match is not optimal
            TokenQueryModeStrict::ById(id) => id,
            TokenQueryModeStrict::ByName(name) => self.query_token(name).await?.into_iter().next().ok_or(sqlx::Error::RowNotFound)?.id,
        };

        self.db.list_users_by_token(token, order, order_by)
    }
}
