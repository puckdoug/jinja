use clap::Parser;
use minijinja::Environment;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "jinja")]
#[command(about = "A command-line jinja2 template processor", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    /// Template file(s) to process
    #[arg(required = true)]
    templates: Vec<PathBuf>,

    /// Input file (defaults to stdin)
    #[arg(long)]
    input: Option<PathBuf>,

    /// Output file (defaults to stdout)
    #[arg(long)]
    output: Option<PathBuf>,

    /// Substitutions file (JSON format)
    #[arg(long)]
    substitutions: Option<PathBuf>,

    /// Enable verbose output
    #[arg(long, short)]
    verbose: bool,

    /// Enable debug mode
    #[arg(long)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read template content - from --input, template file, or stdin
    let template_content = if let Some(input_file) = &args.input {
        fs::read_to_string(input_file)?
    } else {
        let template_path = &args.templates[0];
        if template_path.exists() {
            fs::read_to_string(template_path)?
        } else {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    // Build variables from environment and substitutions
    let mut variables: HashMap<String, String> = env::vars().collect();

    if let Some(subs_file) = &args.substitutions {
        let subs_content = fs::read_to_string(subs_file)?;
        for line in subs_content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();
                let value = value.trim().trim_matches('"').to_string();
                variables.insert(key, value);
            }
        }
    }

    // Render the template
    let mut env = Environment::new();
    env.add_template("template", &template_content)?;
    let tmpl = env.get_template("template")?;
    let rendered = tmpl.render(&variables)?;

    // Write output
    if let Some(output_file) = &args.output {
        fs::write(output_file, &rendered)?;
    } else {
        print!("{}", rendered);
    }

    Ok(())
}
