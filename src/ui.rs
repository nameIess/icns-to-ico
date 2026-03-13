use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Screen};

const ACCENT: Color = Color::Rgb(99, 179, 237);   // soft blue
const SUCCESS: Color = Color::Rgb(104, 211, 145);  // green
const ERROR: Color = Color::Rgb(252, 129, 129);    // red
const MUTED: Color = Color::Rgb(160, 160, 160);    // grey
const BG: Color = Color::Rgb(18, 18, 27);          // near-black

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Overall background block
    let bg = Block::default().style(Style::default().bg(BG));
    frame.render_widget(bg, area);

    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // header
            Constraint::Min(0),     // body
            Constraint::Length(3),  // footer / key hints
        ])
        .split(area);

    draw_header(frame, root[0]);
    draw_body(frame, app, root[1]);
    draw_footer(frame, app, root[2]);
}

fn draw_header(frame: &mut Frame, area: Rect) {
    let title = vec![
        Line::from(Span::styled(
            "▄█▀ ICNS → ICO Converter ▀█▄",
            Style::default()
                .fg(ACCENT)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "  Fast · Parallel · Multi-Resolution  ",
            Style::default().fg(MUTED),
        )),
    ];
    let block = Paragraph::new(title)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(ACCENT))
                .style(Style::default().bg(BG)),
        );
    frame.render_widget(block, area);
}

fn draw_body(frame: &mut Frame, app: &App, area: Rect) {
    match app.screen {
        Screen::WaitingForInput => draw_waiting(frame, app, area),
        Screen::Converting => draw_converting(frame, area),
        Screen::Done => draw_results(frame, app, area),
    }
}

fn draw_waiting(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let msg = vec![
        Line::from(""),
        Line::from(Span::styled("Ready to convert!", Style::default().fg(SUCCESS).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Input:  ", Style::default().fg(MUTED)),
            Span::styled(
                app.icns_dir.to_string_lossy().into_owned(),
                Style::default().fg(ACCENT),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Output: ", Style::default().fg(MUTED)),
            Span::styled(
                app.ico_dir.to_string_lossy().into_owned(),
                Style::default().fg(ACCENT),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  Place your .icns files in the input folder.",
            Style::default().fg(MUTED),
        )),
    ];

    let block = Paragraph::new(msg)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Instructions ")
                .title_style(Style::default().fg(ACCENT))
                .border_style(Style::default().fg(Color::DarkGray))
                .style(Style::default().bg(BG)),
        );
    frame.render_widget(block, chunks[0]);

    // tips panel
    let tips = Paragraph::new(vec![
        Line::from(Span::styled("  Tips", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(Span::styled("  • Download .icns from macosicons.com", Style::default().fg(MUTED))),
        Line::from(Span::styled("  • All sizes are converted in parallel", Style::default().fg(MUTED))),
        Line::from(Span::styled("  • Output ICOs contain 16/32/48/64/128/256px", Style::default().fg(MUTED))),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Info ")
            .title_style(Style::default().fg(ACCENT))
            .border_style(Style::default().fg(Color::DarkGray))
            .style(Style::default().bg(BG)),
    );
    frame.render_widget(tips, chunks[1]);
}

fn draw_converting(frame: &mut Frame, area: Rect) {
    let msg = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  ⏳ Converting files, please wait...",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Status ")
            .title_style(Style::default().fg(ACCENT))
            .border_style(Style::default().fg(Color::DarkGray))
            .style(Style::default().bg(BG)),
    );
    frame.render_widget(msg, area);
}

fn draw_results(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(0)])
        .split(area);

    // Summary stats
    let summary = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ✅ Converted:  ", Style::default().fg(MUTED)),
            Span::styled(
                app.converted.to_string(),
                Style::default().fg(SUCCESS).add_modifier(Modifier::BOLD),
            ),
            Span::styled("    ❌ Failed:  ", Style::default().fg(MUTED)),
            Span::styled(
                app.failed.to_string(),
                Style::default().fg(if app.failed > 0 { ERROR } else { MUTED }).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];
    let summary_block = Paragraph::new(summary).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Results ")
            .title_style(Style::default().fg(SUCCESS))
            .border_style(Style::default().fg(Color::DarkGray))
            .style(Style::default().bg(BG)),
    );
    frame.render_widget(summary_block, chunks[0]);

    // Log list — show last N entries that fit the area
    let items: Vec<ListItem> = app
        .logs
        .iter()
        .map(|entry| {
            let color = if entry.success { SUCCESS } else { ERROR };
            let prefix = if entry.success { "  [OK]  " } else { "  [ERR] " };
            ListItem::new(Line::from(vec![
                Span::styled(prefix, Style::default().fg(color).add_modifier(Modifier::BOLD)),
                Span::styled(entry.message.clone(), Style::default().fg(Color::White)),
            ]))
        })
        .collect();

    let log = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Conversion Log ")
            .title_style(Style::default().fg(ACCENT))
            .border_style(Style::default().fg(Color::DarkGray))
            .style(Style::default().bg(BG)),
    );
    frame.render_widget(log, chunks[1]);
}

fn draw_footer(frame: &mut Frame, app: &App, area: Rect) {
    let hints: Vec<Span> = match app.screen {
        Screen::WaitingForInput => vec![
            Span::styled(" [Enter] ", Style::default().fg(BG).bg(ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled(" Start conversion   ", Style::default().fg(MUTED)),
            Span::styled(" [q] ", Style::default().fg(BG).bg(Color::DarkGray).add_modifier(Modifier::BOLD)),
            Span::styled(" Quit ", Style::default().fg(MUTED)),
        ],
        Screen::Converting => vec![
            Span::styled("  Converting… please wait ", Style::default().fg(MUTED)),
        ],
        Screen::Done => vec![
            Span::styled(" [q] ", Style::default().fg(BG).bg(ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled(" Quit ", Style::default().fg(MUTED)),
        ],
    };

    let footer = Paragraph::new(Line::from(hints))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
                .style(Style::default().bg(BG)),
        );
    frame.render_widget(footer, area);
}
