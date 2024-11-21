use clap::ValueEnum;
use cli_consumer::CliConsumer;

pub mod cli_consumer;

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

#[derive(Debug)]
pub struct CliDummy;

impl CliConsumer for CliDummy
{
    fn query_user(&self, name: Option<&str>)
    {
        println!("UserList: {name:?}");
    }

    fn query_token(&self, name: Option<&str>)
    {
        println!("TokenList: {name:?}");
    }

    fn transaction(&self, sender_id: UserID, receiver_id: UserID, token_id: TokenID, amount: TokenAmount)
    {
        println!("Tr: {sender_id}, {receiver_id}, {token_id}, {amount}");
    }

    fn list_user_token(&self, user_id: UserID, token_id: TokenID, order: Order, order_by: Option<OrderBySenderOrAmount>)
    {
        println!("LsUserTokens: {user_id}, {token_id}, {order:?}, {order_by:?}");
    }

    fn list_tokens_by_user(&self, user_id: UserID, order: Order, order_by: Option<OrderByTokenOrSenderOrAmount>)
    {
        println!("LsTokens: {user_id}, {order:?}, {order_by:?}");
    }

    fn list_users_by_token(&self, token_id: UserID, order: Order, order_by: Option<OrderByReceiverOrSenderOrAmount>)
    {
        println!("LsUsers: {token_id}, {order:?}, {order_by:?}");
    }

    fn create_user(&self, name: &str)
    {
        println!("CreateUser: {name}");
    }

    fn create_token(&self, name: &str)
    {
        println!("CreateToken: {name}");
    }
}
