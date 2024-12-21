trait DbApi
{
    // INSERT INTO user(name) VALUES(:name)
    // LAST ID
    fn create_user(_name: &str) -> UserListEntry
    {
        todo!();
    }

    // SELECT * FROM user
    // SELECT * FROM user WHERE name LIKE '%:name%'
    fn query_user(_name: Option<&str>) -> Vec<UserListEntry>
    {
        todo!();
    }

    // INSERT INTO token(name) VALUES(:name)
    // LAST ID
    fn create_token(_name: &str) -> TokenListEntry
    {
        todo!();
    }

    // SELECT * FROM token
    // SELECT * FROM token WHERE name LIKE '%:name%'
    fn query_token(_name: Option<&str>) -> Vec<TokenListEntry>
    {
        todo!();
    }

    // SELECT current_total FROM user_balance WHERE sender_id = :sender_id, receiver_id = :receiver_id, token_id = :token_id
    // INSERT INTO transaction_history(sender_id, receiver_id, token_id, amount) VALUES(:sender_id, :receiver_id, :token_id, :amount)
    fn transaction(
        sender: UserQueryModeWithCreation,
        receiver: UserQueryModeWithCreation,
        token: TokenQueryModeWithCreation,
        amount: TokenAmount,
    ) -> Result<TokenAmount, ()>
    {
        todo!();
    }

    // SELECT sender.*, balance.mount
    // FROM user_balance AS balance
    // JOIN user AS sender ON sender.id = balance.sender_id
    // WHERE receiver_id = :receiver_id, token_id = :token_id
    // ORDER BY :order_by :order
    fn list_user_token(
        receiver: UserQueryModeStrict,
        token: TokenQueryModeStrict,
        _order: Order,
        _order_by: Option<OrderBySenderOrAmount>,
    ) -> Result<Vec<RelativeUserTokenAmountEntry>, ()>
    {
        todo!();
    }

    // SELECT sender.*, token.*, balance.amount
    // FROM user_balance AS balance
    // JOIN user AS sender ON sender.id = balance.sender_id
    // WHERE receiver_id = :receiver_id
    // ORDER BY :order_by :order
    fn list_tokens_by_user(
        receiver: UserQueryModeStrict,
        _order: Order,
        _order_by: Option<OrderByTokenOrSenderOrAmount>,
    ) -> Result<Vec<RelativeTokenAmountEntry>, ()>
    {
        todo!();
    }

    // SELECT sender.*, receiver.*, balance.amount
    // FROM user_balance AS balance
    // JOIN user AS sender ON sender.id = balance.sender_id
    // JOIN user AS receiver ON receiver.id = balance.receiver_id
    // WHERE balance.token_id = :token_id
    // ORDER BY :order_by :order
    fn list_users_by_token(
        token: TokenQueryModeStrict,
        _order: Order,
        _order_by: Option<OrderByReceiverOrSenderOrAmount>,
    ) -> Result<Vec<RelativeUserAmountEntry>, ()>
    {
        todo!();
    }
}
