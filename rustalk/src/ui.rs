use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;
use tokio::sync::mpsc;
use chrono::Utc;

use crate::app::UIEvent;

#[allow(dead_code)] // Terminal UI for future interactive mode
pub struct ChatUI {
    messages: Vec<ChatMessage>,
    input: String,
    event_sender: Option<mpsc::UnboundedSender<UIEvent>>,
    should_quit: bool,
}

#[derive(Clone)]
#[allow(dead_code)] // Used by future TUI implementation
struct ChatMessage {
    sender: String,
    content: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    is_own: bool,
    is_system: bool,
}

#[allow(dead_code)] // Terminal UI implementation for future use
impl ChatUI {
    pub fn new() -> Result<Self> {
        Ok(ChatUI {
            messages: Vec::new(),
            input: String::new(),
            event_sender: None,
            should_quit: false,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Create event channel
        let (tx, mut rx) = mpsc::unbounded_channel::<UIEvent>();
        self.event_sender = Some(tx);

        // Welcome message
        self.add_system_message("🚀 Welcome to Rustalk! Type /help for commands.").await;

        // Main UI loop
        loop {
            terminal.draw(|f| self.draw_ui(f))?;

            // Handle events
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                                self.should_quit = true;
                                break;
                            }
                            KeyCode::Enter => {
                                if !self.input.is_empty() {
                                    let input = self.input.clone();
                                    self.input.clear();
                                    
                                    if input == "/quit" {
                                        self.should_quit = true;
                                        break;
                                    }
                                    
                                    if let Some(sender) = &self.event_sender {
                                        let _ = sender.send(UIEvent::Input(input));
                                    }
                                }
                            }
                            KeyCode::Backspace => {
                                self.input.pop();
                            }
                            KeyCode::Char(c) => {
                                self.input.push(c);
                            }
                            KeyCode::Esc => {
                                self.should_quit = true;
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }

            // Handle UI events from app
            while let Ok(event) = rx.try_recv() {
                match event {
                    UIEvent::Quit => {
                        self.should_quit = true;
                        break;
                    }
                    _ => {
                        // Handle other events if needed
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<()> {
        // Restore terminal
        disable_raw_mode()?;
        execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        Ok(())
    }

    pub async fn next_event(&mut self) -> Option<UIEvent> {
        // This is a simplified version - in a real implementation,
        // you'd have a more sophisticated event handling system
        
        if self.should_quit {
            Some(UIEvent::Quit)
        } else {
            None
        }
    }

    fn draw_ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(f.size());

        self.draw_messages(f, chunks[0]);
        self.draw_input(f, chunks[1]);
    }

    fn draw_messages(&self, f: &mut Frame, area: Rect) {
        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .map(|msg| {
                let time = msg.timestamp.format("%H:%M:%S");
                let style = if msg.is_system {
                    Style::default().fg(Color::Yellow)
                } else if msg.is_own {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Blue)
                };

                let content = if msg.is_system {
                    format!("[{}] {}", time, msg.content)
                } else {
                    format!("[{}] {}: {}", time, msg.sender, msg.content)
                };

                ListItem::new(Line::from(Span::styled(content, style)))
            })
            .collect();

        let messages_list = List::new(messages)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("💬 Rustalk Chat")
                    .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            );

        f.render_widget(messages_list, area);
    }

    fn draw_input(&self, f: &mut Frame, area: Rect) {
        let input = Paragraph::new(self.input.as_str())
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("📝 Type your message (Enter to send, Ctrl+C or /quit to exit)")
                    .title_style(Style::default().fg(Color::Green))
            );

        f.render_widget(input, area);
    }

    pub async fn add_message(&mut self, sender: &str, content: &str, is_own: bool) {
        let message = ChatMessage {
            sender: sender.to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
            is_own,
            is_system: false,
        };
        
        self.messages.push(message);
        
        // Keep only last 100 messages to prevent memory issues
        if self.messages.len() > 100 {
            self.messages.remove(0);
        }
    }

    pub async fn add_system_message(&mut self, content: &str) {
        let message = ChatMessage {
            sender: "System".to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
            is_own: false,
            is_system: true,
        };
        
        self.messages.push(message);
        
        // Keep only last 100 messages
        if self.messages.len() > 100 {
            self.messages.remove(0);
        }
    }
}