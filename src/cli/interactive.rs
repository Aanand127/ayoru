use crate::app::PickerRuntime;
use crate::core::models::{Episode, Title};
use crate::errors::AppError;
use crossterm::cursor;
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{self, ClearType};
use std::io::{IsTerminal, Write, stdin, stdout};

use super::picker::{Key, Picker, PickerEvent};

pub struct InteractivePickerRuntime;

impl PickerRuntime for InteractivePickerRuntime {
    fn pick_title(&self, titles: &[Title]) -> Result<usize, AppError> {
        let items = titles.iter().map(|t| t.name.clone()).collect::<Vec<_>>();
        pick_from_list("Select title", &items)
    }

    fn pick_episode(&self, episodes: &[Episode]) -> Result<usize, AppError> {
        let items = episodes
            .iter()
            .map(|e| format!("Episode {}", e.number))
            .collect::<Vec<_>>();
        pick_from_list("Select episode", &items)
    }
}

fn pick_from_list(prompt: &str, items: &[String]) -> Result<usize, AppError> {
    if items.is_empty() {
        return Err(AppError::Provider("No items to select".to_string()));
    }

    if !stdin().is_terminal() || !stdout().is_terminal() {
        return Err(AppError::Provider(
            "Interactive picker requires a TTY terminal".to_string(),
        ));
    }

    terminal::enable_raw_mode().map_err(|e| AppError::Provider(e.to_string()))?;
    let mut picker = Picker::new(items.len());

    loop {
        render(prompt, items, picker.selected())?;

        let event = event::read().map_err(|e| AppError::Provider(e.to_string()))?;
        let Event::Key(key_event) = event else {
            continue;
        };

        let key = match key_event.code {
            KeyCode::Up => Some(Key::Up),
            KeyCode::Down => Some(Key::Down),
            KeyCode::Char('j') => Some(Key::J),
            KeyCode::Char('k') => Some(Key::K),
            KeyCode::Enter => Some(Key::Enter),
            KeyCode::Esc => Some(Key::Esc),
            KeyCode::Char('q') => Some(Key::Q),
            _ => None,
        };

        let Some(key) = key else {
            continue;
        };

        match picker.handle(key) {
            PickerEvent::Continue => {}
            PickerEvent::Selected(idx) => {
                cleanup_screen()?;
                terminal::disable_raw_mode().map_err(|e| AppError::Provider(e.to_string()))?;
                return Ok(idx);
            }
            PickerEvent::Cancelled => {
                cleanup_screen()?;
                terminal::disable_raw_mode().map_err(|e| AppError::Provider(e.to_string()))?;
                return Err(AppError::Cancelled);
            }
        }
    }
}

fn render(prompt: &str, items: &[String], selected: usize) -> Result<(), AppError> {
    let mut out = stdout();
    execute!(out, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))
        .map_err(|e| AppError::Provider(e.to_string()))?;
    writeln!(out, "{prompt} (↑/↓, j/k, Enter, q/Esc)")
        .map_err(|e| AppError::Provider(e.to_string()))?;
    writeln!(out).map_err(|e| AppError::Provider(e.to_string()))?;

    for (idx, item) in items.iter().enumerate() {
        if idx == selected {
            writeln!(out, "> {item}").map_err(|e| AppError::Provider(e.to_string()))?;
        } else {
            writeln!(out, "  {item}").map_err(|e| AppError::Provider(e.to_string()))?;
        }
    }

    out.flush().map_err(|e| AppError::Provider(e.to_string()))
}

fn cleanup_screen() -> Result<(), AppError> {
    let mut out = stdout();
    execute!(out, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))
        .map_err(|e| AppError::Provider(e.to_string()))
}
