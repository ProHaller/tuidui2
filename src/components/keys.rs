use std::time::Instant;

use color_eyre::Result;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Span,
    widgets::Paragraph,
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;

use super::Component;

use crate::{action::Action, app::Mode, config::Config};

#[derive(Default)]
pub struct Keys {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    mode: Mode,
}

impl Keys {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Keys {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn mode(&mut self) -> Option<Mode> {
        Some(self.mode)
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Switch => {
                self.mode.switch();
                self.mode();
                info!("Switched mode to: {}", self.mode);
            }
            _ => (),
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let [top, _] = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(area);
        let message = format!("Mode : {:?}", &self.mode());
        let span = Span::styled(message, Style::new().dim());
        let paragraph = Paragraph::new(span).centered();
        frame.render_widget(paragraph, top);
        Ok(())
    }
}
