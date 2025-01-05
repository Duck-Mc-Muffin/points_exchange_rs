use super::*;

pub trait PersistanceLayer
{
    // INSERT INTO user(name) VALUES(:name)
    // LAST ID
    fn create_user(&mut self, _name: &str) -> UserListEntry;

    // SELECT * FROM user
    // SELECT * FROM user WHERE name LIKE '%:name%'
    fn query_user(&mut self, _name: Option<&str>) -> Vec<UserListEntry>;

    // INSERT INTO token(name) VALUES(:name)
    // LAST ID
    fn create_token(&mut self, _name: &str) -> TokenListEntry;

    // SELECT * FROM token
    // SELECT * FROM token WHERE name LIKE '%:name%'
    fn query_token(&mut self, _name: Option<&str>) -> Vec<TokenListEntry>;

    // SELECT current_total FROM user_balance WHERE sender_id = :sender_id, receiver_id = :receiver_id, token_id = :token_id
    fn get_current_total(
        &mut self,
        _sender: UserQueryModeStrict,
        _receiver: UserQueryModeStrict,
        _token: TokenQueryModeStrict,
    ) -> Option<TokenAmount>;

    // INSERT INTO transaction_history(sender_id, receiver_id, token_id, amount) VALUES(:sender_id, :receiver_id, :token_id, :amount)
    fn transaction(
        &mut self,
        _sender: UserQueryModeStrict,
        _receiver: UserQueryModeStrict,
        _token: TokenQueryModeStrict,
        _amount: TokenAmount,
    ) -> Result<TokenAmount, ()>;

    // SELECT sender.*, balance.mount
    // FROM user_balance AS balance
    // JOIN user AS sender ON sender.id = balance.sender_id
    // WHERE receiver_id = :receiver_id, token_id = :token_id
    // ORDER BY :order_by :order
    fn list_user_token(
        &mut self,
        _receiver: UserQueryModeStrict,
        _token: TokenQueryModeStrict,
        _order: Order,
        _order_by: Option<OrderBySenderOrAmount>,
    ) -> Result<Vec<RelativeUserTokenAmountEntry>, ()>;

    // SELECT sender.*, token.*, balance.amount
    // FROM user_balance AS balance
    // JOIN user AS sender ON sender.id = balance.sender_id
    // WHERE receiver_id = :receiver_id
    // ORDER BY :order_by :order
    fn list_tokens_by_user(
        &mut self,
        _receiver: UserQueryModeStrict,
        _order: Order,
        _order_by: Option<OrderByTokenOrSenderOrAmount>,
    ) -> Result<Vec<RelativeTokenAmountEntry>, ()>;

    // SELECT sender.*, receiver.*, balance.amount
    // FROM user_balance AS balance
    // JOIN user AS sender ON sender.id = balance.sender_id
    // JOIN user AS receiver ON receiver.id = balance.receiver_id
    // WHERE balance.token_id = :token_id
    // ORDER BY :order_by :order
    fn list_users_by_token(
        &mut self,
        _token: TokenQueryModeStrict,
        _order: Order,
        _order_by: Option<OrderByReceiverOrSenderOrAmount>,
    ) -> Result<Vec<RelativeUserAmountEntry>, ()>;
}
