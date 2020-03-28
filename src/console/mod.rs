use std::sync::mpsc;
use std::thread;

pub struct Console {
    pub previous_lines: Vec<String>,
    current_line: String,
    cursor_position: u32,
    text_width: u32,
    receiver: Option<mpsc::Receiver<Packet>>
}

impl Console {
    pub fn new(width: u32) -> Console {
        Console {
            previous_lines: Vec::new(),
            current_line: String::from(""),
            cursor_position: 0,
            text_width: width,
            receiver: None
        }
    }

    pub fn give_receiver(&mut self, receiver: mpsc::Receiver<Packet>) {
        self.receiver = Some(receiver);
    }

    pub fn receive(&mut self) {
        if let Some(rec) = &self.receiver {
            match rec.try_recv() {
                Ok(m) => {
                    match m {
                        Packet::Chars(t) => {
                            // println!("{:?}", &t);
                            for c in &t {
                                if self.current_line.len() < self.text_width as usize && *c != '\n'{
                                    self.cursor_position += 1;
                                    self.current_line.push(*c);
                                } else {
                                    self.newline();
                                }
                            }
                            // println!("{}", &self.current_line);
                        },
                        Packet::End => {}
                    }
                },
                Err(_) => ()
            }
        }
    }

    fn newline(&mut self) {
        self.previous_lines.push(self.current_line.clone());
        self.cursor_position = 0;
        self.current_line = String::from("");
    }

    pub fn view(&self, max_lines: u32) -> String {
        let mut screen = String::from("");
        let size: usize = std::cmp::min(self.previous_lines.len(), max_lines as usize);
        self.previous_lines[0..size].iter().rfold(&mut screen, |accum, line| {
            accum.push('\n');
            accum.push_str(&line);
            accum
        });
        screen.push('\n');
        screen.push_str(&self.current_line);
        screen
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Packet {
    Chars([char; 8]),
    End
}