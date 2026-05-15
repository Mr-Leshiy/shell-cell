//! Headless implementation of `scell ls`: prints a plain-text table of
//! Shell-Cell containers to stdout.

use std::fmt::Write as _;

use crate::buildkit::{BuildKitD, container_info::SCellContainerInfo};

pub async fn ls(buildkit: &BuildKitD) -> color_eyre::Result<()> {
    let containers = buildkit.list_containers().await?;
    if containers.is_empty() {
        println!("No Shell-Cell containers");
        return Ok(());
    }

    let rows: Vec<[String; 5]> = containers
        .iter()
        .map(|c| {
            [
                SCellContainerInfo::container_name(&c.id, c.service_name.as_ref()),
                c.status.to_string(),
                c.target
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
                c.location
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_default(),
                if c.orphan { "orphan" } else { "" }.to_string(),
            ]
        })
        .collect();

    let headers = ["NAME", "STATUS", "TARGET", "LOCATION", "FLAGS"];
    let mut widths: [usize; 5] = headers.map(str::len);
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            if let Some(w) = widths.get_mut(i)
                && cell.len() > *w
            {
                *w = cell.len();
            }
        }
    }

    let print_row = |cells: &[String; 5]| {
        let mut line = String::new();
        for (i, cell) in cells.iter().enumerate() {
            let pad = widths.get(i).copied().unwrap_or(0);
            if i > 0 {
                line.push_str("  ");
            }
            // Padding write to in-memory String can't fail.
            let _ = write!(line, "{cell:<pad$}");
        }
        println!("{}", line.trim_end());
    };

    let header_row: [String; 5] = headers.map(String::from);
    print_row(&header_row);
    for row in &rows {
        print_row(row);
    }
    Ok(())
}
