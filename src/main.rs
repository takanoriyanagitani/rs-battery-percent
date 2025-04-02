use std::process::ExitCode;

use std::io;

use starship_battery::Battery;

fn binfo() -> Result<Battery, io::Error> {
    rs_battery_percent::battery_info()
}

fn print_binfo() -> Result<(), io::Error> {
    let bi: Battery = binfo()?;
    rs_battery_percent::print_battery(&bi)
}

fn main() -> ExitCode {
    print_binfo()
        .map(|_| ExitCode::SUCCESS)
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            ExitCode::FAILURE
        })
}
