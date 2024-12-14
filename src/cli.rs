use crate::core::*;
use clap::{Parser, Subcommand};

pub mod cli_consumer;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args
{
    #[command(subcommand)]
    pub command: Action,
}

#[derive(Subcommand)]
pub enum Action
{
    /// Register a new user
    CreateUser
    {
        /// Displayed name of this new user
        name: String,
    },
    /// Get user information by user name (or all users)
    UserList
    {
        /// User name to search for
        name: Option<String>,
    },

    /// Register a new token explicitly
    CreateToken
    {
        /// Displayed name of this new token
        name: String,
    },
    /// Get token information by token name (or all tokens)
    TokenList
    {
        /// Token name to search for
        name: Option<String>,
    },

    /// Send tokens from User A to User B
    #[command(name = "tr")]
    Transaction
    {
        /// Token "sender"
        sender_id: UserID,

        /// Token "receiver"
        receiver_id: UserID,

        /// Token ID
        token_id: TokenID,

        /// Amount
        amount: TokenAmount,
    },

    /// Show the amount of a _specific_ token a _specific_ user received from each other user
    LsUserToken
    {
        /// User
        user_id: UserID,

        /// Token
        token_id: TokenID,

        /// Order output in ascending (asc) or descending (desc) order
        #[arg(value_enum, long, short = 'o', default_value_t=Order::Desc)]
        order: Order,

        /// Which parameter should be used to order the output
        #[arg(value_enum, long)]
        order_by: Option<OrderBySenderOrAmount>,
    },
    /// Show the amount of _all_ tokens a _specific_ user received from each other user
    LsTokensByUser
    {
        /// User
        user_id: UserID,

        /// Order output in ascending (asc) or descending (desc) order
        #[arg(value_enum, long, short = 'o', default_value_t=Order::Desc)]
        order: Order,

        /// Which parameter should be used to order the output
        #[arg(value_enum, long)]
        order_by: Option<OrderByTokenOrSenderOrAmount>,
    },
    /// Show the amount of a _specific_ token _all_ users received from each other user
    LsUsersByToken
    {
        /// Token
        token_id: TokenID,

        /// Order output in ascending (asc) or descending (desc) order
        #[arg(value_enum, long, short = 'o', default_value_t=Order::Desc)]
        order: Order,

        /// Which parameter should be used to order the output
        #[arg(value_enum, long = "order-by")]
        order_by: Option<OrderByReceiverOrSenderOrAmount>,
    },
}
