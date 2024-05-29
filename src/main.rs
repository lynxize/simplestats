use clap::{Parser, Subcommand};
use rocm_smi_lib::RocmSmi;
use sysinfo::{Components, System, MINIMUM_CPU_UPDATE_INTERVAL};

#[derive(Parser, Debug)]
#[command(about = "Simple system stats gathering tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    CpuUsage,
    GpuUsage,
    RamUsage,
    Temp {
        device: String
    },
}

fn main() {
    let args = Cli::parse();

    let mut sys = System::new();

    match args.command {
        Commands::CpuUsage => {
            sys.refresh_cpu();
            std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
            sys.refresh_cpu();
            println!("{}", sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32);
        }
        Commands::GpuUsage => {
            let mut rocm = RocmSmi::init().unwrap();
            let usage = rocm.get_device_busy_percent(0).unwrap();
            println!("{}", usage);
        }
        Commands::RamUsage => {
            sys.refresh_memory();
            println!("{}", (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0);
        }
        Commands::Temp { device } => {
            let components = Components::new_with_refreshed_list();
            dbg!(&components);
            if let Some(component) = components.iter().find(|it| it.label().contains(&device)) {
                println!("{}", component.temperature())
            }
        }
    }
}
