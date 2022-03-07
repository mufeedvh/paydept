use std::fs::File;
use std::io::Result;

use colored::Colorize;
use comfy_table::*;
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;

use crate::search::DEP_SUPPORT_LIST;

/// Export results in CSV format.
pub fn export_results() -> Result<()> {
    eprintln!(
        "{}{}{} Writing results to paydept.csv.",
        "[".bold(),
        "INFO".bold().cyan(),
        "]".bold()
    );

    let fd = File::create("paydept.csv")?;
    
    let mut csv_wtr = csv::Writer::from_writer(fd);
    csv_wtr.write_record(&["Projects", "Support Links"])?;

    for (key, value) in DEP_SUPPORT_LIST.lock().iter() {
        csv_wtr.write_record(&[key, value])?;
    }

    csv_wtr.flush().unwrap();

    eprintln!(
        "{}{}{} Results exported to paydept.csv.",
        "[".bold(),
        "SUCCESS".bold().green(),
        "]".bold()
    );

    Ok(())
}

/// Show the results in a beautiful table interface to stdout.
pub fn show_results() -> Result<()> {
    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Projects")
                .add_attribute(Attribute::Bold)
                .fg(Color::Yellow),
            Cell::new("Support Links")
                .add_attribute(Attribute::Bold)
                .fg(Color::Yellow)
        ]);

    for (key, value) in DEP_SUPPORT_LIST.lock().iter() {
        table.add_row(vec![
            Cell::new(key).add_attribute(Attribute::Bold).fg(Color::White),
            Cell::new(value).fg(Color::Cyan)
        ]);
    }

    println!("{table}");

    Ok(())
}