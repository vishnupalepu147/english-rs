use std::io::{self, IsTerminal, Read, Write};
use std::process::ExitCode;
use std::time::Instant;

use englishc::render::{self, Ansi, Painter, Plain, Renderer, Style};
use englishc::{Config, Level, explain, lints};

const USAGE: &str = "\
Usage: englishc [OPTIONS] [FILE]...

Compiles English (C edition) prose and reports what English (Rust edition)
would reject. Reads standard input when no file is given, or for `-`.

Options:
      --fix                Apply machine-applicable suggestions in place
                           (with standard input, print the fixed text)
      --explain <CODE>     Explain an error code, such as E0001
  -A, --allow <LINT>       Allow a lint (a name, a code, or `warnings`)
  -D, --deny <LINT>        Turn a warning into an error
  -W help                  List every lint
      --error-format <F>   `human` (default) or `short`
      --color <WHEN>       `auto` (default), `always` or `never`
  -V, --version            Print version information
  -h, --help               Print this help

Inside a document, `#![allow(lint)]` on its own line allows a lint for the
whole file and `#[allow(lint)]` allows it for the next line.";

struct Args {
    files: Vec<String>,
    fix: bool,
    short: bool,
    color: Option<bool>,
    config: Config,
}

enum Command {
    Compile(Args),
    Explain(String),
    ListLints,
    Print(String),
}

fn parse() -> Result<Command, String> {
    let mut args = Args {
        files: Vec::new(),
        fix: false,
        short: false,
        color: None,
        config: Config::default(),
    };
    let mut iter = std::env::args().skip(1);
    while let Some(arg) = iter.next() {
        let mut value = |name: &str| {
            iter.next()
                .ok_or_else(|| format!("option `{name}` requires a value"))
        };
        match arg.as_str() {
            "-h" | "--help" => return Ok(Command::Print(USAGE.to_string())),
            "-V" | "--version" => {
                return Ok(Command::Print(format!(
                    "englishc {} (english 2026 edition)",
                    env!("CARGO_PKG_VERSION")
                )));
            }
            "--fix" => args.fix = true,
            "--explain" => return Ok(Command::Explain(value("--explain")?)),
            "-A" | "--allow" => args.config.allow.push(value(&arg)?.replace('-', "_")),
            "-D" | "--deny" => args.config.deny.push(value(&arg)?.replace('-', "_")),
            "-W" | "--warn" => {
                if value(&arg)? == "help" {
                    return Ok(Command::ListLints);
                }
            }
            "--error-format" => match value("--error-format")?.as_str() {
                "human" => args.short = false,
                "short" => args.short = true,
                other => return Err(format!("unknown error format `{other}`")),
            },
            "--color" => {
                args.color = match value("--color")?.as_str() {
                    "always" => Some(true),
                    "never" => Some(false),
                    "auto" => None,
                    other => return Err(format!("unknown color choice `{other}`")),
                }
            }
            s if s.starts_with("-A") && s.len() > 2 => {
                args.config.allow.push(s[2..].replace('-', "_"))
            }
            s if s.starts_with("-D") && s.len() > 2 => {
                args.config.deny.push(s[2..].replace('-', "_"))
            }
            s if s.starts_with('-') && s != "-" => return Err(format!("unknown option `{s}`")),
            _ => args.files.push(arg),
        }
    }
    if args.files.is_empty() {
        args.files.push("-".to_string());
    }
    Ok(Command::Compile(args))
}

fn list_lints(painter: &dyn Painter) -> String {
    let width = lints::LINTS.iter().map(|l| l.name.len()).max().unwrap_or(0);
    let mut out = String::from("\nLints provided by englishc:\n\n");
    out += &format!(
        "    {:>width$}  {:<7}  {:<5}  meaning\n",
        "name", "default", "code"
    );
    out += &format!(
        "    {:>width$}  {:<7}  {:<5}  -------\n",
        "----", "-------", "----"
    );
    for lint in lints::LINTS {
        let level = match lint.level {
            Level::Error => painter.paint(Style::Error, "error  "),
            Level::Warning => painter.paint(Style::Warning, "warn   "),
        };
        out += &format!(
            "    {:>width$}  {level}  {:<5}  {}\n",
            lint.name,
            lint.code.unwrap_or(""),
            lint.summary
        );
    }
    out
}

fn main() -> ExitCode {
    let command = match parse() {
        Ok(command) => command,
        Err(message) => {
            eprintln!(
                "{}: {message}\n\n{USAGE}",
                Ansi.paint(Style::Error, "error")
            );
            return ExitCode::from(2);
        }
    };
    let color_default = io::stderr().is_terminal() && std::env::var_os("NO_COLOR").is_none();
    match command {
        Command::Print(text) => {
            println!("{text}");
            ExitCode::SUCCESS
        }
        Command::ListLints => {
            let painter: &dyn Painter = if io::stdout().is_terminal() {
                &Ansi
            } else {
                &Plain
            };
            print!("{}", list_lints(painter));
            ExitCode::SUCCESS
        }
        Command::Explain(code) => match explain::explain(&code) {
            Some(text) => {
                println!("{text}");
                ExitCode::SUCCESS
            }
            None => {
                eprintln!("error: `{code}` is not a valid error code");
                ExitCode::FAILURE
            }
        },
        Command::Compile(args) => compile(args, color_default),
    }
}

fn compile(args: Args, color_default: bool) -> ExitCode {
    let painter: &dyn Painter = if args.color.unwrap_or(color_default) {
        &Ansi
    } else {
        &Plain
    };
    let started = Instant::now();
    let mut failed = false;
    let mut stderr = io::stderr().lock();
    for file in &args.files {
        let (name, src) = if file == "-" {
            let mut src = String::new();
            if let Err(e) = io::stdin().read_to_string(&mut src) {
                let _ = writeln!(
                    stderr,
                    "{}: could not read standard input: {e}",
                    painter.paint(Style::Error, "error")
                );
                return ExitCode::FAILURE;
            }
            ("<stdin>".to_string(), src)
        } else {
            match std::fs::read_to_string(file) {
                Ok(src) => (file.clone(), src),
                Err(e) => {
                    let _ = writeln!(
                        stderr,
                        "{}: couldn't read `{file}`: {e}",
                        painter.paint(Style::Error, "error")
                    );
                    failed = true;
                    continue;
                }
            }
        };

        // After a fix, report what is left.
        let current = if args.fix {
            let (fixed, count) = englishc::fix(&src, &args.config);
            if file == "-" {
                print!("{fixed}");
            } else if count > 0
                && let Err(e) = std::fs::write(file, &fixed)
            {
                let _ = writeln!(
                    stderr,
                    "{}: couldn't write `{file}`: {e}",
                    painter.paint(Style::Error, "error")
                );
                failed = true;
                continue;
            }
            let fixes = if count == 1 {
                "1 fix".to_string()
            } else {
                format!("{count} fixes")
            };
            let _ = writeln!(
                stderr,
                "{} {name} ({fixes})",
                painter.paint(Style::Success, &format!("{:>12}", "Fixed"))
            );
            fixed
        } else {
            src
        };
        let diagnostics = englishc::check(&current, &args.config);
        let renderer = Renderer::new(&current, &name, painter);
        let _ = writeln!(
            stderr,
            "{} {name} (english 2026)",
            painter.paint(Style::Success, &format!("{:>12}", "Compiling"))
        );
        for d in &diagnostics {
            let text = if args.short {
                renderer.short(d)
            } else {
                format!("{}\n", renderer.diagnostic(d))
            };
            let _ = write!(stderr, "{text}");
        }
        let errors = diagnostics
            .iter()
            .filter(|d| d.level == Level::Error)
            .count();
        let warnings = diagnostics.len() - errors;
        if let Some(summary) = render::summary(
            &name,
            errors,
            warnings,
            englishc::fixable(&diagnostics),
            painter,
        ) {
            let _ = write!(stderr, "{summary}");
        }
        failed |= errors > 0;
    }
    if !failed {
        let elapsed = started.elapsed().as_secs_f64();
        let _ = writeln!(
            stderr,
            "{} `proofread` profile [grammatical + spell-checked] target(s) in {elapsed:.2}s",
            painter.paint(Style::Success, &format!("{:>12}", "Finished"))
        );
    }
    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
