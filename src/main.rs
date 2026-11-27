use sigmaos_zen_browser::ZenProfile;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("help");
    let profile = ZenProfile::default();

    match cmd {
        "workspaces" => match profile.export_workspaces_json() {
            Ok(json) => println!("{}", json),
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        },
        "args" => println!("{}", profile.build_launch_args().join(" ")),
        _ => {
            println!("sigmaos-zen-browser - Zen Browser Workspace Manager");
            println!("Usage:");
            println!("  sigmaos-zen-browser workspaces");
            println!("  sigmaos-zen-browser args");
        }
    }
}
