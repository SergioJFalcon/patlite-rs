use clap::{arg, command, ArgMatches, Command};
use rusb::{Context, Device, DeviceHandle, Result, UsbContext};
use std::fmt;
use std::{fmt::Debug, iter, time::Duration};
use tabled::{builder::Builder, settings::Style};

mod constants;

#[derive(Debug)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
}

#[derive(Debug)]
enum LEDColors {
    Off,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    LightBlue,
    White,
    Keep,
}

// Implementing the Display trait for LEDColors
impl fmt::Display for LEDColors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color_str = match self {
            LEDColors::Off => "Off",
            LEDColors::Red => "Red",
            LEDColors::Green => "Green",
            LEDColors::Yellow => "Yellow",
            LEDColors::Blue => "Blue",
            LEDColors::Purple => "Purple",
            LEDColors::LightBlue => "LightBlue",
            LEDColors::White => "White",
            LEDColors::Keep => "Keep",
        };
        write!(f, "{}", color_str)
    }
}

fn get_led_color(color: u8) -> LEDColors {
    match color {
        constants::LED_COLOR_OFF => LEDColors::Off,
        constants::LED_COLOR_RED => LEDColors::Red,
        constants::LED_COLOR_GREEN => LEDColors::Green,
        constants::LED_COLOR_YELLOW => LEDColors::Yellow,
        constants::LED_COLOR_BLUE => LEDColors::Blue,
        constants::LED_COLOR_PURPLE => LEDColors::Purple,
        constants::LED_COLOR_LIGHTBLUE => LEDColors::LightBlue,
        constants::LED_COLOR_WHITE => LEDColors::White,
        constants::LED_COLOR_KEEP => LEDColors::Keep,
        0x8..=0xF => LEDColors::Keep,
        _ => LEDColors::Off,
    }
}

struct Data {
    command_version: u8,
    command_id: u8,
    alarm_control: u8,
    volume: u8,
    led_control: u8,
    reserved_first: u8,
    reserved_second: u8,
    reserved_third: u8,
}

trait States {
    fn blank() -> Self;
    fn default() -> Self;
    fn settings() -> Self;
}

impl States for Data {
    fn default() -> Self {
        Data {
            command_version: constants::COMMAND_VERSION,
            command_id: constants::COMMAND_ID_CONTROL,
            alarm_control: constants::BUZZER_COUNT_KEEP << 4 | constants::BUZZER_KEEP,
            volume: constants::BUZZER_VOLUME_KEEP,
            led_control: constants::LED_COLOR_KEEP << 4 | constants::LED_PATTERN_KEEP,
            reserved_first: constants::BLANK,
            reserved_second: constants::BLANK,
            reserved_third: constants::BLANK,
        }
    }
    fn blank() -> Self {
        Data {
            command_version: constants::COMMAND_VERSION,
            command_id: constants::COMMAND_ID_CONTROL,
            alarm_control: constants::BLANK,
            volume: constants::BLANK,
            led_control: constants::BLANK,
            reserved_first: constants::BLANK,
            reserved_second: constants::BLANK,
            reserved_third: constants::BLANK,
        }
    }
    fn settings() -> Self {
        Data {
            command_version: constants::COMMAND_VERSION,
            command_id: constants::COMMAND_ID_SETTING,
            alarm_control: constants::SETTING_ON, // or constants::SETTING_OFF
            volume: constants::BLANK,
            led_control: constants::BLANK,
            reserved_first: constants::BLANK,
            reserved_second: constants::BLANK,
            reserved_third: constants::BLANK,
        }
    }
}

trait Setters {
    fn set_led_control(&mut self, led_control: u8);
    fn set_alarm_control(&mut self, alarm_control: u8);
    fn set_volume(&mut self, volume: u8);
}
impl Setters for Data {
    fn set_alarm_control(&mut self, alarm_control: u8) {
        self.alarm_control = alarm_control;
    }
    fn set_led_control(&mut self, led_control: u8) {
        self.led_control = led_control;
    }
    fn set_volume(&mut self, volume: u8) {
        self.volume = volume;
    }
}

trait Getters {
    fn get_led_control(&self) -> u8;
    fn get_alarm_control(&self) -> u8;
    fn get_volume(&self) -> u8;
}
impl Getters for Data {
    fn get_alarm_control(&self) -> u8 {
        self.alarm_control
    }
    fn get_led_control(&self) -> u8 {
        self.led_control
    }
    fn get_volume(&self) -> u8 {
        self.volume
    }
}

trait Transform {
    fn to_array(&self) -> [u8; 8];
}
impl Transform for Data {
    fn to_array(&self) -> [u8; 8] {
        [
            self.command_version,
            self.command_id,
            self.alarm_control,
            self.volume,
            self.led_control,
            self.reserved_first,
            self.reserved_second,
            self.reserved_third,
        ]
    }
}

fn main() -> Result<()> {
    let matches: ArgMatches = command!() // requires `cargo` feature
    .version("1.0")
    .about("Patlite NE-SN-USB CLI Tool")
    .propagate_version(true)
    // .subcommand_required(true)
    // .arg_required_else_help(true)
    .subcommand(
      Command::new("lightbuzz")
      .about("Set light and buzzer")
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
        Some(("lightbuzz", sub_matches)) => {
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
        Some(("Off", _)) => {
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

fn setup_device() -> Result<DeviceHandle<rusb::Context>> {
    let mut context: Context = match Context::new() {
        Ok(c) => c,
        Err(_) => {
            println!("Failed to create USB context");
            return Err(rusb::Error::NotFound);
        }
    };
    let (mut device, mut handle) =
        open_device(&mut context, constants::VENDOR_ID, constants::DEVICE_ID)
            .expect("Failed to open USB device");
    // check if device exists and hasn't been claimed
    // if device.is_some() {
    //     let (device, handle) = device.unwrap();
    //     if handle.kernel_driver_active(0)? {

    let endpoints: Vec<Endpoint> = match find_readable_endpoints(&mut device) {
        Ok(endpoints) => endpoints,
        Err(e) => {
            println!("Failed to find readable endpoints: {:?}", e);
            return Err(rusb::Error::NotFound);
        }
    };

    // println!("Endpoints: {:#?}", endpoints);
    let endpoint: &Endpoint = endpoints
        .first()
        .expect("No Configurable endpoint found on device");
    // get endpoint with address 0x01
    // let endpoint = endpoints.iter().find(|e| e.address == constants::ENDPOINT_ADDRESS_GET).expect("No Configurable endpoint found on device");
    // println!("Endpoint: {:#?}", endpoint);
    // claim and configure device
    let _endpoint_config = match configure_endpoint(&mut handle, endpoint) {
        Ok(_) => println!("Endpoint configured successfully"),
        Err(e) => {
            println!("Failed to configure endpoint: {:?}", e);
            return Err(rusb::Error::NotFound);
        }
    };

    Ok(handle)
}

fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<(Device<T>, DeviceHandle<T>)> {
    let devices: rusb::DeviceList<T> = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc: rusb::DeviceDescriptor = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, handle)),
                Err(_) => continue,
            }
        }
    }

    None
}

fn print_device_info<T: UsbContext>(
    handle: &mut DeviceHandle<T>,
    table_builder: &mut Builder,
) -> Result<()> {
    let device_desc = handle.device().device_descriptor()?;
    let timeout = Duration::from_secs(1);
    let languages = handle.read_languages(timeout)?;

    // println!("Active configuration: {}", handle.active_configuration()?);

    if !languages.is_empty() {
        let language: rusb::Language = languages[0];
        table_builder.push_record(["Language", "Manufacturer", "Product", "Serial Number"]);
        table_builder.push_record([
            format!("{:?}", language),
            handle
                .read_manufacturer_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string()),
            handle
                .read_product_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string()),
            handle
                .read_serial_number_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string()),
        ]);
        let table: String = table_builder
            .clone()
            .build()
            .with(Style::rounded())
            .to_string();
        println!("{}", table);
    }
    Ok(())
}

fn find_readable_endpoints<T: UsbContext>(device: &mut Device<T>) -> Result<Vec<Endpoint>> {
    let device_desc = device.device_descriptor()?;
    let mut endpoints = vec![];

    for n in 0..device_desc.num_configurations() {
        // println!("Configuration: {}", n);
        let config_desc = match device.config_descriptor(n) {
            Ok(c) => c,
            Err(_) => continue, // Skip on error
        };

        // println!("Config descriptor: {:#?}", config_desc);
        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                // println!("\nInterface: {:#?}", interface_desc);
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    // println!("\n\nEndpoint descriptor: {:#?}", endpoint_desc);
                    endpoints.push(Endpoint {
                        config: config_desc.number(),
                        iface: interface_desc.interface_number(),
                        setting: interface_desc.setting_number(),
                        address: endpoint_desc.address(),
                    })
                }
            }
        }
    }
    // println!("\nFound {} endpoints", endpoints.len());

    Ok(endpoints)
}

fn configure_endpoint<T: UsbContext>(
    handle: &mut DeviceHandle<T>,
    endpoint: &Endpoint,
) -> Result<()> {
    handle.set_active_configuration(endpoint.config)?;
    handle.claim_interface(endpoint.iface)?;
    handle.set_alternate_setting(endpoint.iface, endpoint.setting)
}

fn send_command<T: UsbContext>(handle: &mut DeviceHandle<T>, data: Data) -> Result<usize> {
    let timeout = Duration::from_millis(constants::SEND_TIMEOUT);
    // Send command
    handle.write_interrupt(constants::ENDPOINT_ADDRESS, &data.to_array(), timeout)
}

fn read_interrupt<T: UsbContext>(handle: &mut DeviceHandle<T>) {
    println!("\tReading interrupt");
    let timeout = Duration::from_millis(constants::SEND_TIMEOUT);
    let mut buf: [u8; 8] = [0u8; 8];
    let res = handle
        .read_interrupt(constants::ENDPOINT_ADDRESS_GET, &mut buf, timeout)
        .map(|_| buf.to_vec());
    print!("Read interrupt: {:?}", res);
}

fn set_master_controls_command(
    handle: &mut DeviceHandle<rusb::Context>,
    color: &u8,
    color_pattern: &u8,
    buzzer_pattern: &u8,
    repetition: &u8,
    volume: &u8,
) -> Result<bool> {
    // Set controls for everything, i.e., alarm, buzzer and led controls
    let buzzer_control: u8 = repetition << 4 | buzzer_pattern;
    let led_control: u8 = color << 4 | color_pattern; // Combine color and pattern into a single byte

    let mut master_controls: Data = Data::default();
    master_controls.set_alarm_control(buzzer_control);
    master_controls.set_led_control(led_control);
    master_controls.set_volume(*volume);

    match send_command(handle, master_controls) {
        Ok(u) => println!("Command sent successfully: {:?}", u),
        Err(e) => {
            println!("Failed to send command: {:?}", e);
            return Ok(false);
        }
    }

    Ok(true)
}

fn set_light_command(
    handle: &mut DeviceHandle<rusb::Context>,
    color: &u8,
    pattern: &u8,
) -> Result<bool> {
    // Ensure color is in the range 0-15 (4 bits) and pattern is in the range 0-15 (4 bits)
    let mut light_data: Data = Data::default();
    let led_control: u8 = (color & 0x0F) << 4 | (pattern & 0x0F); // Combine color and pattern into a single byte
    light_data.set_led_control(led_control);

    match send_command(handle, light_data) {
        Ok(u) => println!("Command sent successfully: {:?}", u),
        Err(e) => {
            println!("Failed to send command: {:?}", e);
            return Ok(false);
        }
    }
    return Ok(true);
}

fn set_buzz_command(
    handle: &mut DeviceHandle<rusb::Context>,
    pattern: &u8,
    repetition: &u8,
    volume: &u8,
) -> Result<bool> {
    // Specify the buzzer pattern, number of times to repeat the buzzer, and the volume
    let mut buzz_data: Data = Data::default();
    let buzz_control: u8 = (repetition & 0x0F) << 4 | (pattern & 0x0F);
    buzz_data.set_alarm_control(buzz_control);
    buzz_data.set_volume(*volume);

    match send_command(handle, buzz_data) {
        Ok(u) => println!("Command sent successfully: {:?}", u),
        Err(e) => {
            println!("Failed to send command: {:?}", e);
            return Ok(false);
        }
    }

    return Ok(true);
}

fn set_blank(handle: &mut DeviceHandle<rusb::Context>) -> Result<bool> {
    // Turn off the light, buzzer, and volume to off
    let blank_data: Data = Data::blank();

    match send_command(handle, blank_data) {
        Ok(u) => println!("Command sent successfully: {:?}", u),
        Err(e) => {
            println!("Failed to send command: {:?}", e);
            return Ok(false);
        }
    }

    return Ok(true);
}

fn get_settings(handle: &mut DeviceHandle<rusb::Context>) -> Result<bool> {
    // Get the current settings of the device
    let get_settings: Data = Data::settings();

    match send_command(handle, get_settings) {
        Ok(u) => println!("Command sent successfully: {:?}", u),
        Err(e) => {
            println!("Failed to send command: {:?}", e);
            return Ok(false);
        }
    }

    return Ok(true);
}

fn set_volume_command(handle: &mut DeviceHandle<rusb::Context>, volume: &u8) -> Result<bool> {
    // Set the volume level of the buzzer
    let mut set_volume: Data = Data::default();
    set_volume.set_volume(*volume);

    match send_command(handle, set_volume) {
        Ok(u) => println!("Command sent successfully: {:?}", u),
        Err(e) => {
            println!("Failed to send command: {:?}", e);
            return Ok(false);
        }
    }

    return Ok(true);
}
