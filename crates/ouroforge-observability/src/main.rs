use std::process::ExitCode;

fn main() -> ExitCode {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command, bundle] if command == "validate" => {
            match ouroforge_observability::validate_bundle(bundle) {
                Ok(report) => {
                    println!(
                        "valid observability bundle: run_id={} run_kind={} artifacts={}",
                        report.run_id, report.run_kind, report.artifact_count
                    );
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("invalid observability bundle: {error:#}");
                    ExitCode::from(1)
                }
            }
        }
        [command, bundle] if command == "render-verdict" => {
            render_verdict(bundle, "1970-01-01T00:00:00Z", false)
        }
        [command, bundle, flag] if command == "render-verdict" && flag == "--write" => {
            render_verdict(bundle, "1970-01-01T00:00:00Z", true)
        }
        [command, bundle, flag, generated_at]
            if command == "render-verdict" && flag == "--generated-at" =>
        {
            render_verdict(bundle, generated_at, false)
        }
        [command, bundle, flag, generated_at, write]
            if command == "render-verdict" && flag == "--generated-at" && write == "--write" =>
        {
            render_verdict(bundle, generated_at, true)
        }
        _ => {
            eprintln!(
                "usage: ouroforge-observability validate <bundle-root>\n       ouroforge-observability render-verdict <bundle-root> [--generated-at <timestamp>] [--write]"
            );
            ExitCode::from(2)
        }
    }
}

fn render_verdict(bundle: &str, generated_at: &str, write: bool) -> ExitCode {
    let options = ouroforge_observability::VerdictOptions {
        generated_at: generated_at.to_string(),
        ..Default::default()
    };
    match ouroforge_observability::render_verdict(bundle, &options) {
        Ok(markdown) => {
            if write {
                let path = std::path::Path::new(bundle).join("verdict.md");
                if let Err(error) = std::fs::write(&path, markdown) {
                    eprintln!("failed to write {}: {error:#}", path.display());
                    return ExitCode::from(1);
                }
            } else {
                print!("{markdown}");
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("failed to render verdict: {error:#}");
            ExitCode::from(1)
        }
    }
}
