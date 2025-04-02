use color_eyre::Result;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text;
use ratatui::widgets::{Block, Borders, Tabs};
use ratatui::{DefaultTerminal, Frame};

fn render(frame: &mut Frame) {
    let titles = ["Nodes", "Devices", "Jobs"]
        .iter()
        .cloned()
        .map(text::Line::from);
    let tabs = Tabs::new(titles)
        .block(Block::default().title("Tabs").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(2);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(frame.area());
    let block = Block::default().title("Block").borders(Borders::ALL);
    frame.render_widget(tabs, chunks[0]);
    let block = Block::default().title("Block 2").borders(Borders::ALL);
    frame.render_widget(block, chunks[1]);
}

fn runtime(mut terminal: DefaultTerminal) -> Result<()> {
    let exit_hotkey = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(exit_hotkey)) {
            return Ok(());
        }
    }
}

pub fn start() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = runtime(terminal);
    ratatui::restore();
    result
}
