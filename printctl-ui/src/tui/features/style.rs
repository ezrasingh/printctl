use ratatui::style::{Color, Modifier, Style};

pub fn opcode_style(is_selected: bool) -> Style {
    let base = Style::default().fg(Color::Green);
    if is_selected {
        base.bg(Color::DarkGray).add_modifier(Modifier::BOLD)
    } else {
        base
    }
}

pub fn arg_style(is_selected: bool) -> Style {
    let base = Style::default().fg(Color::Blue);
    if is_selected {
        base.bg(Color::DarkGray).add_modifier(Modifier::BOLD)
    } else {
        base
    }
}

pub fn value_style(is_selected: bool) -> Style {
    let base = Style::default();
    if is_selected {
        base.fg(Color::White)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD)
    } else {
        base.fg(Color::LightYellow)
    }
}

pub fn comment_style(is_selected: bool) -> Style {
    let base = Style::default().fg(Color::DarkGray);
    if is_selected {
        base.bg(Color::Gray)
            .add_modifier(Modifier::BOLD & Modifier::ITALIC)
    } else {
        base.add_modifier(Modifier::ITALIC)
    }
}

pub fn gutter_style(is_selected: bool) -> Style {
    let base = Style::default();
    if is_selected {
        base.fg(Color::Yellow)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD)
    } else {
        base.fg(Color::DarkGray)
    }
}
