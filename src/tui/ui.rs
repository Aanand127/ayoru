use crate::tui::state::{Mode, TuiState};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Alignment;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

const GRAPHITE: Color = Color::Rgb(22, 24, 28);
const SOFT_CHARCOAL: Color = Color::Rgb(29, 33, 39);
const SMOKE: Color = Color::Rgb(38, 44, 52);
const WARM_SILVER: Color = Color::Rgb(231, 232, 234);
const MUTED_SILVER: Color = Color::Rgb(184, 190, 199);
const QUIET_GRAY: Color = Color::Rgb(139, 147, 161);
const STEEL_BLUE: Color = Color::Rgb(94, 116, 143);
const MIST_BLUE: Color = Color::Rgb(142, 163, 186);

pub fn render_to_buffer(state: &TuiState, width: u16, height: u16) -> Buffer {
    let area = Rect::new(0, 0, width, height);
    let mut buffer = Buffer::empty(area);
    render(state, area, &mut buffer);
    buffer
}

pub fn render(state: &TuiState, area: Rect, buffer: &mut Buffer) {
    let view = build_view(state, area);
    Paragraph::new(view.header)
        .block(styled_block(""))
        .alignment(Alignment::Left)
        .style(Style::default().fg(WARM_SILVER).bg(GRAPHITE))
        .render(view.sections[0], buffer);
    Paragraph::new(view.body)
        .block(styled_block(&view.body_title))
        .style(Style::default().fg(WARM_SILVER).bg(SOFT_CHARCOAL))
        .render(view.sections[1], buffer);
    Paragraph::new(view.footer)
        .block(styled_block("Status"))
        .style(Style::default().fg(QUIET_GRAY).bg(GRAPHITE))
        .render(view.sections[2], buffer);
}

pub fn draw(frame: &mut Frame<'_>, state: &TuiState) {
    let area = frame.area();
    let view = build_view(state, area);
    frame.render_widget(
        Paragraph::new(view.header)
            .block(styled_block(""))
            .style(Style::default().fg(WARM_SILVER).bg(GRAPHITE)),
        view.sections[0],
    );
    frame.render_widget(
        Paragraph::new(view.body)
            .block(styled_block(&view.body_title))
            .style(Style::default().fg(WARM_SILVER).bg(SOFT_CHARCOAL)),
        view.sections[1],
    );
    frame.render_widget(
        Paragraph::new(view.footer)
            .block(styled_block("Status"))
            .style(Style::default().fg(QUIET_GRAY).bg(GRAPHITE)),
        view.sections[2],
    );
}

fn search_lines(state: &TuiState) -> Vec<Line<'static>> {
    if state.results.is_empty() {
        return vec![
            Line::from(Span::styled(
                "Search, choose, watch.",
                Style::default()
                    .fg(WARM_SILVER)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Type a title, then press Enter",
                Style::default().fg(QUIET_GRAY),
            )),
        ];
    }

    state
        .results
        .iter()
        .enumerate()
        .map(|(idx, title)| {
            if idx == state.selected_result {
                Line::from(vec![
                    Span::styled(
                        "> ",
                        Style::default().fg(STEEL_BLUE).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        title.name.clone(),
                        Style::default()
                            .fg(WARM_SILVER)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])
            } else {
                Line::from(Span::styled(
                    format!("  {}", title.name),
                    Style::default().fg(QUIET_GRAY),
                ))
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
                    Span::styled(
                        "> ",
                        Style::default().fg(STEEL_BLUE).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        label,
                        Style::default()
                            .fg(WARM_SILVER)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])
            } else {
                Line::from(Span::styled(
                    format!("  {label}"),
                    Style::default().fg(QUIET_GRAY),
                ))
            }
        })
        .collect()
}

struct View {
    header: Vec<Line<'static>>,
    body_title: String,
    body: Vec<Line<'static>>,
    footer: String,
    sections: Vec<Rect>,
}

fn build_view(state: &TuiState, area: Rect) -> View {
    let sections = Layout::vertical([
        Constraint::Length(5),
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
        Some(message) => format!("{message}  / search  Enter confirm  Esc back  q quit"),
        None => "Ready  / search  Enter confirm  Esc back  q quit".to_string(),
    };

    View {
        header: vec![
            Line::from(Span::styled(
                "AYORU",
                Style::default()
                    .fg(WARM_SILVER)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                "A quieter way to watch anime.",
                Style::default().fg(MUTED_SILVER),
            )),
            Line::from(Span::styled(
                format!("{title}  Query: {}", state.query),
                Style::default().fg(QUIET_GRAY),
            )),
        ],
        body_title: title.to_string(),
        body,
        footer,
        sections,
    }
}

fn styled_block(title: &str) -> Block<'static> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(MIST_BLUE).bg(SMOKE))
        .style(Style::default().bg(SMOKE));

    if title.is_empty() {
        block
    } else {
        block.title(Span::styled(
            title.to_string(),
            Style::default().fg(QUIET_GRAY),
        ))
    }
}
