use std::sync::mpsc;

use super::features::editor::GCodeEditor;
use super::input::{AppEvent, EventHandler};

#[derive(Debug)]
pub enum AppState {
    Home,
    GcodeWorkbench(GCodeEditor),
}

impl Default for AppState {
    fn default() -> Self {
        Self::Home
    }
}

use super::components::layout::{Modal, StackedLayout};

impl AppState {
    fn open_gcode_workbech() -> AppEvent {
        let editor = GCodeEditor::default();
        AppEvent::SetState(Self::GcodeWorkbench(editor))
    }

    fn home_screen(&self) -> impl Widget {
        StackedLayout::new()
            .header(Paragraph::new("Home"))
            .content(Modal::new("Welcome").content(Paragraph::new("This is printctl")))
            .footer(
                Paragraph::new("[J] Jobs [G] Gcode Editor [Q] Quit").alignment(Alignment::Center),
            )
    }
}

use ratatui::layout::Alignment;
use ratatui::prelude::{Buffer, Rect};
use ratatui::widgets::{Paragraph, Widget};

impl Widget for &AppState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            AppState::Home => self.home_screen().render(area, buf),
            AppState::GcodeWorkbench(editor) => editor.render(area, buf),
        }
    }
}

use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyEventKind};

impl EventHandler for AppState {
    fn handle_key_event(&mut self, key_event: &crossterm::event::KeyEvent) -> Option<AppEvent> {
        match self {
            AppState::Home => match key_event.code {
                KeyCode::Char('G') | KeyCode::Char('g') => Self::open_gcode_workbech().into(),
                _ => None,
            },
            _ => None,
        }
    }

    fn handle_app_event(
        &mut self,
        app_event: AppEvent,
        app_emitter: mpsc::Sender<AppEvent>,
    ) -> eyre::Result<()> {
        if let AppEvent::Input(key_event) = &app_event {
            if key_event.kind == KeyEventKind::Press {
                if let Some(app_event) = self.handle_key_event(key_event) {
                    app_emitter.send(app_event)?;
                }
            }
        }

        match self {
            AppState::GcodeWorkbench(editor) => editor.handle_app_event(app_event, app_emitter),
            _ => Ok(()),
        }
    }
}
