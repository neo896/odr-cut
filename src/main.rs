mod handle_xml;
mod rm;

use clap::Parser;
use handle_xml::xml_parse;
use rm::cal_road_id;

/// A Quick OpenDRIVE Map Cut Tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the OpenDRIVE file
    #[arg(short, long)]
    xodr: String,
    /// Path of the host car's position .txt file
    #[arg(short, long)]
    position: String,
}

fn main() {
    let args = Args::parse();
    let odr_path = args.xodr.to_string();
    let position_path = args.position.to_string();
    match cal_road_id(position_path, &odr_path) {
        Ok(road_id) => match xml_parse(odr_path, road_id) {
            Ok(()) => (),
            Err(e) => println!("{:?}", e),
        },
        Err(e) => println!("{:?}", e),
    }
}
