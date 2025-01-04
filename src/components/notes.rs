use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, app::Mode, config::Config};

#[derive(Default)]
pub struct Note {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl Note {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Note {
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
        Some(Mode::Note)
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
        frame.render_widget(Paragraph::new("This is a note").centered(), area);
        Ok(())
    }
}
