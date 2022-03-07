mod walk;
mod search;
mod results;

use std::env;

fn main() -> std::io::Result<()> {
    // walk directories
    let (readme_files, funding_files) = walk::walk_dir()?;

    // parse `FUNDING.yml` files
    search::funding_yml_files(funding_files)?;

    // parse URLs from `README.md` files
    search::parse_urls(readme_files)?;

    // CLI
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if args[1].as_str() == "export" {
            results::export_results()?
        }
    } else {
        results::show_results()?
    }

    Ok(())
}