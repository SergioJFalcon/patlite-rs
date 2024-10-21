use clap::{command, Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "Patlite NE-SN-USB")]
#[command(author = "Sergio Falcon")]
#[command(about = "Control Patlite NE-SN-USB")]
#[command(version = "1.0")]
#[command(about = "Does awesome things", long_about = None)]
#[command()]
struct Cli {
  #[arg(value_enum, default_value = "state")]
  mode: Mode,

  #[arg(short, long, requires_if("Light", "mode"), value_parser = clap::value_parser!(u8).range(0..16), default_value="15")]
  color: Option<u8>,

  #[arg(short, long, requires_if("Light", "mode"), value_parser = clap::value_parser!(u8).range(0..16), default_value="15")]
  pattern: Option<u8>,

  #[arg(short, long, default_value = "0", requires_if("Buzzer", "mode"), requires_if("Volume", "mode"))]
  volume: Option<u8>,

  #[arg(short, long, default_value = "0", requires_if("Buzzer", "mode"))]
  repetition: Option<u8>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Light,
    Buzzer,
    Volume,
    Off,
    State,
    Colors,
    Patterns,
    Volumes,
    BuzzerPatterns,
}


fn main() {
  let cli = Cli::parse();

  match cli.mode {
      Mode::Light => {
        if let (Some(color), Some(pattern)) = (cli.color, cli.pattern) {
            println!("Mode: Light, Color: {}, Pattern: {}", color, pattern);
        } else {
            println!("Mode: Light requires both color and pattern arguments.");
        }
      }
      Mode::Buzzer => {
          println!("Tortoise");
      },
      Mode::Volume => {
        if let Some(volume) = cli.volume {
          println!("Mode: Volume, volume: {}", volume);
      } else {
          println!("Mode: Volume requires volume arguments.");
      }
      },
      Mode::Off => {
        println!("Mode: Off");
          // send_command("off");
      },
      Mode::State => {
          println!("Mode: State");
      },
      Mode::Colors => {
        // Display all available colors
        println!("Mode: Colors");
      },
      Mode::Patterns => {
        // Display all available patterns
        println!("Mode: Patterns");
      },
      Mode::BuzzerPatterns => {
        // Display all available buzzer patterns
        println!("Mode: Buzzer Patterns");
      },
      Mode::Volumes => {
        // Display all available volume levels
        println!("Mode: Volumes");
      },
  }
}