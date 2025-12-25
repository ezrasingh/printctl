use std::sync::mpsc;

use color_eyre::eyre;
use crossterm::event::{Event, KeyEvent, KeyEventKind, MouseEvent};

use super::state::AppState;

#[derive(Debug)]
pub enum AppEvent {
    SetState(AppState),
    Input(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Paste(String),
    Focus(bool),
}

impl From<&Event> for AppEvent {
    fn from(term_event: &Event) -> Self {
        match term_event {
            Event::Key(key_event) => AppEvent::Input(*key_event),
            Event::Paste(text) => AppEvent::Paste(text.to_owned()),
            Event::Mouse(mouse_event) => AppEvent::Mouse(*mouse_event),
            Event::Resize(width, height) => AppEvent::Resize(*width, *height),
            Event::FocusGained => AppEvent::Focus(true),
            Event::FocusLost => AppEvent::Focus(false),
        }
    }
}

impl AppEvent {
    pub fn start_term_event_thread(tx: mpsc::Sender<Self>) {
        std::thread::spawn(move || {
            loop {
                match crossterm::event::read() {
                    Err(error) => {
                        eprintln!("Could not read terminal event: {error}");
                        break;
                    }
                    Ok(term_event) => {
                        let app_event = AppEvent::from(&term_event);
                        if let Err(error) = tx.send(app_event) {
                            eprintln!("Send AppEvent failed: {error}");
                            break;
                        }
                    }
                };
            }
        });
    }
}

pub trait EventHandler {
    fn handle_key_event(&mut self, _: &KeyEvent) -> Option<AppEvent> {
        None
    }

    fn handle_app_event(
        &mut self,
        app_event: AppEvent,
        app_emitter: mpsc::Sender<AppEvent>,
    ) -> eyre::Result<()> {
        if let AppEvent::Input(key_event) = &app_event {
            if key_event.kind == KeyEventKind::Press {
                if let Some(event) = self.handle_key_event(key_event) {
                    app_emitter.send(event)?;
                }
            }
        }
        Ok(())
    }
}
