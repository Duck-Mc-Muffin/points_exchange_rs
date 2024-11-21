use super::*;

pub trait CliConsumer
{
    fn query_user(&self, name: Option<&str>);
    fn query_token(&self, name: Option<&str>);
    fn transaction(&self, sender_id: UserID, receiver_id: UserID, token_id: TokenID, amount: TokenAmount);
    fn list_user_token(&self, user_id: UserID, token_id: TokenID, order: Order, order_by: Option<OrderBySenderOrAmount>);
    fn list_tokens_by_user(&self, user_id: UserID, order: Order, order_by: Option<OrderByTokenOrSenderOrAmount>);
    fn list_users_by_token(&self, token_id: UserID, order: Order, order_by: Option<OrderByReceiverOrSenderOrAmount>);
    fn create_user(&self, name: &str);
    fn create_token(&self, name: &str);
}
