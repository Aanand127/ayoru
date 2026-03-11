use crate::tui::state::{Mode, TuiState};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

pub fn render_to_buffer(state: &TuiState, width: u16, height: u16) -> Buffer {
    let area = Rect::new(0, 0, width, height);
    let mut buffer = Buffer::empty(area);
    render(state, area, &mut buffer);
    buffer
}

pub fn render(state: &TuiState, area: Rect, buffer: &mut Buffer) {
    let view = build_view(state, area);
    Paragraph::new(view.header)
        .block(Block::default().borders(Borders::ALL).title("ani tui"))
        .render(view.sections[0], buffer);
    Paragraph::new(view.body)
        .block(Block::default().borders(Borders::ALL).title("Main"))
        .render(view.sections[1], buffer);
    Paragraph::new(view.footer)
        .block(Block::default().borders(Borders::ALL).title("Hints"))
        .render(view.sections[2], buffer);
}

pub fn draw(frame: &mut Frame<'_>, state: &TuiState) {
    let area = frame.area();
    let view = build_view(state, area);
    frame.render_widget(
        Paragraph::new(view.header).block(Block::default().borders(Borders::ALL).title("ani tui")),
        view.sections[0],
    );
    frame.render_widget(
        Paragraph::new(view.body).block(Block::default().borders(Borders::ALL).title("Main")),
        view.sections[1],
    );
    frame.render_widget(
        Paragraph::new(view.footer).block(Block::default().borders(Borders::ALL).title("Hints")),
        view.sections[2],
    );
}

fn search_lines(state: &TuiState) -> Vec<Line<'static>> {
    if state.results.is_empty() {
        return vec![Line::from("Type a query and press Enter")];
    }

    state
        .results
        .iter()
        .enumerate()
        .map(|(idx, title)| {
            if idx == state.selected_result {
                Line::from(vec![
                    Span::styled("> ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(title.name.clone()),
                ])
            } else {
                Line::from(format!("  {}", title.name))
            }
        })
        .collect()
}

fn episode_lines(state: &TuiState) -> Vec<Line<'static>> {
    if state.episodes.is_empty() {
        return vec![Line::from("No episodes loaded")];
    }

    state
        .episodes
        .iter()
        .enumerate()
        .map(|(idx, episode)| {
            let label = format!("Episode {}", episode.number);
            if idx == state.selected_episode {
                Line::from(vec![
                    Span::styled("> ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(label),
                ])
            } else {
                Line::from(format!("  {label}"))
            }
        })
        .collect()
}

struct View {
    header: String,
    body: Vec<Line<'static>>,
    footer: String,
    sections: Vec<Rect>,
}

fn build_view(state: &TuiState, area: Rect) -> View {
    let sections = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
    .split(area)
    .to_vec();

    let title = match state.mode {
        Mode::Search => "Search",
        Mode::Episodes => "Episodes",
        Mode::Launching => "Launching",
    };

    let body = match state.mode {
        Mode::Search => search_lines(state),
        Mode::Episodes => episode_lines(state),
        Mode::Launching => vec![Line::from("Launching playback...")],
    };

    let footer = match &state.message {
        Some(message) => format!("{message}  Enter select  Esc back  q quit"),
        None => "Enter select  Esc back  q quit".to_string(),
    };

    View {
        header: format!("{title}  Query: {}", state.query),
        body,
        footer,
        sections,
    }
}
