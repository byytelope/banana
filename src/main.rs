use std::io::{self, Write};

use crossterm::{
    event::{
        self, Event, KeyCode, KeyModifiers, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
        PushKeyboardEnhancementFlags,
    },
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    DefaultTerminal, Frame, Terminal, backend::CrosstermBackend, layout::Position,
    widgets::Paragraph,
};

fn main() -> io::Result<()> {
    let app = {
        let mut stdout = io::stdout();
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(
            stdout,
            EnterAlternateScreen,
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
        )?;
        let _terminal_restore = TerminalRestore;
        let terminal = Terminal::new(CrosstermBackend::new(stdout));

        run(&mut terminal?)?
    };

    // Print input on SIGINT
    println!("{}", app.input.iter().collect::<String>());
    Ok(())
}

struct TerminalRestore;

impl Drop for TerminalRestore {
    fn drop(&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();

        let mut stdout = io::stdout();
        let _ = crossterm::execute!(stdout, LeaveAlternateScreen, PopKeyboardEnhancementFlags);
        if std::thread::panicking() {
            let _ = stdout.write(b"Panic! At The Disco\n");
        }
    }
}

#[derive(Default)]
struct App {
    input: Vec<char>,
    expected: Vec<char>,
    cursor: usize,
    should_quit: bool,
}

enum JumpType {
    Start,
    End,
    Prev,
    Next,
}

enum Message {
    InputChar(char),
    Jump(JumpType),
    Backspace,
    Space,
    Quit,
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<App> {
    let mut app = App::default();

    while !app.should_quit {
        terminal.draw(|f| view(f, &app))?;

        if let Some(message) = read_message()? {
            update(&mut app, message);
        }
    }

    Ok(app)
}

fn read_message() -> io::Result<Option<Message>> {
    let Event::Key(key) = event::read()? else {
        return Ok(None);
    };

    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        return Ok(Some(Message::Quit));
    }

    if !key.modifiers.is_empty() {
        return Ok(None);
    }

    let message = match key.code {
        KeyCode::Char(c) => Message::InputChar(c),
        KeyCode::Backspace => Message::Backspace,
        _ => return Ok(None),
    };

    Ok(Some(message))
}

fn update(app: &mut App, message: Message) {
    match message {
        Message::InputChar(ch) => {
            app.input.push(ch);
            app.cursor += 1;
        }
        Message::Jump(_) => todo!(),
        Message::Backspace => {
            app.input.pop();
            if app.cursor != 0 {
                app.cursor -= 1
            };
        }
        Message::Space => todo!(),
        Message::Quit => app.should_quit = true,
    }
}

fn view(frame: &mut Frame, app: &App) {
    let p = Paragraph::new(app.input.iter().collect::<String>());

    frame.render_widget(p, frame.area());
    frame.set_cursor_position(Position::new(frame.area().x + app.cursor as u16, 0));
}
