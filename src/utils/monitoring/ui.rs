use crate::log;
use crate::Log;
use crate::LogType;
use crate::PlotData;
use crate::{DATA, DESCR, LOGS};
use sysinfo::{NetworkExt, NetworksExt, PidExt, ProcessExt, System, SystemExt};

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::ListState,
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Clear, Dataset, Gauge, LineGauge, List,
        ListItem, Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};

pub struct TabsState {
    pub titles: Vec<String>,
    pub index: usize,
}

impl<'a> TabsState {
    pub fn new(titles: Vec<String>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct UI {
    pub tabs: TabsState,
    pub should_quit: bool,
    pub show_chart: bool,
    pub logs_state: ListState,
    pub processor_data: Vec<(f64, f64)>,
    pub memory_data: Vec<(f64, f64)>,
    pub data_window: (u64, u64),
    pub progress: f64,
    pub reps: Vec<(String, u64)>,
    pub step: u64,
    pub rep: u64,
    pub tot_reps: u64,
    pub tot_steps: u64,
    pub show_description: bool,
}
impl UI {
    pub fn new(tsteps: u64, treps: u64) -> UI {
        UI {
            tabs: TabsState::new(vec![String::from("Home")]),
            should_quit: false,
            show_chart: true,
            logs_state: ListState::default(),
            processor_data: Vec::new(),
            memory_data: Vec::new(),
            data_window: (0, 100),
            progress: 0.0,
            reps: Vec::new(),
            step: 0,
            rep: 0,
            tot_reps: treps,
            tot_steps: tsteps,
            show_description: false,
        }
    }
    pub fn on_up(&mut self) {
        let logs = LOGS.lock().unwrap();
        let i = match self.logs_state.selected() {
            Some(i) => {
                if i == 0 {
                    logs.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.logs_state.select(Some(i));
    }

    pub fn on_down(&mut self) {
        let logs = LOGS.lock().unwrap();
        let i = match self.logs_state.selected() {
            Some(i) => {
                if i >= logs.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.logs_state.select(Some(i));
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            'Q' => {
                self.should_quit = true;
            }
            'c' => {
                self.show_chart = !self.show_chart;
            }
            'C' => {
                self.show_chart = !self.show_chart;
            }
            's' => {
                self.show_description = !self.show_description;
            }
            'S' => {
                self.show_description = !self.show_description;
            }
            _ => {
                log!(LogType::Critical, format!("Invalid key pressed for {}", c));
            }
        }
    }

    pub fn on_tick(&mut self, step: u64, progress: f64) {
        // Update progress
        self.progress = progress;
        self.step = step;

        // System info - Monitoring CPU and Memory used
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut cpu_used: f64 = 0.0;
        let mut mem_used: f64 = 0.0;
        let total_mem = sys.total_memory();
        for (pid, process) in sys.processes() {
            if std::process::id() == pid.as_u32() {
                cpu_used = (process.cpu_usage() / num_cpus::get() as f32) as f64;
                mem_used = (process.memory() / total_mem) as f64 * 100.;
                break;
            }
        }

        if self.processor_data.len() > 100 {
            self.processor_data.remove(0);
            self.data_window.0 = self.data_window.0 + 1;
        }
        let position = self
            .processor_data
            .last()
            .unwrap_or(&(0. as f64, 0. as f64))
            .0;
        self.processor_data.push((position + 1., cpu_used as f64));

        if self.memory_data.len() > 100 {
            self.memory_data.remove(0);
        }

        self.memory_data.push((position + 1., mem_used as f64));
    }

    pub fn on_rep(&mut self, rep: u64, step_second_for_rep: u64) {
        self.reps
            .insert(0, ((rep + 1).to_string(), step_second_for_rep));
        self.rep = rep;
    }

    pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_y) / 2),
                    Constraint::Percentage(percent_y),
                    Constraint::Percentage((100 - percent_y) / 2),
                ]
                .as_ref(),
            )
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_x) / 2),
                    Constraint::Percentage(percent_x),
                    Constraint::Percentage((100 - percent_x) / 2),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1]
    }

    pub fn show_popup<B: Backend>(&mut self, f: &mut Frame<B>, s: String) {
        let size = f.size();
        let area = UI::centered_rect(60, 20, size);

        let text = vec![
            // Spans::from("Commands:"),
            Spans::from(vec![Span::styled(s, Style::default().fg(Color::Black))]),
            // Spans::from(vec![  Span::styled("(C)lose CPU and Memory performance monitor.", Style::default().fg(Color::Black))]),
            // Spans::from(vec![Span::styled("(← →) Arrows left/right moves between charts tabs.", Style::default().fg(Color::Black))])
        ];
        let block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                "Simulation Info",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ))
            .style(Style::default().bg(Color::Blue));
        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(paragraph, area);
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(f.size());

        for (pname, _pdata) in DATA.lock().unwrap().iter() {
            if !self.tabs.titles.contains(pname) {
                self.tabs.titles.push(String::from(pname));
            }
        }
        let titles = self
            .tabs
            .titles
            .iter()
            .map(|t| Spans::from(Span::styled(t, Style::default().fg(Color::Green))))
            .collect();

        let title = format!("Rust-ab 🦀");
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(self.tabs.index);
        f.render_widget(tabs, chunks[0]);

        match self.tabs.index {
            0 => {
                self.draw_first_tab(f, chunks[1]);
            }
            id => {
                self.draw_tab(id, f, chunks[1]);
            }
        };

        if self.show_description {
            let d = DESCR.lock().unwrap().clone();
            if d.len() != 0 {
                self.show_popup(f, d);
            }
        }
    }
    fn draw_tab<B>(&self, id: usize, f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let data = DATA.lock().unwrap();

        let mut datasets = Vec::new();
        let chart_name = self.tabs.titles[id].clone();
        let pdata = data.get(&chart_name).unwrap();

        let markers = [
            symbols::Marker::Dot,
            symbols::Marker::Braille,
            symbols::Marker::Block,
        ];
        let colors = [
            Color::Red,
            Color::Yellow,
            Color::Green,
            Color::Magenta,
            Color::Blue,
            Color::Yellow,
            Color::Green,
            Color::Cyan,
        ];

        let mut marker_id = 0;
        let mut color_id = 0;
        for (sname, points) in pdata.series.iter() {
            datasets.push(
                Dataset::default()
                    .name(sname)
                    .marker(markers[marker_id])
                    .style(Style::default().fg(colors[color_id]).clone())
                    .data(points),
            );
            marker_id = (marker_id + 1) % markers.len();
            color_id = (color_id + 1) % colors.len();
        }

        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .title(Span::styled(
                        chart_name,
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ))
                    .borders(Borders::ALL),
            )
            .x_axis(
                Axis::default()
                    .title(pdata.xlabel.clone())
                    .style(Style::default().fg(Color::Gray))
                    //TODO +10 is a temporary fix for plot range
                    .bounds([pdata.min_x, pdata.max_x + 10.0])
                    .labels(vec![
                        Span::styled(
                            pdata.min_x.to_string(),
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            pdata.max_x.to_string(),
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                    ]),
            )
            .y_axis(
                Axis::default()
                    .title(pdata.ylabel.clone())
                    .style(Style::default().fg(Color::Gray))
                    //TODO +10 is a temporary fix for plot range
                    .bounds([pdata.min_y, pdata.max_y + 10.0])
                    .labels(vec![
                        Span::styled(
                            pdata.min_y.to_string(),
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            pdata.max_y.to_string(),
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                    ]),
            );
        // TODO check if needed to reclear the area before drawing a repetition
        // if self.step == 1{
        //     f.render_widget(Clear, area);
        // }
        f.render_widget(chart, area);
    }

    fn draw_first_tab<B>(&mut self, f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Length(9),
                    Constraint::Min(8),
                    Constraint::Length(7),
                ]
                .as_ref(),
            )
            .split(area);

        self.draw_gauges(f, chunks[0]);
        self.draw_text(f, chunks[2]);
        self.draw_charts(f, chunks[1]);
    }

    fn draw_gauges<B>(&self, f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let chunks = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Length(3)].as_ref())
            .margin(1)
            .split(area);
        let block = Block::default().borders(Borders::ALL).title("Simulation");
        f.render_widget(block, area);
        let title = format!("Repetitions {}/{}:", self.rep + 1, self.tot_reps);
        let line_gauge = LineGauge::default()
            .block(Block::default().title(title))
            .gauge_style(Style::default().fg(Color::Blue))
            .line_set(symbols::line::THICK)
            .ratio((self.rep + 1) as f64 / (self.tot_reps) as f64);
        f.render_widget(line_gauge, chunks[0]);

        let label = format!("{:.2}%", self.progress * 100.);
        let gauge = Gauge::default()
            .block(Block::default().title("Repetition Progress:"))
            .gauge_style(
                Style::default()
                    .fg(Color::Magenta)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC | Modifier::BOLD),
            )
            .label(label)
            .ratio(self.progress);
        f.render_widget(gauge, chunks[1]);
    }

    fn draw_charts<B>(&mut self, f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let constraints = if self.show_chart {
            vec![Constraint::Percentage(50), Constraint::Percentage(50)]
        } else {
            vec![Constraint::Percentage(100)]
        };
        let chunks = Layout::default()
            .constraints(constraints)
            .direction(Direction::Horizontal)
            .split(area);
        {
            let chunks_pane_one = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Horizontal)
                .split(chunks[0]);

            // Draw tasks
            let logs = LOGS.lock().unwrap();

            let info_style = Style::default().fg(Color::Blue);
            let warning_style = Style::default().fg(Color::Yellow);
            let error_style = Style::default().fg(Color::Magenta);
            let critical_style = Style::default().fg(Color::Red);

            let logs: Vec<ListItem> = logs
                .iter()
                .map(|x| {
                    let s = match x.ltype {
                        LogType::Warning => warning_style,
                        LogType::Error => error_style,
                        LogType::Critical => critical_style,
                        _ => info_style,
                    };

                    let content = vec![Spans::from(vec![
                        Span::styled(format!("{:<9}", x.ltype), s),
                        Span::raw(x.body.clone()),
                    ])];

                    ListItem::new(content)
                })
                .collect();
            let logs = List::new(logs)
                .block(Block::default().borders(Borders::ALL).title("Logs"))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");
            f.render_stateful_widget(logs, chunks_pane_one[0], &mut self.logs_state);

            let new: Vec<(&str, u64)> = self
                .reps
                .iter()
                .map(|(string, val)| (string.as_str(), *val))
                .collect();

            let barchart = BarChart::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Step/Seconds for Repetitions"),
                )
                .data(&new[..])
                .bar_width(3)
                .bar_gap(2)
                .bar_set(symbols::bar::NINE_LEVELS)
                .value_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Green)
                        .add_modifier(Modifier::ITALIC),
                )
                .label_style(Style::default().fg(Color::Yellow))
                .bar_style(Style::default().fg(Color::Green));
            f.render_widget(barchart, chunks_pane_one[1]);
        }
        if self.show_chart {
            let x_labels = vec![
                Span::styled(
                    format!("{}", self.data_window.0),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{}", (self.data_window.0 + 100)),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ];
            let datasets = vec![
                Dataset::default()
                    .name("% processor")
                    .marker(symbols::Marker::Dot)
                    .style(Style::default().fg(Color::Cyan))
                    .data(&self.processor_data),
                Dataset::default()
                    .name("% memory")
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Yellow))
                    .data(&self.memory_data),
            ];
            let chart = Chart::new(datasets)
                .block(
                    Block::default()
                        .title(Span::styled(
                            "CPU and Memory",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("Step")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([self.data_window.0 as f64, (self.data_window.0 + 100) as f64])
                        .labels(x_labels),
                )
                .y_axis(
                    Axis::default()
                        .title("%")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, 100.0])
                        .labels(vec![
                            Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                            Span::styled("100", Style::default().add_modifier(Modifier::BOLD)),
                        ]),
                );
            f.render_widget(chart, chunks[1]);
        }
    }

    fn draw_text<B>(&self, f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let text = vec![
            Spans::from("Commands:"),
            Spans::from(vec![Span::styled(
                "(Q)uit",
                Style::default().fg(Color::Red),
            )]),
            Spans::from(vec![Span::styled(
                "(C)lose CPU and Memory performance monitor.",
                Style::default().fg(Color::Green),
            )]),
            Spans::from(vec![Span::styled(
                "(← →) Arrows left/right moves between charts tabs.",
                Style::default().fg(Color::Blue),
            )]),
            Spans::from(vec![Span::styled(
                "(S)how model info.",
                Style::default().fg(Color::White),
            )]),
        ];
        let block = Block::default().borders(Borders::ALL).title(Span::styled(
            "Help",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
    }
}
