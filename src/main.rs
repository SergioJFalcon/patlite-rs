use rusb::{Context, DeviceHandle, Result};
use clap::{arg, command, ArgMatches, Command};
use tabled::{builder::Builder, settings::Style};
use patlite_rs::{get_settings, print_device_info, set_blank, set_buzz_command, set_light_command, set_master_controls_command, set_volume_command, setup_device};

fn main() -> Result<()> {
    let matches: ArgMatches = command!()
    .version("1.0")
    .about("Patlite NE-SN-USB CLI Tool")
    .propagate_version(true)
    // .subcommand_required(true)
    // .arg_required_else_help(true)
    .subcommand(
      Command::new("master")
      .about("Master control to set light and buzzer controls")
      .arg(
        arg!([COLOR] "Color to set the light to")
          .value_parser(clap::value_parser!(u8).range(0..16))
          .default_value("0")
      )
      .arg(
        arg!([COLORPATTERN] "Pattern to set the light to")
          .value_parser(clap::value_parser!(u8).range(0..16))
          .default_value("0")
      )
      .arg(
        arg!([BUZZERPATTERN] "Pattern to set the buzzer to")
          .value_parser(clap::value_parser!(u8).range(0..16))
          .default_value("0")
      )
      .arg(
        arg!([REPETITION] "Number of times to repeat the buzzer")
          .value_parser(clap::value_parser!(u8).range(0..16))
          .default_value("0")
      )
      .arg(
        arg!([VOLUME] "Volume level to set the buzzer to")
          .value_parser(clap::value_parser!(u8).range(0..16))
          .default_value("0")
      )
    )
    .subcommand(
  Command::new("light")
      .about("Light control")
      .arg(
        arg!([COLOR] "Color to set the light to")
          .value_parser(clap::value_parser!(u8).range(0..16))
          .default_value("0")
      )
      .arg(
        arg!([PATTERN] "Pattern to set the light to")
          .value_parser(clap::value_parser!(u8).range(0..16))
          .default_value("0")
      ),
    )
    .subcommand(
      Command::new("buzz")
      .about("Create a buzzer")
      .arg(
        arg!([PATTERN] "Pattern to set the buzz to")
        .value_parser(clap::value_parser!(u8).range(0..16))
        .default_value("0")
      )
      .arg(
        arg!([REPETITION] "Number of times to repeat the buzz")
        .value_parser(clap::value_parser!(u8).range(0..16))
        .default_value("0")
      )
      .arg(
        arg!([VOLUME] "Volume level to set the buzz to")
        .value_parser(clap::value_parser!(u8).range(0..16))
        .default_value("0")
      ),
    )
    .subcommand(
      Command::new("volume")
      .about("Set the volume level")
      .arg(
        arg!([LEVEL] "Volume level to set")
          .value_parser(clap::value_parser!(u8).range(0..16))
          .default_value("0")
      ),
    )
    .subcommand(
      Command::new("state")
      .about("Get the current state of the device")
    )
    .subcommand(
      Command::new("off")
      .about("Set the device to default state")
    )
    .subcommand(
      Command::new("info")
      .about("Get information on available colors, led patterns, buzzer patterns, and volume levels")
      .after_help("AFTER HELP")
      .arg(
        arg!([CONTROL] "Control to get information on")
          .value_parser(["color", "led", "buzzer", "volume", "device", "all"])
          .required(false)
      )
    )
    .get_matches();

    match matches.subcommand() {
        Some(("master", sub_matches)) => {
            let color: &u8 = sub_matches
                .get_one::<u8>("COLOR")
                .expect("Color is required");
            let color_pattern: &u8 = sub_matches
                .get_one::<u8>("COLORPATTERN")
                .expect("Color Pattern is required");
            let buzzer_pattern: &u8 = sub_matches
                .get_one::<u8>("BUZZERPATTERN")
                .expect("Buzzer Pattern is required");
            let volume: &u8 = sub_matches
                .get_one::<u8>("VOLUME")
                .expect("Volume is required");
            let repetition: &u8 = sub_matches
                .get_one::<u8>("REPETITION")
                .expect("Repetition is required");

            let mut handle: DeviceHandle<Context> = setup_device()?;
            set_master_controls_command(
                &mut handle,
                color,
                color_pattern,
                buzzer_pattern,
                repetition,
                volume,
            )?;
        }
        Some(("light", sub_matches)) => {
            let color: &u8 = sub_matches
                .get_one::<u8>("COLOR")
                .expect("Color is required");
            let pattern: &u8 = sub_matches
                .get_one::<u8>("PATTERN")
                .expect("Pattern is required");

            let mut handle: DeviceHandle<Context> = setup_device()?;
            set_light_command(&mut handle, color, pattern)?;
        }
        Some(("buzz", sub_matches)) => {
            let buzzer_pattern: &u8 = sub_matches
                .get_one::<u8>("PATTERN")
                .expect("Buzzer Pattern is required");
            let volume: &u8 = sub_matches
                .get_one::<u8>("VOLUME")
                .expect("Volume is required");
            let repetition: &u8 = sub_matches
                .get_one::<u8>("REPETITION")
                .expect("Repetition is required");

            let mut handle: DeviceHandle<Context> = setup_device()?;
            set_buzz_command(&mut handle, buzzer_pattern, repetition, volume)?;
        }
        Some(("volume", sub_matches)) => {
            let level: &u8 = sub_matches
                .get_one::<u8>("LEVEL")
                .expect("Level is required");

            let mut handle: DeviceHandle<Context> = setup_device()?;
            set_volume_command(&mut handle, level)?;
        }
        Some(("state", _)) => {
            let mut handle: DeviceHandle<Context> = setup_device()?;
            get_settings(&mut handle)?;
        }
        Some(("off", _)) => {
            let mut handle: DeviceHandle<Context> = setup_device()?;
            set_blank(&mut handle)?;
        }
        Some(("info", sub_matches)) => {
            let control: &String = sub_matches
                .get_one::<String>("CONTROL")
                .expect("Control type is required");
            let mut builder: Builder = Builder::new();

            match control.as_str() {
                "color" => {
                    builder.push_record(["Color", "Value"]);
                    builder.push_record(["Off", "0"]);
                    builder.push_record(["Red", "1"]);
                    builder.push_record(["Green", "2"]);
                    builder.push_record(["Yellow", "3"]);
                    builder.push_record(["Blue", "4"]);
                    builder.push_record(["Purple", "5"]);
                    builder.push_record(["Sky Blue", "6"]);
                    builder.push_record(["White", "7"]);
                    builder.push_record(["Keep", "8 - 15"]);
                    let table: String = builder.build().with(Style::rounded()).to_string();
                    println!("{}", table);
                }
                "led" => {
                    builder.push_record(["LED Pattern", "Value"]);
                    builder.push_record(["Off", "0"]);
                    builder.push_record(["On", "1"]);
                    builder.push_record(["Pattern 1", "2"]);
                    builder.push_record(["Pattern 2", "3"]);
                    builder.push_record(["Pattern 3", "4"]);
                    builder.push_record(["Pattern 4", "5"]);
                    builder.push_record(["Pattern 5", "6"]);
                    builder.push_record(["Pattern 6", "7"]);
                    builder.push_record(["Keep", "8 - 15"]);
                    let table: String = builder.build().with(Style::rounded()).to_string();
                    println!("{}", table);
                }
                "buzzer" => {
                    builder.push_record(["Buzzer Pattern", "Value"]);
                    builder.push_record(["Off", "0"]);
                    builder.push_record(["Continuously On", "1"]);
                    builder.push_record(["Sweep", "2"]);
                    builder.push_record(["Intermittent", "3"]);
                    builder.push_record(["Weak Attention", "4"]);
                    builder.push_record(["Strong Attention", "5"]);
                    builder.push_record(["Shining Star Melody", "6"]);
                    builder.push_record(["London Bridge Melody", "7"]);
                    builder.push_record(["Keep", "8 - 15"]);
                    let table: String = builder.build().with(Style::rounded()).to_string();
                    println!("{}", table);
                }
                "volume" => {
                    builder.push_record(["Volume Level", "Value"]);
                    builder.push_record(["Silent", "0"]);
                    builder.push_record(["1", "1"]);
                    builder.push_record(["2", "2"]);
                    builder.push_record(["3", "3"]);
                    builder.push_record(["4", "4"]);
                    builder.push_record(["5", "5"]);
                    builder.push_record(["6", "6"]);
                    builder.push_record(["7", "7"]);
                    builder.push_record(["8", "8"]);
                    builder.push_record(["9", "9"]);
                    builder.push_record(["Max", "10"]);
                    builder.push_record(["Keep", "11 - 15"]);
                    let table: String = builder.build().with(Style::rounded()).to_string();
                    println!("{}", table);
                }
                "device" => {
                    let mut handle: DeviceHandle<Context> = setup_device()?;
                    print_device_info(&mut handle, &mut builder)?;
                }
                "all" => {
                    builder.push_record([
                        "Color",
                        "Value",
                        "LED Pattern",
                        "Value",
                        "Buzzer Pattern",
                        "Value",
                        "Volume Level",
                        "Value",
                    ]);
                    builder.push_record(["Off", "0", "Off", "0", "Off", "0", "Silent", "0"]);
                    builder.push_record(["Red", "1", "On", "1", "Continuously On", "1", "1", "1"]);
                    builder.push_record(["Green", "2", "Pattern 1", "2", "Sweep", "2", "2", "2"]);
                    builder.push_record([
                        "Yellow",
                        "3",
                        "Pattern 2",
                        "3",
                        "Intermittent",
                        "3",
                        "3",
                        "3",
                    ]);
                    builder.push_record([
                        "Blue",
                        "4",
                        "Pattern 3",
                        "4",
                        "Weak Attention",
                        "4",
                        "4",
                        "4",
                    ]);
                    builder.push_record([
                        "Purple",
                        "5",
                        "Pattern 4",
                        "5",
                        "Strong Attention",
                        "5",
                        "5",
                        "5",
                    ]);
                    builder.push_record([
                        "Sky Blue",
                        "6",
                        "Pattern 5",
                        "6",
                        "Shining Star Melody",
                        "6",
                        "6",
                        "6",
                    ]);
                    builder.push_record([
                        "White",
                        "7",
                        "Pattern 6",
                        "7",
                        "London Bridge Melody",
                        "7",
                        "7",
                        "7",
                    ]);
                    builder.push_record([
                        "Keep", "8 - 15", "Keep", "8 - 15", "Keep", "8 - 15", "Keep", "8 - 15",
                    ]);
                    let table: String = builder.build().with(Style::rounded()).to_string();
                    println!("{}", table);
                }
                _ => println!("Invalid control type"),
            }
        }
        _ => {
            unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`")
        }
    }
    // cleanup after use
    // handle.release_interface(endpoint.iface)?;
    println!("Completed!~");
    Ok(())
}
