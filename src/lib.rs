use std::io::{prelude::*, BufReader, BufWriter};

pub struct Formatter<R: Read, W: Write> {
    reader: BufReader<R>,
    writer: BufWriter<W>,
    current: u8,
    newline: String,
    indent: String,
    indent_level: u32,
}

impl<R: Read, W: Write> Formatter<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader: BufReader::new(reader),
            writer: BufWriter::new(writer),
            current: 0,
            newline: "\n".to_string(),
            indent: "    ".to_string(),
            indent_level: 0,
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
    fn read_next(&mut self) -> bool {
        let buf = std::slice::from_mut(&mut self.current);
        self.reader.read_exact(buf).is_ok()
    }

    fn update(&mut self) {
        self.update_indent();
    }

    fn update_indent(&mut self) {
        match self.current {
            b'{' | b'(' => self.indent_level += 1,
            b'}' | b')' if self.indent_level > 0 => self.indent_level -= 1,
            _ => {}
        };
    }

    fn write(&mut self) -> std::io::Result<()> {
        self.write_newline_before()?;

        let buf = std::slice::from_ref(&self.current);
        self.writer.write_all(buf).unwrap();

        self.write_newline_after()?;

        Ok(())
    }

    fn write_newline(&mut self) -> std::io::Result<()> {
        self.writer.write_all(self.newline.as_bytes())
    }

    fn write_newline_before(&mut self) -> std::io::Result<()> {
        match self.current {
            b'}' | b')' => {
                self.write_newline()?;
                self.write_indent()?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn write_newline_after(&mut self) -> std::io::Result<()> {
        match self.current {
            b'{' | b'(' | b',' => {
                self.write_newline()?;
                self.write_indent()?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn write_indent(&mut self) -> std::io::Result<()> {
        let buf = self.indent.as_bytes();
        for _ in 0..self.indent_level {
            self.writer.write_all(buf)?;
        }
        Ok(())
    }
}
