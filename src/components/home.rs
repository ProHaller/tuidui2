use std::{
    collections::{hash_map, hash_set, HashMap},
    fmt::format,
};

use color_eyre::Result;
use crossterm::event::KeyModifiers;
use itertools::Itertools;
use libc::group;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;
use tui_markdown::*;

use super::Component;
use crate::{
    action::Action,
    app::Mode,
    config::{Config, KeyBindings},
    theme::THEME,
};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    // This should set the mode to its own.
    fn mode(&self) -> Option<Mode> {
        Some(Mode::Home)
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let welcome_message = r#"
## Welcome
Welcome, intrepid task conqueror, to the most epic quest of productivity you've ever embarked upon: the Rusty Ratatui To-Do App! 🦀🍝

Picture this: a world where your tasks are tamed, your errands are executed, and your chores are... well, less chore-like. All thanks to the magical powers of Rust and the whimsical wonders of Ratatui!

### Why Rust? Why Ratatui? Why Not?

Rust is the fearless guardian of memory safety, ensuring your tasks are managed with the utmost precision and zero segfaults! And Ratatui? It's the delightful dish of terminal user interfaces, serving up your tasks with a side of style and a sprinkle of panache.

### Features That Will Make You Go "Wow!"

- **Task Slaying**: _Add tasks_, _delete tasks_, and _mark them as done_ with the swipe of your keyboard. It's like a sword fight, but with less danger and more productivity.
- **Colorful Chaos**: Watch your tasks light up your terminal in vibrant colors, like a rainbow after a storm of procrastination.
- **Epic Simplicity**: No need for a PhD in rocket science to use this app. Just your wits, your keyboard, and a thirst for task domination.
- **Instant Gratification**: Feel the sweet, sweet satisfaction of checking off tasks. It's like popping bubble wrap, but for your to-do list.
"#;
        let formated_welcome_message = from_str(welcome_message);
        let block = Block::default()
            .title(format!(" {} ", self.mode().unwrap()))
            .title_position(block::Position::Top)
            .title_alignment(Alignment::Center)
            .title_style(THEME.app_title)
            .padding(Padding {
                left: 2,
                right: 2,
                top: 5,
                bottom: 5,
            })
            .borders(Borders::ALL);

        let bindings = &self.config.keybindings[&self.mode().unwrap()];

        let mut grouped_bindings = HashMap::new();
        for (key_events, action) in bindings {
            for key_event in key_events {
                match key_event.modifiers {
                    KeyModifiers::NONE => {
                        let conf = format!("<{}>", key_event.code);
                        grouped_bindings
                            .entry(action.to_string())
                            .or_insert(Vec::new())
                            .push(conf);
                    }
                    KeyModifiers::CONTROL => {
                        let conf = format!("<C-{}>", key_event.code);
                        grouped_bindings
                            .entry(action.to_string())
                            .or_insert(Vec::new())
                            .push(conf);
                    }
                    _ => {}
                }
            }
        }

        let mut spans = Vec::new();
        let ordered_keys = grouped_bindings.keys().sorted().collect_vec();
        for ordered_key in ordered_keys {
            if let Some(keys) = grouped_bindings.get(ordered_key) {
                let key = keys.join(" | ");
                let key_span = Span::styled(format!(" {} ", key), THEME.key_binding.key);
                let desc_span =
                    Span::styled(format!(" {} ", ordered_key), THEME.key_binding.description);
                spans.push(desc_span);
                spans.push(key_span);
            }
        }

        let page = Rect::new(0, 0, area.width, area.height).inner(Margin {
            horizontal: 0,
            vertical: 2,
        });
        let [top, bottom] = Layout::vertical([Constraint::Fill(9), Constraint::Min(5)]).areas(page);
        let line = Line::from(spans).centered().style(THEME.borders);
        let tab = Tabs::new(vec![Mode::Home.to_string(), Mode::Note.to_string()])
            .select(0)
            .highlight_style(THEME.tabs_selected);

        let bottom_block = Block::bordered()
            .title(" Key Bindings ")
            .title_alignment(Alignment::Center)
            .padding(Padding::new(1, 1, 1, 0))
            .title_style(THEME.app_title);
        let keys = Paragraph::new(line)
            .alignment(Alignment::Center)
            .block(bottom_block);

        let text = Paragraph::new(formated_welcome_message)
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Left)
            .block(block);

        frame.render_widget(tab, area);
        frame.render_widget(text, top);
        frame.render_widget(keys, bottom);

        Ok(())
    }
}
