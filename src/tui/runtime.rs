use crate::app::SystemPlayerRuntime;
use crate::errors::AppError;
use crate::provider::allanime::AllAnimeProvider;
use crate::tui::action::Action;
use crate::tui::controller::TuiController;
use crate::tui::state::{Mode, Panel, TuiState};
use crate::tui::storage::LibraryStorage;
use crate::tui::ui;
use crossterm::event::{self, Event, KeyCode};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io::{IsTerminal, stdout};
use std::path::PathBuf;

pub trait TerminalSession {
    fn enable_raw_mode(&mut self) -> std::io::Result<()>;
    fn disable_raw_mode(&mut self) -> std::io::Result<()>;
    fn enter_alt_screen(&mut self) -> std::io::Result<()>;
    fn leave_alt_screen(&mut self) -> std::io::Result<()>;
}

#[async_trait::async_trait]
pub trait RuntimeApp {
    async fn step(&mut self) -> Result<RunDecision, AppError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunDecision {
    Continue,
    Quit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputCommand {
    Action(Action),
    Submit,
    Back,
    Quit,
    FocusSearch,
}

pub async fn run_with_terminal<T, A>(terminal: &mut T, mut app: A) -> Result<(), AppError>
where
    T: TerminalSession,
    A: RuntimeApp,
{
    terminal
        .enable_raw_mode()
        .map_err(|e| AppError::Provider(e.to_string()))?;
    terminal
        .enter_alt_screen()
        .map_err(|e| AppError::Provider(e.to_string()))?;

    let result = app.step().await;

    terminal
        .leave_alt_screen()
        .map_err(|e| AppError::Provider(e.to_string()))?;
    terminal
        .disable_raw_mode()
        .map_err(|e| AppError::Provider(e.to_string()))?;

    match result? {
        RunDecision::Continue | RunDecision::Quit => Ok(()),
    }
}

pub async fn run() -> Result<(), AppError> {
    if !std::io::stdin().is_terminal() || !stdout().is_terminal() {
        return Err(AppError::Provider(
            "TUI requires a TTY terminal".to_string(),
        ));
    }

    let mut session = CrosstermTerminalSession;
    session
        .enable_raw_mode()
        .map_err(|e| AppError::Provider(e.to_string()))?;
    session
        .enter_alt_screen()
        .map_err(|e| AppError::Provider(e.to_string()))?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).map_err(|e| AppError::Provider(e.to_string()))?;
    let storage = LibraryStorage::new(default_library_path());
    let mut controller =
        TuiController::with_storage(AllAnimeProvider::new(), SystemPlayerRuntime, storage).await?;

    let result = run_loop(&mut terminal, &mut controller).await;

    terminal
        .show_cursor()
        .map_err(|e| AppError::Provider(e.to_string()))?;
    session
        .leave_alt_screen()
        .map_err(|e| AppError::Provider(e.to_string()))?;
    session
        .disable_raw_mode()
        .map_err(|e| AppError::Provider(e.to_string()))?;

    result
}

struct CrosstermTerminalSession;

impl TerminalSession for CrosstermTerminalSession {
    fn enable_raw_mode(&mut self) -> std::io::Result<()> {
        crossterm::terminal::enable_raw_mode()
    }

    fn disable_raw_mode(&mut self) -> std::io::Result<()> {
        crossterm::terminal::disable_raw_mode()
    }

    fn enter_alt_screen(&mut self) -> std::io::Result<()> {
        crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)
    }

    fn leave_alt_screen(&mut self) -> std::io::Result<()> {
        crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)
    }
}

pub fn map_key_code_for_state(state: &TuiState, key_code: KeyCode) -> Option<InputCommand> {
    match key_code {
        KeyCode::Char('/') => Some(InputCommand::FocusSearch),
        KeyCode::Char('q') => Some(InputCommand::Quit),
        KeyCode::Tab => Some(InputCommand::Action(Action::FocusNextPanel)),
        KeyCode::Char('h') if !state.search_focused => {
            Some(InputCommand::Action(Action::FocusPrevPanel))
        }
        KeyCode::Char('l') if !state.search_focused => {
            Some(InputCommand::Action(Action::FocusNextPanel))
        }
        KeyCode::Char('f') if !state.search_focused => {
            Some(InputCommand::Action(Action::ToggleFavorite))
        }
        KeyCode::Char('j') if state.mode == Mode::Search && state.search_focused => {
            Some(InputCommand::Action(Action::InsertChar('j')))
        }
        KeyCode::Char('k') if state.mode == Mode::Search && state.search_focused => {
            Some(InputCommand::Action(Action::InsertChar('k')))
        }
        KeyCode::Char('j') | KeyCode::Down => Some(InputCommand::Action(Action::MoveDown)),
        KeyCode::Char('k') | KeyCode::Up => Some(InputCommand::Action(Action::MoveUp)),
        KeyCode::Enter => Some(InputCommand::Submit),
        KeyCode::Esc => Some(InputCommand::Back),
        KeyCode::Char(ch) => Some(InputCommand::Action(Action::InsertChar(ch))),
        _ => None,
    }
}

async fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    controller: &mut TuiController<AllAnimeProvider, SystemPlayerRuntime>,
) -> Result<(), AppError> {
    loop {
        terminal
            .draw(|frame| ui::draw(frame, controller.state()))
            .map_err(|e| AppError::Provider(e.to_string()))?;

        let event = event::read().map_err(|e| AppError::Provider(e.to_string()))?;
        let Event::Key(key_event) = event else {
            continue;
        };

        let Some(command) = map_key_code_for_state(controller.state(), key_event.code) else {
            continue;
        };

        match command {
            InputCommand::Action(action) => controller.dispatch(action).await?,
            InputCommand::FocusSearch => controller.dispatch(Action::FocusSearch).await?,
            InputCommand::Back => {
                if controller.state().mode == Mode::Episodes {
                    controller.dispatch(Action::Back).await?;
                } else if controller.state().mode == Mode::Search {
                    return Ok(());
                }
            }
            InputCommand::Quit => return Ok(()),
            InputCommand::Submit => {
                if let Some(action) = submit_action(controller.state()) {
                    controller.dispatch(action).await?;
                }
            }
        }
    }
}

fn submit_action(state: &TuiState) -> Option<Action> {
    if state.focused_panel == Panel::ContextRail {
        return None;
    }

    match state.mode {
        Mode::Search if state.search_focused || state.results.is_empty() || state.is_loading => {
            Some(Action::SubmitSearch)
        }
        Mode::Search => Some(Action::OpenSelectedTitle),
        Mode::Episodes => Some(Action::PlaySelectedEpisode),
        Mode::Launching => None,
    }
}

fn default_library_path() -> PathBuf {
    if let Ok(state_home) = std::env::var("XDG_STATE_HOME") {
        return PathBuf::from(state_home).join("ayoru").join("library.json");
    }

    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home)
            .join(".local")
            .join("state")
            .join("ayoru")
            .join("library.json");
    }

    PathBuf::from(".ayoru").join("library.json")
}
