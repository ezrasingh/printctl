mod code;
mod debugger;
mod script;
mod style;

use ratatui_explorer::{FileExplorer, Theme};

use crate::tui::components::layout::StackedLayout;
use crate::tui::input::{AppEvent, EventHandler};

pub use script::GCodeScript;

#[derive(Debug)]
pub struct GCodeEditor {
    file_explorer: FileExplorer,
    selected_script: Option<GCodeScript>,
}

impl Default for GCodeEditor {
    fn default() -> Self {
        let file_explorer = {
            let theme = Theme::default().add_default_title();
            FileExplorer::with_theme(theme).expect("Could not load FileExplorer")
        };

        Self {
            file_explorer,
            selected_script: None,
        }
    }
}

impl GCodeEditor {
    fn remove_file(&mut self) {
        self.selected_script.take();
    }

    fn select_current_file(&mut self) {
        let path = self.file_explorer.current().path();
        if let Some(ext) = path.extension() {
            if matches!(ext.to_ascii_lowercase().to_str(), Some("gcode")) {
                let script = GCodeScript::new(path);
                self.selected_script.replace(script);
            }
        }
    }
}

use ratatui::layout::Alignment;
use ratatui::widgets::{Paragraph, Widget};

impl Widget for &GCodeEditor {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let layout = StackedLayout::new();
        match &self.selected_script {
            None => layout
                .header(Paragraph::new("Select GCode"))
                .content(&self.file_explorer.widget())
                .footer(Paragraph::new("[B] Go Back [Q] Quit").alignment(Alignment::Center))
                .render(area, buf),
            Some(script) => layout
                .header(Paragraph::new("[F] Choose File"))
                .content(script)
                .footer(
                    Paragraph::new("[B] Go Back [H] Help [Q] Quit").alignment(Alignment::Center),
                )
                .render(area, buf),
        }
    }
}

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui_explorer::Input as ExplorerInput;

impl Into<ExplorerInput> for AppEvent {
    fn into(self) -> ExplorerInput {
        match &self {
            AppEvent::Input(key) => match key.code {
                KeyCode::End => ExplorerInput::End,
                KeyCode::Home => ExplorerInput::Home,
                KeyCode::PageUp => ExplorerInput::PageUp,
                KeyCode::PageDown => ExplorerInput::PageDown,
                KeyCode::Char('K') | KeyCode::Char('k') | KeyCode::Up => ExplorerInput::Up,
                KeyCode::Char('J') | KeyCode::Char('j') | KeyCode::Down => ExplorerInput::Down,
                KeyCode::Char('H') | KeyCode::Char('h') | KeyCode::Left | KeyCode::Backspace => {
                    ExplorerInput::Left
                }
                KeyCode::Char('L') | KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                    ExplorerInput::Right
                }
                _ => ExplorerInput::None,
            },
            _ => ExplorerInput::None,
        }
    }
}

impl EventHandler for GCodeEditor {
    fn handle_key_event(&mut self, key_event: &crossterm::event::KeyEvent) -> Option<AppEvent> {
        match key_event.code {
            KeyCode::Char('F') | KeyCode::Char('f') => self.remove_file(),
            KeyCode::Enter => {
                if self.selected_script.is_none() {
                    self.select_current_file();
                }
            }
            _ => {}
        }
        None
    }

    fn handle_app_event(
        &mut self,
        app_event: AppEvent,
        app_emitter: std::sync::mpsc::Sender<AppEvent>,
    ) -> color_eyre::eyre::Result<()> {
        if let AppEvent::Input(key_event) = &app_event {
            if key_event.kind == KeyEventKind::Press {
                if let Some(app_event) = self.handle_key_event(key_event) {
                    app_emitter.send(app_event)?;
                }
            }
        }

        match &mut self.selected_script {
            Some(script) => script.handle_app_event(app_event, app_emitter),
            None => {
                self.file_explorer.handle(app_event)?;
                Ok(())
            }
        }
    }
}
