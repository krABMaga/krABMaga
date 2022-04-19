use crate::utils::monitoring::app::App;
use crate::{DATA, LOGS};
use crate::Log;
use crate::LogType;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt, PidExt};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
    widgets::ListState,
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
    pub processor_data: Vec<(f64,f64)>,
    pub memory_data: Vec<(f64,f64)>,
    pub data_window: (u64, u64),
    pub progress: f64
}
impl UI {
    pub fn new() -> UI {
       UI {
        tabs: TabsState::new(vec![String::from("Home")]),
        should_quit: false,
        show_chart: true,
        logs_state: ListState::default(),
        processor_data: Vec::new(),
        memory_data: Vec::new(),
        data_window: (0, 100),
        progress: 0.0
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
            _ => {}
        }
    }

    pub fn on_tick(&mut self, step: u64, progress: f64) {
        
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut cpu_used: f64 = 0.0;
        let mut mem_used: f64 = 0.0;
        for (pid, process) in sys.processes() {
            if std::process::id() == pid.as_u32(){
               cpu_used = (process.cpu_usage() / num_cpus::get() as f32) as f64;
               mem_used = (process.memory() /1_000) as f64;

               // calcolare percentuale memoria
               break;
            }
        }

        if self.processor_data.len() > 100
        {
            self.processor_data.remove(0);
            self.data_window.0 = self.data_window.0 + 1;
        }

        self.processor_data.push( (step as f64, cpu_used as f64));
        
        if self.memory_data.len() > 100
        {   
            self.memory_data.remove(0);
        }        
    
        self.memory_data.push( (step as f64, mem_used as f64));


        // Update progress
         self.progress = progress;
        // if self.progress > 1.0 {
        //     self.progress = 0.0;
        // }

        // self.sparkline.on_tick();
        // self.signals.on_tick();

        // let log = self.logs.items.pop().unwrap();
        // self.logs.items.insert(0, log);

        // let event = self.barchart.pop().unwrap();
        // self.barchart.insert(0, event);
    }
    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(f.size());

        for (pname, pdata) in DATA.lock().unwrap().iter() {
            if !self.tabs.titles.contains( pname ) {
                self.tabs.titles.push( String::from(pname));
            }
            
        }
        let titles = self
            .tabs
            .titles
            .iter()
            .map(|t| Spans::from(Span::styled(t, Style::default().fg(Color::Green))))
            .collect();
     
        {
            let data = DATA.lock().unwrap();
            let title = format!("Rust-ab 🦀");
            let tabs = Tabs::new(titles)
                .block(Block::default().borders(Borders::ALL).title(title))
                .highlight_style(Style::default().fg(Color::Yellow))
                .select(self.tabs.index);
            f.render_widget(tabs, chunks[0]);
        }   //end del lock scope
        // id di ogni tab
        match self.tabs.index {
            0 => self.draw_first_tab(f, chunks[1]),
            id => self.draw_tab(id, f, chunks[1])
        };
       
    }
    fn draw_tab<B>(&self, id:usize,  f: &mut Frame<B>,  area: Rect) where
    B: Backend,
    {
        let data = DATA.lock().unwrap();
        
        let mut datasets = Vec::new();
        let chart_name = self.tabs.titles[id].clone();
        let pdata = data.get(&chart_name).unwrap();

        let markers = [symbols::Marker::Dot, symbols::Marker::Block, symbols::Marker::Braille];
        let colors = [Color::Red, Color::Yellow, Color::Green, Color::Magenta, Color::Blue, Color::Yellow, Color::Green, Color::Cyan];
        let mut marker_id = 0; 
        let mut color_id = 0;
        for (sname, points) in pdata.series.iter(){
            datasets.push(
                Dataset::default()
                    .name(sname)
                    .marker(markers[marker_id])
                    .style(Style::default().fg(colors[color_id]).clone())
                    .data(points)
            );
            marker_id = (marker_id + 1 ) % markers.len();
            color_id = (color_id + 1 ) % colors.len();
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
                        .title("X Axis")
                        .style(Style::default().fg(Color::Gray))
                        //TODO inchiovato con le puntine
                        .bounds([pdata.min_x, pdata.max_x + 10.0])
                        .labels(vec![
                            Span::styled(pdata.min_x.to_string(), Style::default().add_modifier(Modifier::BOLD)),
                            Span::styled(pdata.max_x.to_string(), Style::default().add_modifier(Modifier::BOLD)),
                        ]),
                )
                .y_axis(
                    Axis::default()
                        .title("Y Axis")
                        .style(Style::default().fg(Color::Gray))
                        //TODO inchiovato con le puntine
                        .bounds([pdata.min_y, pdata.max_y + 10.0])
                        .labels(vec![
                            Span::styled(pdata.min_y.to_string(), Style::default().add_modifier(Modifier::BOLD)),
                            Span::styled(pdata.max_y.to_string(), Style::default().add_modifier(Modifier::BOLD)),
                        ]),
                );
        f.render_widget(chart, area);
    }

    fn draw_first_tab<B>(&mut self, f: &mut Frame<B>,  area: Rect)
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
        self.draw_charts(f, chunks[1]);
        self.draw_text(f, chunks[2]);
    }

    fn draw_gauges<B>(&self, f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3)
                ]
                .as_ref(),
            )
            .margin(1)
            .split(area);
        let block = Block::default().borders(Borders::ALL).title("Simulation progress");
        f.render_widget(block, area);

        let label = format!("{:.2}%",  self.progress * 100.);
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
        f.render_widget(gauge, chunks[0]);

        // let sparkline = Sparkline::default()
        //     .block(Block::default().title("Repetition (step/seconds):"))
        //     .style(Style::default().fg(Color::Green))
        //     .data(&app.sparkline.points)
        //     .bar_set(if app.enhanced_graphics {
        //         symbols::bar::NINE_LEVELS
        //     } else {
        //         symbols::bar::THREE_LEVELS
        //     });
        // f.render_widget(sparkline, chunks[1]);

        // let line_gauge = LineGauge::default()
        //     .block(Block::default().title("LineGauge:"))
        //     .gauge_style(Style::default().fg(Color::Magenta))
        //     .line_set(if app.enhanced_graphics {
        //         symbols::line::THICK
        //     } else {
        //         symbols::line::NORMAL
        //     })
        //     .ratio(app.progress);
        // f.render_widget(line_gauge, chunks[2]);
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
               f.render_stateful_widget(logs, chunks[0], &mut self.logs_state);
        }
        if self.show_chart {

            let x_labels = vec![
                Span::styled(
                    format!("{}",  self.data_window.0),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{}",  (self.data_window.0 + 100)),
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
            Spans::from("Basic commands:"),
            Spans::from(""),
            Spans::from(vec![
                Span::styled("(Q)uit", Style::default().fg(Color::Red)),
                Span::raw(" "),
                Span::styled("(C)lose performance monitor", Style::default().fg(Color::Green)),
                Span::raw(" "),
                Span::styled("Arrows left/right moves between charts tabs", Style::default().fg(Color::Blue)),
                Span::raw("."),
            ]),
            Spans::from(vec![
                Span::raw("Oh and if you didn't "),
                Span::styled("notice", Style::default().add_modifier(Modifier::ITALIC)),
                Span::raw(" you can "),
                Span::styled("automatically", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" "),
                Span::styled("wrap", Style::default().add_modifier(Modifier::REVERSED)),
                Span::raw(" your "),
                Span::styled("text", Style::default().add_modifier(Modifier::UNDERLINED)),
                Span::raw(".")
            ]),
            Spans::from(
                "One more thing is that it should display unicode characters: 10€"
            ),
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
