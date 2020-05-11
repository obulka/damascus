#[derive(Debug, Clone, Copy)]
enum Event {
    MouseClick {x_position: i32, y_position: i32},
    NothingHappend,
}

#[derive(Debug)]
struct Widget {}


enum Action {
    CreateWidget {widget: Widget},
    CloseWidget {widget: Widget},
}

impl Action
{
    fn propose(proposal: Action, model: &mut Model)
    {
        model.submit(proposal);
    }

    fn handle_event(event: Event, model: &mut Model)
    {
        let proposal = match event {
            Event::MouseClick {..} => Action::CreateWidget{widget: Widget{}},
            Event::NothingHappend => Action::CloseWidget{widget: Widget{}},
        };
        Action::propose(proposal, model);
    }
}

#[derive(Debug)]
struct Model
{
    widgets: Vec<Widget>,
}

impl Model
{
    fn new() -> Model
    {
        Model {widgets: Vec::new()}
    }

    fn submit(&mut self, proposal: Action)
    {
        match proposal {
            Action::CreateWidget{widget} => self.create_widget(widget),
            Action::CloseWidget{widget} => self.close_widget(widget),
        }
        State::compute_control_state(self);
    }

    fn create_widget(&mut self, widget: Widget)
    {
        self.widgets.push(widget);
    }

    fn close_widget(&mut self, widget: Widget)
    {
        println!("{:?}", self.widgets);
        println!("Deleting {:?}", widget);
    }
}


struct State {}

impl State {

    fn compute_control_state(model: &Model)
    {
        if !State::next_action_predicate(model)
        {
            State::render(model);
        }
    }

    fn render(model: &Model)
    {
        println!("{:?}", model);
    }

    fn next_action_predicate(model: &Model) -> bool
    {
        false
    }
}



fn main() {
    let mut model = Model::new();

    // Sequence of events (might be dynamical based on what State::run did)
    let events = [
        Event::NothingHappend,
        Event::NothingHappend,
        Event::MouseClick{x_position: 32, y_position: 45},
        Event::NothingHappend,
        Event::NothingHappend,
        Event::NothingHappend,
        Event::MouseClick{x_position: 32, y_position: 45},
    ];

    for event in events.iter()
    {
        Action::handle_event(*event, &mut model);
    }
}