use tokio::sync::{oneshot, mpsc};


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
    fn handle_event(event: Event, model: &mut Model, responder: oneshot::Sender<View>)
    {
        let proposal = match event {
            Event::MouseClick {..} => Action::CreateWidget{widget: Widget{}},
            Event::NothingHappend => Action::CloseWidget{widget: Widget{}},
        };
        model.submit(proposal, responder);
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

    fn submit(&mut self, proposal: Action, responder: oneshot::Sender<View>)
    {
        match proposal {
            Action::CreateWidget{widget} => self.create_widget(widget),
            Action::CloseWidget{widget} => self.close_widget(widget),
        }
        State::compute_control_state(self, responder);
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

#[derive(Debug)]
struct View
{
    view_str: String,
}


struct State {}

impl State {

    fn compute_control_state(model: &Model, responder: oneshot::Sender<View>)
    {
        if !State::next_action_predicate(model)
        {
            responder.send(State::render(model)).unwrap();
        }
    }

    fn render(model: &Model) -> View
    {
        View{view_str: format!("{:?}", model)}
    }

    fn next_action_predicate(model: &Model) -> bool
    {
        if model.widgets.len() > 1
        {
            println!("Shit thats long");
        }
        false
    }
}


#[tokio::main]
async fn main()
{
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<(Event, oneshot::Sender<View>)>(100);

    let mut model = Model::new();

    tokio::spawn(
        async move
        {
            while let Some((event, responder)) = cmd_rx.recv().await
            {
                Action::handle_event(event, &mut model, responder);
            }
        }
    );

    let events = vec![
        Event::NothingHappend,
        Event::NothingHappend,
        Event::MouseClick{x_position: 32, y_position: 45},
        Event::NothingHappend,
        Event::NothingHappend,
        Event::NothingHappend,
        Event::MouseClick{x_position: 32, y_position: 45},
    ];

    let mut join_handles = vec![];

    // Spawn tasks that will send the increment command.
    for event in events
    {
        let mut cmd_tx = cmd_tx.clone();

        join_handles.push(tokio::spawn(
            async move
            {
                let (resp_tx, resp_rx) = oneshot::channel();

                cmd_tx.send((event, resp_tx)).await.ok().unwrap();
                let res = resp_rx.await.unwrap();

                println!("previous value = {:?}", res);
            }
        ));
    }

    // Wait for all tasks to complete
    for join_handle in join_handles.drain(..)
    {
        join_handle.await.unwrap();
    }
}
