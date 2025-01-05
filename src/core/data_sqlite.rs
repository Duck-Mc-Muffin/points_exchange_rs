use super::persistance_layer::PersistanceLayer;
use super::*;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

#[derive(Debug)]
pub struct DataSQLite
{
    _connection_pool: Option<SqlitePool>,
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
            Ok(pool) => Some(pool),
            Err(err) => panic!("SQLite error: {}", err),
        };

        DataSQLite {
            _connection_pool: connection_pool,
        }
    }
}

impl PersistanceLayer for DataSQLite
{
    fn create_user(&mut self, _name: &str) -> UserListEntry
    {
        // INSERT INTO user(name) VALUES(:name)
        // LAST ID
        UserListEntry {
            id:   233,
            name: "Max Mustermann".to_string(),
        }
    }

    fn query_user(&mut self, _name: Option<&str>) -> Vec<UserListEntry>
    {
        // SELECT * FROM user
        // SELECT * FROM user WHERE name LIKE '%:name%'
        vec![UserListEntry {
            id:   974,
            name: "John Doe".to_string(),
        }]
    }

    fn create_token(&mut self, _name: &str) -> TokenListEntry
    {
        // INSERT INTO token(name) VALUES(:name)
        // LAST ID
        TokenListEntry {
            id:   890,
            name: "my-other-token".to_string(),
        }
    }

    fn query_token(&mut self, _name: Option<&str>) -> Vec<TokenListEntry>
    {
        // SELECT * FROM token
        // SELECT * FROM token WHERE name LIKE '%:name%'
        vec![TokenListEntry {
            id:   863,
            name: "my-token".to_string(),
        }]
    }

    fn get_current_total(
        &mut self,
        _sender: UserQueryModeStrict,
        _receiver: UserQueryModeStrict,
        _token: TokenQueryModeStrict,
    ) -> Option<TokenAmount>
    {
        // SELECT current_total FROM user_balance WHERE sender_id = :sender_id, receiver_id = :receiver_id, token_id = :token_id
        Some(1337)
    }

    fn transaction(
        &mut self,
        _sender: UserQueryModeStrict,
        _receiver: UserQueryModeStrict,
        _token: TokenQueryModeStrict,
        _amount: TokenAmount,
    ) -> Result<TokenAmount, ()>
    {
        // INSERT INTO transaction_history(sender_id, receiver_id, token_id, amount) VALUES(:sender_id, :receiver_id, :token_id, :amount)
        Ok(420)
    }

    fn list_user_token(
        &mut self,
        _receiver: UserQueryModeStrict,
        _token: TokenQueryModeStrict,
        _order: Order,
        _order_by: Option<OrderBySenderOrAmount>,
    ) -> Result<Vec<RelativeUserTokenAmountEntry>, ()>
    {
        // SELECT sender.*, balance.mount
        // FROM user_balance AS balance
        // JOIN user AS sender ON sender.id = balance.sender_id
        // WHERE receiver_id = :receiver_id, token_id = :token_id
        // ORDER BY :order_by :order
        Ok(vec![RelativeUserTokenAmountEntry {
            sender: UserListEntry {
                id:   1,
                name: "Jane Doe".to_string(),
            },
            amount: 4321,
        }])
    }

    fn list_tokens_by_user(
        &mut self,
        _receiver: UserQueryModeStrict,
        _order: Order,
        _order_by: Option<OrderByTokenOrSenderOrAmount>,
    ) -> Result<Vec<RelativeTokenAmountEntry>, ()>
    {
        // SELECT sender.*, token.*, balance.amount
        // FROM user_balance AS balance
        // JOIN user AS sender ON sender.id = balance.sender_id
        // WHERE receiver_id = :receiver_id
        // ORDER BY :order_by :order
        Ok(vec![RelativeTokenAmountEntry {
            token:            TokenListEntry {
                id:   2,
                name: "yet-another-token".to_string(),
            },
            amount_by_sender: vec![RelativeUserTokenAmountEntry {
                sender: UserListEntry {
                    id:   1,
                    name: "Jane Doe".to_string(),
                },
                amount: 4321,
            }],
        }])
    }

    fn list_users_by_token(
        &mut self,
        _token: TokenQueryModeStrict,
        _order: Order,
        _order_by: Option<OrderByReceiverOrSenderOrAmount>,
    ) -> Result<Vec<RelativeUserAmountEntry>, ()>
    {
        // SELECT sender.*, receiver.*, balance.amount
        // FROM user_balance AS balance
        // JOIN user AS sender ON sender.id = balance.sender_id
        // JOIN user AS receiver ON receiver.id = balance.receiver_id
        // WHERE balance.token_id = :token_id
        // ORDER BY :order_by :order
        Ok(vec![RelativeUserAmountEntry {
            receiver:         UserListEntry {
                id:   5,
                name: "John doe".to_string(),
            },
            amount_by_sender: vec![RelativeUserTokenAmountEntry {
                sender: UserListEntry {
                    id:   1,
                    name: "Max Mustermann".to_string(),
                },
                amount: 4321,
            }],
        }])
    }
}
