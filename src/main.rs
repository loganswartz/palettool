use std::{error::Error, str::FromStr, thread};
use clap::Parser;
use color::{get_averages_for_regions_in, to_hex};
use region::Region;

mod region;
mod color;

/// A tool for creating palettes, based off of the averaged colors of regions from a set of images.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    file: Vec<String>,

    /// A pixel region to capture from each input file.
    #[arg(short, long)]
    region: Vec<String>,

    /// Dump the output to stdout as a CSV.
    #[arg(short, long)]
    csv: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    eprintln!("Parsing {} files...", args.file.len());
    let regions = args.region.iter().map(|reg| Region::from_str(reg)).collect::<Result<Vec<Region>, _>>()?;

    if regions.is_empty() {
        return Err("You must specify at least one region!".into());
    }

    let mut threads = vec![];
    for file in args.file {
        let regions = regions.clone();
        threads.push(thread::spawn(move || {
            get_averages_for_regions_in(file, &regions)
        }))
    }

    let results  = threads.into_iter().map(|c| c.join().unwrap()).collect::<Result<Vec<_>, _>>()?;

    if args.csv {
        let mut index = 1;
        let labels: Vec<String> = regions.iter().map(|rect| rect.label.clone().unwrap_or_else(|| {
            let generated = format!("Unlabelled {}", index);
            index += 1;

            generated
        })).collect();

        println!("{}", labels.join(","));
    }

    for (_, triplets) in results {
        // eprintln!("Palette for {}:", filename);

        let separator = if args.csv { "," } else { "\n" };
        println!("{}", triplets.iter().map(to_hex).collect::<Vec<String>>().join(separator));

        // insert a blank line
        if !args.csv {
            println!();
        }
    }

    Ok(())
}
