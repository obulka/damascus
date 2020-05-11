
pub mod state_action_model
{
    pub mod actions
    {
        pub struct Proposal
        {
            pub increment_by: i32,
        }

        pub fn count_by(step: i32, model: &mut super::Model)
        {
            model.accept(Proposal {increment_by: step});
        }
    }

    pub mod state
    {
        pub fn render(model: &mut super::Model)
        {
            if !self::next_action(model)
            {
                println!("{:?}", model.counter);
            }
        }

        pub fn next_action(model: &mut super::Model) -> bool
        {
            if model.counter % 2 == 0
            {
                super::actions::count_by(1, model);
                return true;
            }
            false
        }
    }

    pub struct Model
    {
        counter: i32,
    }

    impl Model
    {
        pub fn new(counter: i32) -> Model
        {
            Model{counter: counter}
        }

        fn accept(&mut self, proposal: self::actions::Proposal)
        {
            self.counter += proposal.increment_by;
        }
    }
}
