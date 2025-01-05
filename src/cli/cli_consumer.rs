use crate::core::Core;

use super::*;

pub trait CliConsumer
{
    fn query_user(core: &mut Core, name: Option<&str>);
    fn query_token(core: &mut Core, name: Option<&str>);
    fn transaction(core: &mut Core, sender_id: UserID, receiver_id: UserID, token_id: TokenID, amount: TokenAmount);
    fn list_user_token(core: &mut Core, user_id: UserID, token_id: TokenID, order: Order, order_by: Option<OrderBySenderOrAmount>);
    fn list_tokens_by_user(core: &mut Core, user_id: UserID, order: Order, order_by: Option<OrderByTokenOrSenderOrAmount>);
    fn list_users_by_token(core: &mut Core, token_id: UserID, order: Order, order_by: Option<OrderByReceiverOrSenderOrAmount>);
    fn create_user(core: &mut Core, name: &str);
    fn create_token(core: &mut Core, name: &str);
}
