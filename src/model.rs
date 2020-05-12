use crate::action::Message;

pub trait Model
{
    fn new() -> Self;

    fn accept_proposal(&mut self, proposal: Message);

    fn submit(&mut self, proposal: Message);

}
