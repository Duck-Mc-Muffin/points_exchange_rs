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

use clap::{arg, Parser, Subcommand};
use cli_consumer::CliConsumer;
use points_exchange_rs::cli::*;

fn main()
{
    let args = Args::parse();
    let wrapper = CliDummy;

    match args.command
    {
        Action::CreateUser { name } => wrapper.create_user(&name),
        Action::UserList { name } => wrapper.query_user(name.as_deref()),
        Action::CreateToken { name } => wrapper.create_token(&name),
        Action::TokenList { name } => wrapper.query_token(name.as_deref()),
        Action::Transaction {
            sender_id,
            receiver_id,
            token_id,
            amount,
        } => wrapper.transaction(sender_id, receiver_id, token_id, amount),
        Action::LsUserToken {
            user_id,
            token_id,
            order,
            order_by,
        } => wrapper.list_user_token(user_id, token_id, order, order_by),
        Action::LsTokensByUser { user_id, order, order_by } => wrapper.list_tokens_by_user(user_id, order, order_by),
        Action::LsUsersByToken { token_id, order, order_by } => wrapper.list_users_by_token(token_id, order, order_by),
    }
}

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args
{
    #[command(subcommand)]
    command: Action,
}

#[derive(Subcommand)]
enum Action
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
