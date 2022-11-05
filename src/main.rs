use copefmt::Formatter;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let input = File::open("data/simple.groq")?;
    let output = std::io::stdout();
    Formatter::new(input, output).format()
}
