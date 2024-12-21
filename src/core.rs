use clap::ValueEnum;

// mod token;
// mod user;
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

// ================================================ User Management ================================================
#[derive(Debug)]
pub struct UserListEntry
{
    pub id:   UserID,
    pub name: String,
}

pub fn create_user(_name: &str) -> UserListEntry
{
    // INSERT INTO user(name) VALUES(:name)
    // LAST ID
    UserListEntry {
        id:   233,
        name: "Max Mustermann".to_string(),
    }
}

pub fn query_user(_name: Option<&str>) -> Vec<UserListEntry>
{
    // SELECT * FROM user
    // SELECT * FROM user WHERE name LIKE '%:name%'
    vec![UserListEntry {
        id:   974,
        name: "John Doe".to_string(),
    }]
}

// ================================================ Token Management ================================================
#[derive(Debug)]
pub struct TokenListEntry
{
    pub id:   TokenID,
    pub name: String,
}

pub fn create_token(_name: &str) -> TokenListEntry
{
    // INSERT INTO token(name) VALUES(:name)
    // LAST ID
    TokenListEntry {
        id:   890,
        name: "my-other-token".to_string(),
    }
}

pub fn query_token(_name: Option<&str>) -> Vec<TokenListEntry>
{
    // SELECT * FROM token
    // SELECT * FROM token WHERE name LIKE '%:name%'
    vec![TokenListEntry {
        id:   863,
        name: "my-token".to_string(),
    }]
}

// ================================================ Transactions ================================================
// TODO: Define error type
pub fn transaction(
    sender: UserQueryModeWithCreation,
    receiver: UserQueryModeWithCreation,
    token: TokenQueryModeWithCreation,
    amount: TokenAmount,
) -> Result<TokenAmount, ()>
{
    let _sender_id: UserID = match sender
    {
        // TODO: pop-ing the >last< match is not intuitive
        UserQueryModeWithCreation::ById(id) => id,
        UserQueryModeWithCreation::ByName(name) => query_user(Some(name)).pop().ok_or(())?.id,
        UserQueryModeWithCreation::ByNameOrCreate(name) => query_user(Some(name)).pop().unwrap_or_else(|| create_user(name)).id,
    };

    let _receiver_id: UserID = match receiver
    {
        // TODO: pop-ing the >last< match is not intuitive
        UserQueryModeWithCreation::ById(id) => id,
        UserQueryModeWithCreation::ByName(name) => query_user(Some(name)).pop().ok_or(())?.id,
        UserQueryModeWithCreation::ByNameOrCreate(name) => query_user(Some(name)).pop().unwrap_or_else(|| create_user(name)).id,
    };

    let _token_id: UserID = match token
    {
        // TODO: pop-ing the >last< match is not intuitive
        TokenQueryModeWithCreation::ById(id) => id,
        TokenQueryModeWithCreation::ByName(name) => query_token(Some(name)).pop().ok_or(())?.id,
        TokenQueryModeWithCreation::ByNameOrCreate(name) => query_token(Some(name)).pop().unwrap_or_else(|| create_token(name)).id,
    };

    // SELECT current_total FROM user_balance WHERE sender_id = :sender_id, receiver_id = :receiver_id, token_id = :token_id
    let previous_total: TokenAmount = 100;

    // INSERT INTO transaction_history(sender_id, receiver_id, token_id, amount) VALUES(:sender_id, :receiver_id, :token_id, :amount)
    Ok(previous_total + amount)
}

// ================================================ List Tokens =================================================
#[derive(Debug)]
pub struct RelativeUserTokenAmountEntry
{
    pub sender: UserListEntry,
    pub amount: TokenAmount,
}

pub fn list_user_token(
    receiver: UserQueryModeStrict,
    token: TokenQueryModeStrict,
    _order: Order,
    _order_by: Option<OrderBySenderOrAmount>,
) -> Result<Vec<RelativeUserTokenAmountEntry>, ()>
{
    let _receiver_id: UserID = match receiver
    {
        // TODO: pop-ing the >last< match is not optimal
        UserQueryModeStrict::ById(id) => id,
        UserQueryModeStrict::ByName(name) => query_user(Some(name)).pop().ok_or(())?.id,
    };

    let _token_id: UserID = match token
    {
        // TODO: pop-ing the >last< match is not optimal
        TokenQueryModeStrict::ById(id) => id,
        TokenQueryModeStrict::ByName(name) => query_token(Some(name)).pop().ok_or(())?.id,
    };

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

#[derive(Debug)]
pub struct RelativeTokenAmountEntry
{
    pub token:            TokenListEntry,
    pub amount_by_sender: Vec<RelativeUserTokenAmountEntry>,
}

pub fn list_tokens_by_user(
    receiver: UserQueryModeStrict,
    _order: Order,
    _order_by: Option<OrderByTokenOrSenderOrAmount>,
) -> Result<Vec<RelativeTokenAmountEntry>, ()>
{
    let _receiver_id: UserID = match receiver
    {
        // TODO: pop-ing the >last< match is not optimal
        UserQueryModeStrict::ById(id) => id,
        UserQueryModeStrict::ByName(name) => query_user(Some(name)).pop().ok_or(())?.id,
    };

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

#[derive(Debug)]
pub struct RelativeUserAmountEntry
{
    pub receiver:         UserListEntry,
    pub amount_by_sender: Vec<RelativeUserTokenAmountEntry>,
}

pub fn list_users_by_token(
    token: TokenQueryModeStrict,
    _order: Order,
    _order_by: Option<OrderByReceiverOrSenderOrAmount>,
) -> Result<Vec<RelativeUserAmountEntry>, ()>
{
    let _token_id: UserID = match token
    {
        // TODO: pop-ing the >last< match is not optimal
        TokenQueryModeStrict::ById(id) => id,
        TokenQueryModeStrict::ByName(name) => query_token(Some(name)).pop().ok_or(())?.id,
    };

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

/*
 * CLI DEFINITION:
 *
 * create-user <name>
 *      <new_id>
 * user-list [<user_name>]
 *      <id> <name>
 *
 * create-token <token_name>
 *      <new_id>
 * token-list [<token_name>]
 *      <id> <name> <owner_name> <owner_id>
 *
 * tr <sender_user_id> <receiver_user_id> <token_id> [-]<amount>
 *      <current_amount>
 *
 * ls-user-tokens <user_id> <token_id> [--order-by=(sender|amount)] [--asc|--desc]
 *      <sender> <amount>
 *      ...
 *
 * ls-tokens <user_id> [--order-by=(token|sender|amount)] [--asc|--desc]
 *      <token> <sender_user> <amount>
 *      ...
 *
 * ls-users <token_id> [--order-by=(receiver|sender|amount)] [--asc|--desc]
 *      <receiver_user> <sender_user> <amount>
 *      ...
 */
