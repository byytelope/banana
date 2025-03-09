use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
        .flex(ratatui::layout::Flex::SpaceBetween)
        .split(frame.area());

    frame.render_widget(
        Paragraph::new(
            "Press `Esc` or `Ctrl-C` to stop running.\n\
                Type to start the timer.",
        )
        .style(Style::new().yellow())
        .block(
            Block::bordered()
                .title("Banana")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                .border_style(Style::new().light_yellow()),
        )
        .centered(),
        layout[0],
    );

    frame.render_widget(&app.input, layout[1]);
    frame.set_cursor_position((layout[1].x + app.input.cursor as u16, layout[1].y))
}
