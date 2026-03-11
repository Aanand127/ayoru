use ayoru::tui::runtime::{RunDecision, RuntimeApp, TerminalSession, run_with_terminal};

#[derive(Default)]
struct FakeTerminal {
    entered_alt_screen: bool,
    left_alt_screen: bool,
    raw_mode_enabled: bool,
    raw_mode_disabled: bool,
}

impl TerminalSession for FakeTerminal {
    fn enable_raw_mode(&mut self) -> std::io::Result<()> {
        self.raw_mode_enabled = true;
        Ok(())
    }

    fn disable_raw_mode(&mut self) -> std::io::Result<()> {
        self.raw_mode_disabled = true;
        Ok(())
    }

    fn enter_alt_screen(&mut self) -> std::io::Result<()> {
        self.entered_alt_screen = true;
        Ok(())
    }

    fn leave_alt_screen(&mut self) -> std::io::Result<()> {
        self.left_alt_screen = true;
        Ok(())
    }
}

struct QuitImmediately;

#[async_trait::async_trait]
impl RuntimeApp for QuitImmediately {
    async fn step(&mut self) -> Result<RunDecision, ayoru::errors::AppError> {
        Ok(RunDecision::Quit)
    }
}

#[tokio::test]
async fn tui_runtime_restores_terminal_on_clean_exit() {
    let mut terminal = FakeTerminal::default();

    run_with_terminal(&mut terminal, QuitImmediately)
        .await
        .unwrap();

    assert!(terminal.entered_alt_screen);
    assert!(terminal.left_alt_screen);
    assert!(terminal.raw_mode_enabled);
    assert!(terminal.raw_mode_disabled);
}
