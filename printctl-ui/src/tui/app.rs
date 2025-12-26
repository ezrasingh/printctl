use std::sync::mpsc;

use color_eyre::eyre;
use ratatui::{DefaultTerminal, Frame};

use super::input::{AppEvent, EventHandler};
use super::state::AppState;

pub struct App {
    exit: bool,
    state: AppState,
    prev_state: Option<AppState>,
    channel: (mpsc::Sender<AppEvent>, mpsc::Receiver<AppEvent>),
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: false,
            prev_state: None,
            state: AppState::default(),
            channel: mpsc::channel::<AppEvent>(),
        }
    }
}

impl App {
    fn state(&self) -> &AppState {
        &self.state
    }

    fn tx(&self) -> &mpsc::Sender<AppEvent> {
        &self.channel.0
    }

    fn rx(&self) -> &mpsc::Receiver<AppEvent> {
        &self.channel.1
    }

    fn emitter(&self) -> mpsc::Sender<AppEvent> {
        self.tx().clone()
    }

    fn close(&mut self) {
        self.exit = true;
    }

    fn go_back(&mut self) {
        if let Some(prev) = self.prev_state.take() {
            self.state = prev;
        }
    }

    fn set_state(&mut self, next_state: AppState) {
        let prev = std::mem::replace(&mut self.state, next_state);
        self.prev_state = Some(prev);
    }
}

impl App {
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> eyre::Result<()> {
        AppEvent::start_term_event_thread(self.emitter());

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            match self.rx().try_recv() {
                Err(mpsc::TryRecvError::Empty) => continue,
                Err(mpsc::TryRecvError::Disconnected) => break,
                Ok(app_event) => self.handle_app_event(app_event, self.emitter())?,
            };
        }

        Ok(())
    }
}

use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Buffer, Rect, Widget};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, BorderType};

impl App {
    fn title() -> String {
        format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    }

    fn layout(area: Rect) -> [Rect; 2] {
        let [layout_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(area);

        let [content_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(layout_area);

        [layout_area, content_area]
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [layout_area, content_area] = App::layout(area);

        Block::bordered()
            .title(App::title())
            .title_alignment(ratatui::layout::Alignment::Center)
            .border_type(BorderType::Rounded)
            .fg(Color::Yellow)
            .render(layout_area, buf);

        self.state().render(content_area, buf);
    }
}

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

impl EventHandler for App {
    fn handle_key_event(&mut self, key_event: &KeyEvent) -> Option<AppEvent> {
        match key_event.code {
            KeyCode::Esc => self.close(),
            KeyCode::Char('Q') | KeyCode::Char('q') => self.close(),
            KeyCode::Char('B') | KeyCode::Char('b') => self.go_back(),
            _ => {}
        };
        None
    }

    fn handle_app_event(
        &mut self,
        app_event: AppEvent,
        app_emitter: mpsc::Sender<AppEvent>,
    ) -> eyre::Result<()> {
        if let AppEvent::SetState(state) = app_event {
            self.set_state(state);
            return Ok(());
        }

        match &app_event {
            AppEvent::Input(key_event) => {
                if key_event.kind == KeyEventKind::Press {
                    if let Some(event) = self.handle_key_event(key_event) {
                        app_emitter.send(event)?;
                    }
                }
            }
            _ => {}
        }
        self.state.handle_app_event(app_event, app_emitter)
    }
}
