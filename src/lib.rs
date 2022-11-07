use std::io::{prelude::*, BufReader, BufWriter};

pub struct Formatter<R: Read, W: Write> {
    reader: BufReader<R>,
    writer: BufWriter<W>,
    current: u8,
    previous: u8,
    newline: String,
    indent: String,
    levels: Vec<Level>,
}

#[derive(Clone)]
struct Level {
    break_newline: bool,
}

impl<R: Read, W: Write> Formatter<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader: BufReader::new(reader),
            writer: BufWriter::new(writer),
            current: 0,
            previous: 0,
            newline: "\n".to_string(),
            indent: "    ".to_string(),
            levels: Vec::with_capacity(10),
        }
    }

    pub fn format(mut self) -> std::io::Result<()> {
        while self.read_next() {
            self.update();
            self.write()?;
        }
        Ok(())
    }
}

// Private methods
impl<R: Read, W: Write> Formatter<R, W> {
    fn push_level(&mut self, level: Level) {
        self.levels.push(level);
    }

    fn pop_level(&mut self) {
        if !self.levels.is_empty() {
            self.levels.pop();
        }
    }

    fn current_level(&self) -> Level {
        self.levels.last().cloned().unwrap_or(Level {
            break_newline: false,
        })
    }

    fn indent_level(&self) -> i32 {
        self.levels.len() as _
    }

    fn read_next(&mut self) -> bool {
        self.previous = self.current;
        let buf = std::slice::from_mut(&mut self.current);
        self.reader.read_exact(buf).is_ok()
    }

    fn update(&mut self) {
        self.update_level();
    }

    fn update_level(&mut self) {
        match self.previous {
            b'}' | b')' if self.indent_level() > 0 => self.pop_level(),
            _ => {}
        };
        match self.current {
            b'{' => self.push_level(Level {
                break_newline: true,
            }),
            b'(' => self.push_level(Level {
                break_newline: false,
            }),
            _ => {}
        };
    }

    fn write(&mut self) -> std::io::Result<()> {
        match self.previous {
            b'{' | b'(' | b',' => {
                if self.current_level().break_newline {
                    self.write_newline()?;
                    self.write_indent(self.indent_level())?;
                }
            }
            _ => {}
        };

        match self.current {
            b'}' | b')' => {
                if self.current_level().break_newline {
                    self.write_newline()?;
                    self.write_indent(self.indent_level() - 1)?;
                }
            }
            _ => {}
        };

        let current = std::slice::from_ref(&self.current);
        self.writer.write_all(current).unwrap();

        Ok(())
    }

    fn write_newline(&mut self) -> std::io::Result<()> {
        self.writer.write_all(self.newline.as_bytes())
    }

    fn write_indent(&mut self, n: i32) -> std::io::Result<()> {
        let buf = self.indent.as_bytes();
        for _ in 0..n {
            self.writer.write_all(buf)?;
        }
        Ok(())
    }
}
