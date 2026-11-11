use std::{
    env, fs,
    io::{self, Read},
    path::Path,
    process::{Command, ExitCode},
};

use lang2lua::{HYPRLAND_RULESET, convert};

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("lang2lua: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode, String> {
    let mut paths = Vec::new();
    let mut check = false;
    let mut offline = false;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--check" => check = true,
            "--offline" => offline = true,
            "-h" | "--help" => {
                print_help();
                return Ok(ExitCode::SUCCESS);
            }
            _ if arg.starts_with('-') => return Err(format!("unknown option: {arg}")),
            _ => paths.push(arg),
        }
    }
    let (input_path, output_path) = positional_paths(&paths)?;
    if let (Some(input), Some(output)) = (input_path, output_path)
        && same_file(input, output)
    {
        return Err("input and output must be different files".into());
    }

    if !offline {
        warn_if_ruleset_is_stale();
    }

    let input = match input_path {
        Some(path) => {
            fs::read_to_string(path).map_err(|error| format!("could not read {path}: {error}"))?
        }
        None => {
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .map_err(|error| format!("could not read stdin: {error}"))?;
            input
        }
    };

    let converted = convert(&input)?;
    if let Some(path) = output_path {
        fs::write(path, &converted.lua)
            .map_err(|error| format!("could not write {path}: {error}"))?;
    } else {
        print!("{}", converted.lua);
    }
    for warning in &converted.warnings {
        eprintln!("lang2lua: line {}: {}", warning.line, warning.message);
    }

    Ok(if check && !converted.warnings.is_empty() {
        ExitCode::from(2)
    } else {
        ExitCode::SUCCESS
    })
}

fn positional_paths(paths: &[String]) -> Result<(Option<&str>, Option<&str>), String> {
    if paths.len() > 2 {
        return Err("expected at most an input and output file".into());
    }
    Ok((
        paths.first().map(String::as_str),
        paths.get(1).map(String::as_str),
    ))
}

fn same_file(input: &str, output: &str) -> bool {
    if Path::new(input) == Path::new(output) {
        return true;
    }
    matches!((fs::canonicalize(input), fs::canonicalize(output)), (Ok(input), Ok(output)) if input == output)
}

fn warn_if_ruleset_is_stale() {
    let output = Command::new("curl")
        .args([
            "--silent",
            "--show-error",
            "--location",
            "--max-time",
            "2",
            "--output",
            "/dev/null",
            "--write-out",
            "%{url_effective}",
            "https://github.com/hyprwm/Hyprland/releases/latest",
        ])
        .output();

    let Ok(output) = output else { return };
    if !output.status.success() {
        return;
    }
    let url = String::from_utf8_lossy(&output.stdout);
    let latest = latest_tag(&url).unwrap_or_default();
    if latest.starts_with('v') && latest != format!("v{HYPRLAND_RULESET}") {
        eprintln!(
            "lang2lua: warning: rules target Hyprland v{HYPRLAND_RULESET}, but {latest} is latest; please reach out to @vanshxgoel on Twitter/X and ask them to take a look"
        );
    }
}

fn latest_tag(url: &str) -> Option<&str> {
    url.trim().trim_end_matches('/').rsplit('/').next()
}

fn print_help() {
    println!(
        "lang2lua [--check] [--offline] [INPUT [OUTPUT]]\n\nConvert INPUT to OUTPUT. Without OUTPUT, write to stdout; without INPUT, read stdin.\n\n  --check    exit 2 when manual-review warnings are emitted\n  --offline  skip the best-effort Hyprland release check"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_latest_release_tag() {
        assert_eq!(
            latest_tag("https://github.com/hyprwm/Hyprland/releases/tag/v0.57.2"),
            Some("v0.57.2")
        );
    }

    #[test]
    fn accepts_input_and_output_paths() {
        let paths = vec!["input.conf".into(), "output.lua".into()];
        assert_eq!(
            positional_paths(&paths),
            Ok((Some("input.conf"), Some("output.lua")))
        );
        assert!(same_file("input.conf", "input.conf"));
    }

    #[test]
    fn rejects_extra_paths() {
        let paths = vec!["one".into(), "two".into(), "three".into()];
        assert!(positional_paths(&paths).is_err());
    }
}
