use clap::ValueEnum;
use data_sqlite::DataSQLite;
use persistance_layer::PersistanceLayer;

// mod token;
// mod user;
mod data_sqlite;
mod persistance_layer;

pub type UserID = u32;
pub type TokenID = u32;
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
    db: Box<dyn PersistanceLayer>,
}

#[derive(Debug)]
pub struct UserListEntry
{
    pub id:   UserID,
    pub name: String,
}

#[derive(Debug)]
pub struct RelativeUserAmountEntry
{
    pub receiver:         UserListEntry,
    pub amount_by_sender: Vec<RelativeUserTokenAmountEntry>,
}

#[derive(Debug)]
pub struct RelativeTokenAmountEntry
{
    pub token:            TokenListEntry,
    pub amount_by_sender: Vec<RelativeUserTokenAmountEntry>,
}

#[derive(Debug)]
pub struct RelativeUserTokenAmountEntry
{
    pub sender: UserListEntry,
    pub amount: TokenAmount,
}

#[derive(Debug)]
pub struct TokenListEntry
{
    pub id:   TokenID,
    pub name: String,
}

impl Core
{
    pub async fn new() -> Core
    {
        Core {
            db: Box::new(DataSQLite::new().await),
        }
    }

    // ================================================ User Management ================================================
    pub fn create_user(&mut self, name: &str) -> UserListEntry
    {
        self.db.create_user(name)
    }

    pub fn query_user(&mut self, name: Option<&str>) -> Vec<UserListEntry>
    {
        self.db.query_user(name)
    }

    // ================================================ Token Management ================================================
    pub fn create_token(&mut self, name: &str) -> TokenListEntry
    {
        self.db.create_token(name)
    }

    pub fn query_token(&mut self, name: Option<&str>) -> Vec<TokenListEntry>
    {
        self.db.query_token(name)
    }

    // ================================================ Transactions ================================================
    // TODO: Define error type
    pub fn transaction(
        &mut self,
        sender: UserQueryModeWithCreation,
        receiver: UserQueryModeWithCreation,
        token: TokenQueryModeWithCreation,
        amount: TokenAmount,
    ) -> Result<TokenAmount, ()>
    {
        let sender_id: UserID = match sender
        {
            // TODO: pop-ing the >last< match is not intuitive
            UserQueryModeWithCreation::ById(id) => id,
            UserQueryModeWithCreation::ByName(name) => self.db.query_user(Some(name)).pop().ok_or(())?.id,
            UserQueryModeWithCreation::ByNameOrCreate(name) =>
            {
                self.db.query_user(Some(name)).pop().unwrap_or_else(|| self.db.create_user(name)).id
            }
        };

        let receiver_id: UserID = match receiver
        {
            // TODO: pop-ing the >last< match is not intuitive
            UserQueryModeWithCreation::ById(id) => id,
            UserQueryModeWithCreation::ByName(name) => self.db.query_user(Some(name)).pop().ok_or(())?.id,
            UserQueryModeWithCreation::ByNameOrCreate(name) =>
            {
                self.db.query_user(Some(name)).pop().unwrap_or_else(|| self.db.create_user(name)).id
            }
        };

        let token_id: UserID = match token
        {
            // TODO: pop-ing the >last< match is not intuitive
            TokenQueryModeWithCreation::ById(id) => id,
            TokenQueryModeWithCreation::ByName(name) => self.db.query_token(Some(name)).pop().ok_or(())?.id,
            TokenQueryModeWithCreation::ByNameOrCreate(name) =>
            {
                self.db
                    .query_token(Some(name))
                    .pop()
                    .unwrap_or_else(|| self.db.create_token(name))
                    .id
            }
        };

        let previous_total = self
            .db
            .get_current_total(
                UserQueryModeStrict::ById(sender_id),
                UserQueryModeStrict::ById(receiver_id),
                TokenQueryModeStrict::ById(token_id),
            )
            .unwrap_or_default();

        Ok(previous_total
            + self
                .db
                .transaction(
                    UserQueryModeStrict::ById(sender_id),
                    UserQueryModeStrict::ById(receiver_id),
                    TokenQueryModeStrict::ById(token_id),
                    amount,
                )
                .unwrap_or_default())
    }

    // ================================================ List Tokens =================================================
    pub fn list_user_token(
        &mut self,
        receiver: UserQueryModeStrict,
        token: TokenQueryModeStrict,
        order: Order,
        order_by: Option<OrderBySenderOrAmount>,
    ) -> Result<Vec<RelativeUserTokenAmountEntry>, ()>
    {
        let _receiver_id: UserID = match receiver
        {
            // TODO: pop-ing the >last< match is not optimal
            UserQueryModeStrict::ById(id) => id,
            UserQueryModeStrict::ByName(name) => self.query_user(Some(name)).pop().ok_or(())?.id,
        };

        let _token_id: UserID = match token
        {
            // TODO: pop-ing the >last< match is not optimal
            TokenQueryModeStrict::ById(id) => id,
            TokenQueryModeStrict::ByName(name) => self.query_token(Some(name)).pop().ok_or(())?.id,
        };

        self.db.list_user_token(receiver, token, order, order_by)
    }

    pub fn list_tokens_by_user(
        &mut self,
        receiver: UserQueryModeStrict,
        order: Order,
        order_by: Option<OrderByTokenOrSenderOrAmount>,
    ) -> Result<Vec<RelativeTokenAmountEntry>, ()>
    {
        let _receiver_id: UserID = match receiver
        {
            // TODO: pop-ing the >last< match is not optimal
            UserQueryModeStrict::ById(id) => id,
            UserQueryModeStrict::ByName(name) => self.query_user(Some(name)).pop().ok_or(())?.id,
        };

        self.db.list_tokens_by_user(receiver, order, order_by)
    }

    pub fn list_users_by_token(
        &mut self,
        token: TokenQueryModeStrict,
        order: Order,
        order_by: Option<OrderByReceiverOrSenderOrAmount>,
    ) -> Result<Vec<RelativeUserAmountEntry>, ()>
    {
        let _token_id: UserID = match token
        {
            // TODO: pop-ing the >last< match is not optimal
            TokenQueryModeStrict::ById(id) => id,
            TokenQueryModeStrict::ByName(name) => self.query_token(Some(name)).pop().ok_or(())?.id,
        };

        self.db.list_users_by_token(token, order, order_by)
    }
}
