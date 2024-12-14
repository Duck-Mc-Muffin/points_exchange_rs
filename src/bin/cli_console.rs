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

use clap::Parser;
use points_exchange_rs::cli::cli_consumer::CliConsumer; // TODO: alias in module or something?
use points_exchange_rs::cli::*;
use points_exchange_rs::core;
use points_exchange_rs::core::{TokenQueryModeStrict, TokenQueryModeWithCreation, UserQueryModeStrict, UserQueryModeWithCreation};

fn main()
{
    let args = Args::parse(); // TODO: encapsulate call to remove lib dependency
    let wrapper = CliWrapper;

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

struct CliWrapper;

impl CliConsumer for CliWrapper
{
    fn create_user(&self, name: &str)
    {
        println!("{:?}", core::create_user(name));
    }

    fn query_user(&self, name: Option<&str>)
    {
        println!("{:?}", core::query_user(name));
    }

    fn create_token(&self, name: &str)
    {
        println!("{:?}", core::create_token(name));
    }

    fn query_token(&self, name: Option<&str>)
    {
        println!("{:?}", core::query_token(name));
    }

    fn transaction(
        &self,
        sender_id: points_exchange_rs::core::UserID,
        receiver_id: points_exchange_rs::core::UserID,
        token_id: points_exchange_rs::core::TokenID,
        amount: points_exchange_rs::core::TokenAmount,
    )
    {
        println!(
            "{:?}",
            core::transaction(
                UserQueryModeWithCreation::ById(sender_id),
                UserQueryModeWithCreation::ById(receiver_id),
                TokenQueryModeWithCreation::ById(token_id),
                amount
            )
            .unwrap()
        );
    }

    fn list_user_token(
        &self,
        user_id: points_exchange_rs::core::UserID,
        token_id: points_exchange_rs::core::TokenID,
        order: points_exchange_rs::core::Order,
        order_by: Option<points_exchange_rs::core::OrderBySenderOrAmount>,
    )
    {
        println!(
            "{:?}",
            core::list_user_token(
                UserQueryModeStrict::ById(user_id),
                TokenQueryModeStrict::ById(token_id),
                order,
                order_by
            )
            .unwrap()
        );
    }

    fn list_tokens_by_user(
        &self,
        user_id: points_exchange_rs::core::UserID,
        order: points_exchange_rs::core::Order,
        order_by: Option<points_exchange_rs::core::OrderByTokenOrSenderOrAmount>,
    )
    {
        println!(
            "{:?}",
            core::list_tokens_by_user(UserQueryModeStrict::ById(user_id), order, order_by).unwrap()
        );
    }

    fn list_users_by_token(
        &self,
        token_id: points_exchange_rs::core::UserID,
        order: points_exchange_rs::core::Order,
        order_by: Option<points_exchange_rs::core::OrderByReceiverOrSenderOrAmount>,
    )
    {
        println!(
            "{:?}",
            core::list_users_by_token(TokenQueryModeStrict::ById(token_id), order, order_by).unwrap()
        );
    }
}
