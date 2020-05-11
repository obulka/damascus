pub mod actions
{
    pub enum Intent
    {
        Increment,
    }

    pub fn propose(intent: Intent, data: Data)
    {
        match intent {
            Intent.Increment => count_by(data.step),
        };
    }

    fn count_by(step: i32, model: &mut super::Model)
    {
        model.accept(Proposal {increment_by: step});
    }

}

pub struct Data
{
    step: i32,
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
        state::render(self);
    }
}


struct View
{
    intents: Intent,
    output: String,
}

impl View
{
    fn new(intent: Intent, model: Model) -> View
    {
        let view = View {intents: intents};
        view.ready(model, intents)
    }

    fn ready(model: mut Model, intent: Intent) -> View
    {
        model.last_edited = match model.last_edited {
            Some(last_edited) => ,
            None => ,
        }
    }

    fn display(representation, next_action)
    {
        match next_action {
            Some(action) => action,
            None => ,
        }
    }
}

enum Intent
{
    Increment,
}

struct Data
{
    step: i32,
}

struct Actions
{
    present: fn(Data, Option<Intent>),
}

impl Actions
{
    fn new(present: fn(Data, Option<Intent>)) -> Actions
    {
        Actions {present: present}
    }

    fn increment(&self, data: Data, next_action: Option<Intent>) -> bool
    {
        (self.present)(data, next_action);
        false
    }

    fn propose(&self, data: EditableData, intent: Intent)
    {
        match intent {
            Intent::Increment => self.save(data, None),
        };
    }
}

trait Presentable
{
    fn present(data: EditableData, next_action: Option<Actions>) -> bool;
}

impl Presentable for Actions
{
    fn present(data: EditableData, next_action: Option<Actions>) -> bool
    {
        false
    }
}



struct State
{
    view: View,
}

impl State
{
    fn display(representation: , next_action: Option<Intent>);
    fn render(model: Model, next_action: Option<Intent>)
    {
        representation(model, next_action);
        next_action(model);
    }
    fn representation(model: Model, next_action: Option<Intent>);
}


struct Model
{
    render: fn(&Model, Option<Intent>),
    counter: i32,
}

impl Model
{
    fn new(render: fn(&Model, Option<Intent>), counter: i32) -> Model
    {
        Model {render: render, counter: counter}
    }

    fn present(&self, data: Data, next_action: Option<Intent>)
    {
        (self.render)(&self, next_action);
    }
}


trait Actionable
{
    fn next_action();
    fn propose();
}

struct Safe
{
    state: State,
    actions: Actions,
    model: Model,
    view: View,
    blocked: bool,
}

impl Safe
{
    fn new(state: State, actions: Actions, model: Model, view: View) -> Safe
    {

        Safe {
            state: state,
            actions: actions,
            model: model,
            view: view,
            blocked: false,
        }
    }

    fn present(data: Data, next_action: Option<Intent>)
    {

    }

    fn dispatcher()
    {

    }

    fn dispatch()
    {

    }

    fn render(model: &Model, next_action: Option<Intent>)
    {
        println!("{:?}", model.counter);

    }
}
