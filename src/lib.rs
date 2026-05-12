use wasm_bindgen::prelude::*;
use ratatui::{
    backend::TestBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, List, ListItem, Paragraph},
    Terminal,
};

// 1. We create a dedicated struct to hold both the name and price
#[derive(Clone)]
struct MenuItem {
    name: String,
    price: u32,
}

#[wasm_bindgen]
pub struct AppState {
    menu_items: Vec<MenuItem>,
    cart: Vec<MenuItem>, // The cart now holds full objects, not just text
    selected_index: usize,
}

#[wasm_bindgen]
impl AppState {
    pub fn new() -> Self {
        console_error_panic_hook::set_once(); 
        
        Self {
            // 2. Testing Text Alignment: Mixing Thai, English, and Emoji
            menu_items: vec![
                MenuItem { name: "🥩 Wagyu Beef (เนื้อวากิว)".to_string(), price: 500 },
                MenuItem { name: "🍣 Salmon Nigiri (ซูชิแซลมอน)".to_string(), price: 120 },
                MenuItem { name: "🐖 Kurobuta Pork (หมูคุโรบูตะ)".to_string(), price: 200 },
                MenuItem { name: "🍜 Udon Noodles (อุด้ง)".to_string(), price: 80 },
                MenuItem { name: "🍵 Matcha Daifuku (ไดฟุกุชาเขียว)".to_string(), price: 50 },
            ],
            cart: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn handle_input(&mut self, input: &str) {
        match input {
            "\x1b[A" => { if self.selected_index > 0 { self.selected_index -= 1; } }
            "\x1b[B" => { if self.selected_index < self.menu_items.len() - 1 { self.selected_index += 1; } }
            "\r" => { self.cart.push(self.menu_items[self.selected_index].clone()); }
            "\x7f" | "\x08" => { self.cart.pop(); }
            _ => {}
        }
    }

    pub fn render(&self) -> String {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|f| {
            // 3. Layout Upgrade: Slice the screen vertically first to make room for a banner
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(f.size());

            // --- The Banner (Top) ---
            let banner = Paragraph::new("=== 🍣 THE TERMINAL SUSHI CART 🍣 ===")
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center);
            f.render_widget(banner, main_chunks[0]);

            // Slice the bottom section horizontally for the menu and cart
            let content_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(main_chunks[1]);

            // --- The Menu (Bottom Left) ---
            let items: Vec<ListItem> = self.menu_items
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    let text = format!("{} - ฿{}", m.name, m.price);
                    if i == self.selected_index {
                        ListItem::new(Line::from(Span::styled(
                            format!("> {}", text),
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                        )))
                    } else {
                        ListItem::new(Line::from(Span::raw(format!("  {}", text))))
                    }
                })
                .collect();

            // Styling Upgrade: Rounded borders with green text
            let menu_block = Block::default()
                .title(" Menu (Up/Down/Enter) ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Green));
                
            let menu_list = List::new(items).block(menu_block);
            f.render_widget(menu_list, content_chunks[0]);

            // --- The Cart & Math (Bottom Right) ---
            let mut cart_lines = Vec::new();
            let mut total_price = 0;

            for item in &self.cart {
                cart_lines.push(Line::from(format!("- {} (฿{})", item.name, item.price)));
                total_price += item.price; // 4. The Math: Tallying the total
            }

            if !self.cart.is_empty() {
                cart_lines.push(Line::from("-----------------------"));
                cart_lines.push(Line::from(Span::styled(
                    format!("TOTAL: ฿{}", total_price),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )));
            }

            // Styling Upgrade: Rounded borders with blue text
            let cart_block = Block::default()
                .title(" Order Receipt (Bksp to Undo) ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Blue));
                
            let cart_display = Paragraph::new(cart_lines).block(cart_block);
            f.render_widget(cart_display, content_chunks[1]);
            
        }).unwrap();

        // 5. The ANSI Engine Update: We added support for Cyan, Green, Blue, and Red
        let buffer = terminal.backend().buffer();
        let mut output = String::new();
        
        for y in 0..24 {
            for x in 0..80 {
                let cell = buffer.get(x, y);
                output.push_str("\x1b[0m"); 
                
                if cell.modifier.contains(Modifier::BOLD) {
                    output.push_str("\x1b[1m");
                }
                
                match cell.fg {
                    Color::Yellow => output.push_str("\x1b[33m"),
                    Color::Cyan => output.push_str("\x1b[36m"),
                    Color::Green => output.push_str("\x1b[32m"),
                    Color::Blue => output.push_str("\x1b[34m"),
                    Color::Red => output.push_str("\x1b[31m"),
                    _ => output.push_str("\x1b[39m"), 
                }
                
                output.push_str(cell.symbol());
            }
            output.push_str("\x1b[0m\r\n"); 
        }
        
        output
    }
}