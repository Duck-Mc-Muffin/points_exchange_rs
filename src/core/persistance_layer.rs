use super::*;

/// SQLite requires i64 according to SQLX type mapping.
/// Other persistance layer implementations might need to accommodate this.
pub type DbPk = i64;

// WORKAROUND {AsyncPersistanceLayerTraitObject}:
//      Since async in trait objects is not a thing yet in rust, this enum will wrap all implementations for now
#[derive(Debug)]
pub enum DbImplementors
{
    SQLite(DataSQLite),
}

#[derive(Debug)]
pub struct User
{
    pub id:   DbPk,
    pub name: String,
}

#[derive(Debug)]
pub struct Token
{
    pub id:   DbPk,
    pub name: String,
}

#[derive(Debug)]
pub struct RelativeUserAmountEntry
{
    pub receiver:         User,
    pub amount_by_sender: Vec<RelativeUserTokenAmountEntry>,
}

#[derive(Debug)]
pub struct RelativeTokenAmountEntry
{
    pub token:            Token,
    pub amount_by_sender: Vec<RelativeUserTokenAmountEntry>,
}

#[derive(Debug)]
pub struct RelativeUserTokenAmountEntry
{
    pub sender: User,
    pub amount: TokenAmount,
}

pub trait PersistanceLayer
{
    // INSERT INTO user(name) VALUES(:name)
    // LAST ID
    async fn create_user(&self, name: &str) -> Result<User, sqlx::Error>;

    // SELECT * FROM user
    async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error>;

    // SELECT * FROM user WHERE name LIKE '%:name%'
    async fn query_user(&self, name: &str) -> Result<Vec<User>, sqlx::Error>;

    // INSERT INTO token(name) VALUES(:name)
    // LAST ID
    async fn create_token(&self, name: &str) -> Result<Token, sqlx::Error>;

    // SELECT * FROM token
    async fn get_all_tokens(&self) -> Result<Vec<Token>, sqlx::Error>;

    // SELECT * FROM token WHERE name LIKE '%:name%'
    async fn query_token(&self, name: &str) -> Result<Vec<Token>, sqlx::Error>;

    // SELECT current_total FROM user_balance WHERE sender_id = :sender_id, receiver_id = :receiver_id, token_id = :token_id
    async fn get_current_total(
        &self,
        _sender: UserQueryModeStrict<'_>,
        _receiver: UserQueryModeStrict<'_>,
        _token: TokenQueryModeStrict<'_>,
    ) -> Result<Option<TokenAmount>, sqlx::Error>;

    // INSERT INTO transaction_history(sender_id, receiver_id, token_id, amount) VALUES(:sender_id, :receiver_id, :token_id, :amount)
    async fn transaction(
        &self,
        _sender: UserQueryModeStrict,
        _receiver: UserQueryModeStrict,
        _token: TokenQueryModeStrict,
        _amount: TokenAmount,
    ) -> Result<TokenAmount, sqlx::Error>;

    // SELECT sender.*, balance.mount
    // FROM user_balance AS balance
    // JOIN user AS sender ON sender.id = balance.sender_id
    // WHERE receiver_id = :receiver_id, token_id = :token_id
    // ORDER BY :order_by :order
    async fn list_user_token(
        &self,
        _receiver: UserQueryModeStrict,
        _token: TokenQueryModeStrict,
        _order: Order,
        _order_by: Option<OrderBySenderOrAmount>,
    ) -> Result<Vec<RelativeUserTokenAmountEntry>, sqlx::Error>;

    // SELECT sender.*, token.*, balance.amount
    // FROM user_balance AS balance
    // JOIN user AS sender ON sender.id = balance.sender_id
    // WHERE receiver_id = :receiver_id
    // ORDER BY :order_by :order
    async fn list_tokens_by_user(
        &self,
        _receiver: UserQueryModeStrict,
        _order: Order,
        _order_by: Option<OrderByTokenOrSenderOrAmount>,
    ) -> Result<Vec<RelativeTokenAmountEntry>, sqlx::Error>;

    // SELECT sender.*, receiver.*, balance.amount
    // FROM user_balance AS balance
    // JOIN user AS sender ON sender.id = balance.sender_id
    // JOIN user AS receiver ON receiver.id = balance.receiver_id
    // WHERE balance.token_id = :token_id
    // ORDER BY :order_by :order
    async fn list_users_by_token(
        &self,
        _token: TokenQueryModeStrict,
        _order: Order,
        _order_by: Option<OrderByReceiverOrSenderOrAmount>,
    ) -> Result<Vec<RelativeUserAmountEntry>, sqlx::Error>;
}
