#[macro_export]
macro_rules! handle_op {
    ($self:ident, $result:ident, $op:pat, $action:block) => {
        if let Some(Token::Operator($op)) = $self.get_next_token() {
            $action
        } else {
            $self.retreat();
        }
    };
}