use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    match (args.next().as_deref(), args.next(), args.next()) {
        (Some("validate"), Some(bundle), None) => {
            match ouroforge_observability::validate_bundle(&bundle) {
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
        _ => {
            eprintln!("usage: ouroforge-observability validate <bundle-root>");
            ExitCode::from(2)
        }
    }
}
