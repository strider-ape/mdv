use clap::Parser;
use crossterm::{
    style::{Attribute, Color, PrintStyledContent, SetAttribute, SetBackgroundColor, SetForegroundColor, Stylize},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use pulldown_cmark::{Event, Parser as MdParser, Tag, TagEnd};
use std::fs;
use std::io::{self, stdout, Write};

#[derive(Parser)]
#[command(name = "mdv")]
#[command(about = "Render markdown to terminal", long_about = None)]
struct Cli {
    #[arg(index = 1)]
    file: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let content = fs::read_to_string(&cli.file)?;
    let parser = MdParser::new(&content);

    let mut stdout = stdout();
    stdout.queue(Clear(ClearType::All))?;

    let mut in_code_block = false;
    let mut code_buf = String::new();
    let mut in_heading = false;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                in_heading = true;
                stdout.queue(SetForegroundColor(Color::Cyan))?;
                stdout.queue(SetAttribute(Attribute::Bold))?;
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
                stdout.queue(SetAttribute(Attribute::NoBold))?;
                stdout.queue(SetForegroundColor(Color::Reset))?;
                writeln!(stdout)?;
            }
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
                code_buf.clear();
                stdout.queue(SetBackgroundColor(Color::DarkGrey))?;
            }
            Event::End(TagEnd::CodeBlock) => {
                for line in code_buf.lines() {
                    stdout.queue(SetForegroundColor(Color::White))?;
                    write!(stdout, "{}", line)?;
                    writeln!(stdout)?;
                }
                stdout.queue(SetBackgroundColor(Color::Reset))?;
                stdout.queue(SetForegroundColor(Color::Reset))?;
                writeln!(stdout)?;
            }
            Event::Start(Tag::Emphasis) => {
                stdout.queue(SetAttribute(Attribute::Italic))?;
            }
            Event::End(TagEnd::Emphasis) => {
                stdout.queue(SetAttribute(Attribute::NoItalic))?;
            }
            Event::Start(Tag::Strong) => {
                stdout.queue(SetAttribute(Attribute::Bold))?;
            }
            Event::End(TagEnd::Strong) => {
                stdout.queue(SetAttribute(Attribute::NoBold))?;
            }
            Event::Text(text) => {
                if in_code_block {
                    code_buf.push_str(&text);
                    code_buf.push('\n');
                } else if in_heading {
                    let styled = (&*text).bold().with(Color::Cyan);
                    stdout.queue(PrintStyledContent(styled))?;
                } else {
                    write!(stdout, "{}", text)?;
                }
            }
            Event::Code(code) => {
                stdout.queue(SetBackgroundColor(Color::DarkGrey))?;
                stdout.queue(SetForegroundColor(Color::White))?;
                let styled = (&*code).bold();
                stdout.queue(PrintStyledContent(styled))?;
                stdout.queue(SetBackgroundColor(Color::Reset))?;
                stdout.queue(SetForegroundColor(Color::Reset))?;
            }
            Event::SoftBreak | Event::HardBreak => {
                writeln!(stdout)?;
            }
            _ => {}
        }
    }

    stdout.queue(SetForegroundColor(Color::Reset))?;
    stdout.queue(SetBackgroundColor(Color::Reset))?;
    stdout.queue(SetAttribute(Attribute::NoBold))?;
    stdout.queue(SetAttribute(Attribute::NoItalic))?;
    stdout.flush()?;
    Ok(())
}