use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

pub fn render(area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(
            Style::default()
                .fg(Color::Blue)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .border_set(border::THICK);

    let crabtype: String = "settings".to_string();

    let menu_options: String = {
        "
    [b]egin
    [s]ettings
    [q]uit
        "
        .to_string()
    };
    let comb_str = crabtype + &menu_options;

    Paragraph::new(comb_str).block(block).render(area, buf);
}
