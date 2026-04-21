use clap::Parser;
use pulldown_cmark::{Event, Parser as MdParser, Tag, TagEnd};
use std::fs;
use std::io::{self, stdout, Write};
use std::path::PathBuf;
use std::time::Duration;
use std::thread;

#[derive(Parser)]
#[command(name = "mdv")]
#[command(about = "Live markdown preview", long_about = None)]
struct Cli {
    #[arg(index = 1)]
    file: PathBuf,
}

fn render_md(content: &str) -> String {
    let parser = MdParser::new(content);
    let mut out = String::new();
    let mut in_code = false;
    let mut code_buf = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { .. }) => out.push_str("\x1b[36;1m"),
            Event::End(TagEnd::Heading(_)) => out.push_str("\x1b[0m\n"),
            Event::Start(Tag::CodeBlock(_)) => { in_code = true; code_buf.clear(); out.push_str("\x1b[48;5;8m"); }
            Event::End(TagEnd::CodeBlock) => { in_code = false; out.push_str("\x1b[0m\n"); }
            Event::Start(Tag::Emphasis) => out.push_str("\x1b[3m"),
            Event::End(TagEnd::Emphasis) => out.push_str("\x1b[0m"),
            Event::Start(Tag::Strong) => out.push_str("\x1b[1m"),
            Event::End(TagEnd::Strong) => out.push_str("\x1b[0m"),
            Event::Text(text) => {
                if in_code { code_buf.push_str(&text); code_buf.push('\n'); }
                else { out.push_str(&text); }
            }
            Event::Code(code) => {
                out.push_str("\x1b[48;5;8m\x1b[38;5;15m\x1b[1m");
                out.push_str(&code);
                out.push_str("\x1b[0m");
            }
            Event::SoftBreak | Event::HardBreak => out.push('\n'),
            _ => {}
        }
    }
    if in_code { out.push_str(&code_buf); out.push_str("\x1b[0m"); }
    out
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    
    print!("\x1b[?1049h\x1b[2J\x1b[H");
    stdout().flush()?;

    loop {
        let content = fs::read_to_string(&cli.file)?;
        let (cols, rows) = crossterm::terminal::size().unwrap_or((80, 24));
        let half = (cols / 2) as usize;

        print!("\x1b[2J");
        
        print!("\x1b[1;44;37m EDIT {:width$} \x1b[0m\x1b[42;30m PREVIEW {:width$} \x1b[0m\n", 
            cli.file.display(), cli.file.display(), width = half.saturating_sub(10));
        
        for r in 2..rows {
            print!("\x1b[{};{}H|\x1b[{};{}H|", r, half, r, half + 1);
        }
        println!();

        for (i, line) in content.lines().take((rows as usize) - 3).enumerate() {
            print!("\x1b[{};1H{}", i + 2, line);
        }

        let rendered = render_md(&content);
        for (i, line) in rendered.lines().take((rows as usize) - 3).enumerate() {
            print!("\x1b[{};{}H{}", i + 2, half + 2, line);
        }

        stdout().flush()?;

        thread::sleep(Duration::from_millis(500));
    }
}