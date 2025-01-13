use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, app::Mode, config::Config, theme::THEME};

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
    fn mode(&mut self) -> Option<Mode> {
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
        let page = Rect::new(0, 0, area.width, area.height).inner(Margin {
            horizontal: 0,
            vertical: 2,
        });
        let tab = Tabs::new(vec![Mode::Home.to_string(), Mode::Note.to_string()])
            .select(1)
            .highlight_style(THEME.tabs_selected);
        let [top, bottom] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)]).areas(page);
        let block = Block::default()
            .title(" Notes ")
            .title_position(block::Position::Top)
            .title_alignment(Alignment::Center)
            .title_style(THEME.app_title)
            .padding(Padding {
                left: 2,
                right: 2,
                top: 2,
                bottom: 2,
            })
            .borders(Borders::ALL);
        let bottom_block = Block::bordered()
            .title(" Text ")
            .title_alignment(Alignment::Center)
            .title_style(THEME.app_title);

        let text = Paragraph::new("A lot of notes")
            .alignment(Alignment::Center)
            .block(block);
        frame.render_widget(tab, area);
        frame.render_widget(text, top);
        frame.render_widget(bottom_block, bottom);
        Ok(())
    }
}
