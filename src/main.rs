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
        let terminal = Terminal::new(CrosstermBackend::new(stdout))?;

        run(terminal)?
    };

    // Print input on SIGINT
    println!("{}", app.input.iter().collect::<String>());
    println!("{:#?}", app.buffer);
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

struct App<'a> {
    input: Vec<char>,
    expected: &'a str,
    cursor: usize,
    should_quit: bool,
    /// SCRATCH BUFFER
    buffer: Vec<String>,
}

impl<'a> App<'a> {
    fn new(expected: &'a str) -> Self {
        Self {
            input: expected.chars().collect(),
            expected,
            cursor: 0,
            should_quit: false,
            buffer: vec![],
        }
    }
}

enum JumpType {
    Start,
    End,
    Prev,
    Next,
}

enum DelType {
    Word,
    Line,
}

enum MoveType {
    Prev,
    Next,
}

enum Message {
    InputChar(char),
    Jump(JumpType),
    Move(MoveType),
    Delete(DelType),
    Backspace,
    Space,
    Quit,
    Buffer(String),
}

fn run(mut terminal: DefaultTerminal) -> io::Result<App<'static>> {
    let mut app = App::new("the quick brown fox jumps over the lazy dog");

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

    let message = match (key.code, key.modifiers) {
        (KeyCode::Char(' '), KeyModifiers::NONE) => Message::Space,
        (KeyCode::Char(c), KeyModifiers::NONE) => Message::InputChar(c),
        (KeyCode::Backspace, KeyModifiers::NONE) => Message::Backspace,

        (KeyCode::Char('b'), KeyModifiers::ALT) => Message::Jump(JumpType::Prev),
        (KeyCode::Char('f'), KeyModifiers::ALT) => Message::Jump(JumpType::Next),
        (KeyCode::Char('a'), KeyModifiers::CONTROL) => Message::Jump(JumpType::Start),
        (KeyCode::Char('e'), KeyModifiers::CONTROL) => Message::Jump(JumpType::End),

        (KeyCode::Backspace, KeyModifiers::ALT) => Message::Delete(DelType::Word),
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => Message::Delete(DelType::Line),

        (KeyCode::Left, KeyModifiers::NONE) => Message::Move(MoveType::Prev),
        (KeyCode::Right, KeyModifiers::NONE) => Message::Move(MoveType::Next),

        _ => Message::Buffer(format!("{} + {}", key.code, key.modifiers)),
    };

    Ok(Some(message))
}

fn update(app: &mut App, message: Message) {
    match message {
        Message::Space => {
            app.input.insert(app.cursor, ' ');
            app.cursor += 1;
        }
        Message::InputChar(ch) => {
            app.input.insert(app.cursor, ch);
            app.cursor += 1;
        }
        Message::Jump(jmp) => match jmp {
            JumpType::Start => {
                app.cursor = 0;
            }
            JumpType::End => {
                app.cursor = app.input.len() - 1;
            }
            JumpType::Prev => {
                if let Some(space_idx) = app.input[..app.cursor]
                    .iter()
                    .rposition(|x| x.is_ascii_whitespace())
                {
                    app.cursor = space_idx
                } else {
                    app.cursor = 0
                }
            }
            JumpType::Next => {
                let current_is_white = app
                    .input
                    .get(app.cursor)
                    .unwrap_or(&' ')
                    .is_ascii_whitespace();
                let pos = if current_is_white && app.cursor < app.input.len() {
                    app.cursor + 1
                } else {
                    app.cursor
                };

                if let Some(space_idx) = app.input[pos..]
                    .iter()
                    .position(|x| x.is_ascii_whitespace())
                {
                    app.cursor = space_idx + pos
                } else {
                    app.cursor = app.input.len()
                }
            }
        },
        Message::Move(mv) => match mv {
            MoveType::Prev => {
                if let Some(val) = app.cursor.checked_sub(1) {
                    app.cursor = val
                }
            }
            MoveType::Next => {
                if let Some(val) = app.cursor.checked_add(1)
                    && app.cursor < app.input.len()
                {
                    app.cursor = val
                }
            }
        },
        Message::Delete(del) => match del {
            DelType::Word => {
                let mut start = app.cursor;
                while start > 0 && app.input[start - 1].is_ascii_whitespace() {
                    start -= 1;
                }
                while start > 0 && !app.input[start - 1].is_ascii_whitespace() {
                    start -= 1;
                }

                app.input.drain(start..app.cursor);
                app.cursor = start;
            }
            DelType::Line => {
                app.input.clear();
                app.cursor = 0;
            }
        },
        Message::Backspace => {
            if app.cursor <= app.input.len() && app.cursor != 0 {
                app.input.remove(app.cursor - 1);
            }

            if app.cursor != 0 {
                app.cursor -= 1
            };
        }
        Message::Quit => app.should_quit = true,
        Message::Buffer(msg) => app.buffer.push(msg),
    }
}

fn view(frame: &mut Frame, app: &App) {
    let p = Paragraph::new(app.input.iter().collect::<String>());

    frame.render_widget(p, frame.area());
    frame.set_cursor_position(Position::new(frame.area().x + app.cursor as u16, 0));
}
