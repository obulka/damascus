use iced_native::Widget;
use crate::update::WidgetUpdate;
use crate::view::WidgetView;

pub mod tab;
pub use tab::Tab;

pub trait WidgetModel<EmittedMessage, Renderer: iced_native::Renderer>: WidgetUpdate<EmittedMessage, Renderer> + WidgetView<EmittedMessage, Renderer> + Widget<EmittedMessage, Renderer> {}
