use crate::link::LinkType;
use nwd::NwgUi;
use regex_lite::Regex;
use std::{cell::RefCell, sync::LazyLock};

#[derive(Default)]
pub struct NewLinkData {
    r#type: LinkType,
}

#[derive(Default, NwgUi)]
pub struct NewLinkUi {
    #[nwg_control(size: (500, 500), position: (300, 300), title: "Link a new YouTube resource", flags: "WINDOW|VISIBLE" )]
    window: nwg::Window,

    #[nwg_layout(parent: window)]
    layout: nwg::GridLayout,

    #[nwg_control(collection: vec!["Account"], selected_index: Some(0), focus: true)]
    #[nwg_events( OnComboxBoxSelection: [NewLinkUi::update_new_link_selection] )]
    #[nwg_layout_item(layout: layout, col: 0, row: 0)]
    new_link_selection: nwg::ComboBox<&'static str>,

    // linking an account
    #[nwg_control(flags: "VISIBLE")]
    #[nwg_layout_item(layout: layout, col: 1, row: 0)]
    account_frame: nwg::Frame,

    #[nwg_layout(parent: account_frame)]
    account_layout: nwg::GridLayout,

    #[nwg_control(text: "Insert the YouTube OAuth code here")]
    #[nwg_layout_item(layout: account_layout, col: 0, row: 0)]
    oauth_label: nwg::Label,

    #[nwg_control(text: "Insert the YouTube OAuth code here")]
    #[nwg_events( OnTextInput: [NewLinkUi::update_oauth_input])]
    #[nwg_layout_item(layout: account_layout, col: 1, row: 0)]
    oauth_input: nwg::TextInput,

    #[nwg_control(text: "OK", enabled: false)]
    #[nwg_events( OnButtonClick: [NewLinkUi::ok] )]
    #[nwg_layout_item(layout: layout, col: 0, row: 1)]
    ok_button: nwg::Button,

    #[nwg_control(text: "Cancel")]
    #[nwg_events( OnButtonClick: [NewLinkUi::cancel] )]
    #[nwg_layout_item(layout: layout, col: 0, row: 1)]
    cancel_button: nwg::Button,

    data: RefCell<NewLinkData>,
}

impl NewLinkUi {
    fn cancel(&self) {
        self.window.close();
        nwg::stop_thread_dispatch();
    }

    fn ok(&self) {
        self.data.borrow_mut().r#type = LinkType::Account(self.oauth_input.text());
        self.window.close();
        nwg::stop_thread_dispatch();
    }

    pub fn get_link_type(&self) -> LinkType {
        self.data.borrow().r#type.clone()
    }

    fn update_new_link_selection(&self) {
        let value = self.new_link_selection.selection_string();
    }

    fn update_oauth_input(&self) {
        static OAUTH: LazyLock<Regex> =
            LazyLock::new(|| Regex::new("AIza[0-9A-Za-z-_]{35}").unwrap());

        if OAUTH.is_match(&self.oauth_input.text()) {
            self.ok_button.set_enabled(true)
        } else {
            self.ok_button.set_enabled(false)
        }
    }
}
