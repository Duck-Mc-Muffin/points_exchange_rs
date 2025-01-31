use super::{
    persistance_layer::*, Order, OrderByReceiverOrSenderOrAmount, OrderBySenderOrAmount, OrderByTokenOrSenderOrAmount, TokenAmount,
    TokenQueryModeStrict, UserQueryModeStrict,
};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

#[derive(Debug)]
pub struct DataSQLite
{
    connection_pool: SqlitePool,
}

const MAX_CONNECTIONS: u32 = 5;

impl DataSQLite
{
    pub async fn new() -> DataSQLite
    {
        let pool = SqlitePoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect("sqlite://var/data.db" /* TODO */)
            .await;

        let connection_pool = match pool
        {
            Ok(pool) => pool,
            Err(err) => panic!("SQLite error: {}", err),
        };

        DataSQLite { connection_pool }
    }
}

impl PersistanceLayer for DataSQLite
{
    async fn create_user(&self, name: &str) -> Result<User, sqlx::Error>
    {
        sqlx::query_as!(User, "INSERT INTO User(name) VALUES (?) RETURNING *", name)
            .fetch_one(&self.connection_pool)
            .await
    }

    async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error>
    {
        // SELECT * FROM user
        Ok(vec![User {
            id:   974,
            name: "John Doe".to_string(),
        }])
    }

    async fn query_user(&self, _name: &str) -> Result<Vec<User>, sqlx::Error>
    {
        // SELECT * FROM user WHERE name LIKE '%:name%'
        Ok(vec![User {
            id:   974,
            name: "John Doe".to_string(),
        }])
    }

    async fn create_token(&self, name: &str) -> Result<Token, sqlx::Error>
    {
        sqlx::query_as!(Token, "INSERT INTO Token(name) VALUES (?) RETURNING *", name)
            .fetch_one(&self.connection_pool)
            .await
    }

    async fn get_all_tokens(&self) -> Result<Vec<Token>, sqlx::Error>
    {
        // SELECT * FROM token
        Ok(vec![Token {
            id:   863,
            name: "my-token".to_string(),
        }])
    }

    async fn query_token(&self, _name: &str) -> Result<Vec<Token>, sqlx::Error>
    {
        // SELECT * FROM token WHERE name LIKE '%:name%'
        Ok(vec![Token {
            id:   863,
            name: "my-token".to_string(),
        }])
    }

    /// A return value of _None_ means, there are no transactions present.
    /// _Some(0)_ means, all found transactions sum up to zero.
    async fn get_current_total(
        &self,
        _sender: UserQueryModeStrict<'_>,
        _receiver: UserQueryModeStrict<'_>,
        _token: TokenQueryModeStrict<'_>,
    ) -> Result<Option<TokenAmount>, sqlx::Error>
    {
        // SELECT current_total FROM user_balance WHERE sender_id = :sender_id, receiver_id = :receiver_id, token_id = :token_id
        Ok(Some(1337))
    }

    async fn transaction(
        &self,
        _sender: UserQueryModeStrict<'_>,
        _receiver: UserQueryModeStrict<'_>,
        _token: TokenQueryModeStrict<'_>,
        _amount: TokenAmount,
    ) -> Result<TokenAmount, sqlx::Error>
    {
        // INSERT INTO transaction_history(sender_id, receiver_id, token_id, amount) VALUES(:sender_id, :receiver_id, :token_id, :amount)
        Ok(420)
    }

    async fn list_user_token(
        &self,
        _receiver: UserQueryModeStrict<'_>,
        _token: TokenQueryModeStrict<'_>,
        _order: Order,
        _order_by: Option<OrderBySenderOrAmount>,
    ) -> Result<Vec<RelativeUserTokenAmountEntry>, sqlx::Error>
    {
        // SELECT sender.*, balance.mount
        // FROM user_balance AS balance
        // JOIN user AS sender ON sender.id = balance.sender_id
        // WHERE receiver_id = :receiver_id, token_id = :token_id
        // ORDER BY :order_by :order
        Ok(vec![RelativeUserTokenAmountEntry {
            sender: User {
                id:   1,
                name: "Jane Doe".to_string(),
            },
            amount: 4321,
        }])
    }

    async fn list_tokens_by_user(
        &self,
        _receiver: UserQueryModeStrict<'_>,
        _order: Order,
        _order_by: Option<OrderByTokenOrSenderOrAmount>,
    ) -> Result<Vec<RelativeTokenAmountEntry>, sqlx::Error>
    {
        // SELECT sender.*, token.*, balance.amount
        // FROM user_balance AS balance
        // JOIN user AS sender ON sender.id = balance.sender_id
        // WHERE receiver_id = :receiver_id
        // ORDER BY :order_by :order
        Ok(vec![RelativeTokenAmountEntry {
            token:            Token {
                id:   2,
                name: "yet-another-token".to_string(),
            },
            amount_by_sender: vec![RelativeUserTokenAmountEntry {
                sender: User {
                    id:   1,
                    name: "Jane Doe".to_string(),
                },
                amount: 4321,
            }],
        }])
    }

    async fn list_users_by_token(
        &self,
        _token: TokenQueryModeStrict<'_>,
        _order: Order,
        _order_by: Option<OrderByReceiverOrSenderOrAmount>,
    ) -> Result<Vec<RelativeUserAmountEntry>, sqlx::Error>
    {
        // SELECT sender.*, receiver.*, balance.amount
        // FROM user_balance AS balance
        // JOIN user AS sender ON sender.id = balance.sender_id
        // JOIN user AS receiver ON receiver.id = balance.receiver_id
        // WHERE balance.token_id = :token_id
        // ORDER BY :order_by :order
        Ok(vec![RelativeUserAmountEntry {
            receiver:         User {
                id:   5,
                name: "John doe".to_string(),
            },
            amount_by_sender: vec![RelativeUserTokenAmountEntry {
                sender: User {
                    id:   1,
                    name: "Max Mustermann".to_string(),
                },
                amount: 4321,
            }],
        }])
    }
}
