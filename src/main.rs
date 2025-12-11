use clap::Parser;
use colored::*;
use reqwest::blocking::Client;
use serde::Serialize;
use std::{
    env,
    error::Error,
    fs,
    io::{self, Write},
    path::PathBuf,
};

const DEFAULT_PERSONALITY: &str = r#"
You are a helpful assistant with the following style:
- Use quick, clever humor when appropriate.
- You can swear, but use profanity sparingly for emphasis, not constantly.
- You have a dark sense of humor and a skeptical, questioning attitude.
- You do NOT act like Gen Z or use TikTok slang.
- You are direct, blunt, and honest, but not cruel. 
- You still follow safety rules and avoid encouraging harmful or hateful behavior.
"#;

/// Tiny ChatGPT CLI
#[derive(Parser, Debug)]
#[command(author, version, about = "ChatGPT in your terminal", long_about = None)]
struct Args {
    /// Run in interactive REPL mode
    #[arg(long)]
    repl: bool,

    /// One or more files to include in the prompt
    #[arg(short, long)]
    file: Vec<PathBuf>,

    /// One-shot prompt (ignored in --repl mode if empty)
    ///
    /// Example:
    ///   gpt "Write me a haiku"
    ///
    /// With files:
    ///   gpt -f src/main.rs "Explain this code"
    #[arg()]
    prompt: Option<String>,
}

#[derive(Serialize)]
struct RequestBody<'a> {
    model: &'a str,
    input: &'a str,
    instructions: &'a str,
}

fn call_chatgpt(prompt: &str) -> Result<String, Box<dyn Error>> {
    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|_| "OPENAI_API_KEY env var not set. Export it first.")?;

    let client = Client::new();

    let body = RequestBody {
        // change this to whatever model you actually have access to
        model: "gpt-4.1-mini",
        input: prompt,
        instructions: DEFAULT_PERSONALITY
    };

    let res = client
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(api_key)
        .json(&body)
        .send()?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().unwrap_or_default();
        return Err(format!("API error: {status} - {text}").into());
    }

    let parsed: serde_json::Value = res.json()?;

    // println!("{parsed:#}"); // uncomment once if you want to inspect the raw JSON

    // Responses API shape:
    // output[0].content[0].text
    let text = parsed["output"][0]["content"][0]["text"]
        .as_str()
        .unwrap_or("<no text in response>")
        .to_string();

    Ok(text)
}

/// Read all files passed via -f/--file and build a context string.
fn build_file_context(files: &[PathBuf]) -> Result<String, Box<dyn Error>> {
    if files.is_empty() {
        return Ok(String::new());
    }

    let mut ctx = String::new();

    for path in files {
        let content = fs::read_to_string(path)?;
        use std::fmt::Write as FmtWrite;

        writeln!(
            &mut ctx,
            "File: {}\n```text\n{}\n```\n",
            path.display(),
            content
        )?;
    }

    Ok(ctx)
}

fn run_one_shot(args: Args) -> Result<(), Box<dyn Error>> {
    let file_ctx = build_file_context(&args.file)?;

    let base_prompt = args
        .prompt
        .unwrap_or_else(|| "Explain the provided files.".to_string());

    let final_prompt = if file_ctx.is_empty() {
        base_prompt.clone()
    } else {
        format!(
            "{}\n\nUser request:\n{}",
            file_ctx,
            base_prompt
        )
    };

    println!("{}", "You:".bright_blue().bold());
    println!("{}", base_prompt);

    let reply = call_chatgpt(&final_prompt)?;
    println!("\n{}", "Assistant:".bright_green().bold());
    println!("{}", reply.bright_green());

    Ok(())
}

fn run_repl(args: Args) -> Result<(), Box<dyn Error>> {
    let file_ctx = build_file_context(&args.file)?;

    println!("{}", "Entering REPL mode. Type :q or :quit to exit.".yellow());
    if !file_ctx.is_empty() {
        println!(
            "{}",
            "File context loaded and will be included with each message."
                .yellow()
        );
    }

    let mut input = String::new();

    loop {
        print!("{}", "You > ".bright_blue().bold());
        io::stdout().flush()?;

        input.clear();
        let n = io::stdin().read_line(&mut input)?;
        if n == 0 {
            // EOF (Ctrl+D)
            break;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == ":q" || trimmed == ":quit" {
            break;
        }

        let final_prompt = if file_ctx.is_empty() {
            trimmed.to_string()
        } else {
            format!(
                "{}\n\nUser request:\n{}",
                file_ctx,
                trimmed
            )
        };

        let reply = match call_chatgpt(&final_prompt) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{} {}", "Error:".red().bold(), e);
                continue;
            }
        };

        println!("\n{}", "Assistant:".bright_green().bold());
        println!("{}\n", reply.bright_green());
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.repl {
        run_repl(args)
    } else {
        run_one_shot(args)
    }
}
