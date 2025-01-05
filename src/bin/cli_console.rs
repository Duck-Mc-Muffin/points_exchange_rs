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
use points_exchange_rs::core::*;

#[tokio::main]
async fn main()
{
    let args = Args::parse(); // TODO: encapsulate call to remove lib dependency
    let mut core = Core::new().await;

    match args.command
    {
        Action::CreateUser { name } => CliWrapper::create_user(&mut core, &name),
        Action::UserList { name } => CliWrapper::query_user(&mut core, name.as_deref()),
        Action::CreateToken { name } => CliWrapper::create_token(&mut core, &name),
        Action::TokenList { name } => CliWrapper::query_token(&mut core, name.as_deref()),
        Action::Transaction {
            sender_id,
            receiver_id,
            token_id,
            amount,
        } => CliWrapper::transaction(&mut core, sender_id, receiver_id, token_id, amount),
        Action::LsUserToken {
            user_id,
            token_id,
            order,
            order_by,
        } => CliWrapper::list_user_token(&mut core, user_id, token_id, order, order_by),
        Action::LsTokensByUser { user_id, order, order_by } => CliWrapper::list_tokens_by_user(&mut core, user_id, order, order_by),
        Action::LsUsersByToken { token_id, order, order_by } => CliWrapper::list_users_by_token(&mut core, token_id, order, order_by),
    }
}

struct CliWrapper;

impl CliConsumer for CliWrapper
{
    fn create_user(core: &mut Core, name: &str)
    {
        println!("{:?}", core.create_user(name));
    }

    fn query_user(core: &mut Core, name: Option<&str>)
    {
        println!("{:?}", core.query_user(name));
    }

    fn create_token(core: &mut Core, name: &str)
    {
        println!("{:?}", core.create_token(name));
    }

    fn query_token(core: &mut Core, name: Option<&str>)
    {
        println!("{:?}", core.query_token(name));
    }

    fn transaction(core: &mut Core, sender_id: UserID, receiver_id: UserID, token_id: TokenID, amount: TokenAmount)
    {
        println!(
            "{:?}",
            core.transaction(
                UserQueryModeWithCreation::ById(sender_id),
                UserQueryModeWithCreation::ById(receiver_id),
                TokenQueryModeWithCreation::ById(token_id),
                amount
            )
            .unwrap()
        );
    }

    fn list_user_token(core: &mut Core, user_id: UserID, token_id: TokenID, order: Order, order_by: Option<OrderBySenderOrAmount>)
    {
        println!(
            "{:?}",
            core.list_user_token(
                UserQueryModeStrict::ById(user_id),
                TokenQueryModeStrict::ById(token_id),
                order,
                order_by
            )
            .unwrap()
        );
    }

    fn list_tokens_by_user(core: &mut Core, user_id: UserID, order: Order, order_by: Option<OrderByTokenOrSenderOrAmount>)
    {
        println!(
            "{:?}",
            core.list_tokens_by_user(UserQueryModeStrict::ById(user_id), order, order_by)
                .unwrap()
        );
    }

    fn list_users_by_token(core: &mut Core, token_id: UserID, order: Order, order_by: Option<OrderByReceiverOrSenderOrAmount>)
    {
        println!(
            "{:?}",
            core.list_users_by_token(TokenQueryModeStrict::ById(token_id), order, order_by)
                .unwrap()
        );
    }
}
