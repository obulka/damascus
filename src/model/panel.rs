// 3rd Party Imports
use iced::{button, pane_grid};

// Local Imports
use crate::model::{
    tabs::{tab_content_from_type, TabContent, TabType},
    Model,
};
use crate::update::{panel::PanelUpdate, tabs::TabContentMessage};
use crate::view::panel::PanelView;

trait PanelModel: PanelUpdate + PanelView {}

pub struct Panel {
    pub pane: Option<pane_grid::Pane>,
    pub focus: bool,
    pub split_horizontally: button::State,
    pub split_vertically: button::State,
    pub float_pane: button::State, // Not Implemented
    pub close: button::State,
    pub tabs: Vec<(String, button::State)>,
    pub tab_contents: Vec<Box<dyn TabContent>>,
    pub focused_tab: usize,
}

impl Model<TabContentMessage> for Panel {}
impl PanelModel for Panel {}

impl Panel {
    pub fn new() -> Self {
        Self {
            pane: None,
            focus: false,
            split_horizontally: button::State::new(),
            split_vertically: button::State::new(),
            float_pane: button::State::new(),
            close: button::State::new(),
            tabs: Vec::new(),
            tab_contents: Vec::new(),
            focused_tab: 0,
        }
    }

    pub fn open_tab(&mut self, tab_type: TabType) {
        self.tabs
            .push((tab_type.clone().into(), button::State::new()));
        self.tab_contents.push(tab_content_from_type(tab_type));

        self.focused_tab = self.tabs.len() - 1;
    }

    pub fn open_tab_with_content(
        &mut self,
        tab: (String, button::State),
        tab_content: Box<dyn TabContent>,
    ) {
        self.tabs.push(tab);
        self.tab_contents.push(tab_content);
        self.focused_tab = self.tabs.len() - 1;
    }

    pub fn focus_tab(&mut self, tab_label: String) {
        for (index, (label, _)) in self.tabs.iter().enumerate() {
            if tab_label == *label {
                self.focused_tab = index;
                break;
            }
        }
    }

    pub fn close_tab(
        &mut self,
        tab_label: &String,
    ) -> (Option<String>, (String, button::State), Box<dyn TabContent>) {
        let current_focus = self.focused_tab;

        let mut close_index = 0;
        for (index, (label, _)) in self.tabs.iter().enumerate() {
            if tab_label == label {
                close_index = index;
                break;
            }
        }

        let tab = self.tabs.remove(close_index);
        let tab_content = self.tab_contents.remove(close_index);

        if self.tabs.is_empty() {
            return (None, tab, tab_content);
        }

        let mut new_focus = if current_focus > close_index {
            current_focus - 1
        } else {
            current_focus
        };
        while new_focus >= self.tabs.len() && new_focus >= 1 {
            new_focus -= 1;
        }
        let (new_focus_label, _) = &self.tabs[new_focus];
        (Some(new_focus_label.to_string()), tab, tab_content)
    }

    pub fn close_all_tabs(&mut self) {
        self.tabs.clear();
        self.tab_contents.clear();
    }

    pub fn index_of_tab_type(&self, tab_type: TabType) -> Option<usize> {
        let tab_string: String = tab_type.into();
        for (index, (label, _)) in self.tabs.iter().enumerate() {
            if *label == tab_string {
                return Some(index);
            }
        }
        None
    }

    pub fn get_focused_label(&self) -> Option<String> {
        if let Some((focused_label, _)) = self.tabs.get(self.focused_tab) {
            return Some(focused_label.to_string());
        }
        None
    }

    pub fn get_focused_content(&self) -> Option<&Box<dyn TabContent>> {
        self.tab_contents.get(self.focused_tab)
    }

    pub fn get_mut_focused_content(&mut self) -> Option<&mut Box<dyn TabContent>> {
        self.tab_contents.get_mut(self.focused_tab)
    }
}
