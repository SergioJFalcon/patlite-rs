use rusb::{Context, Device, DeviceHandle, Result, UsbContext};
use std::{env::Args, time::Duration};
use clap::{arg, ArgMatches, command, Command};

mod constants;

#[derive(Debug)]
struct Endpoint {
  config: u8,
  iface: u8,
  setting: u8,
  address: u8,
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

trait Getters {
  fn get_led_control(&self) -> u8;
  fn get_alarm_control(&self) -> u8;
  fn get_volume(&self) -> u8;
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

fn main() -> Result<()> {
    println!("************* START *************");
    let matches: ArgMatches = command!() // requires `cargo` feature
      .version("1.0")
      .about("Patlite NE-SN-USB CLI Tool")
      .propagate_version(true)
      .subcommand_required(true)
      .arg_required_else_help(true)
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
        Command::new("Off")
        .about("Set the device to default state")
      )
      .subcommand(
        Command::new("info")
        .about("Get information on available colors, led patterns, buzzer patterns, and volume levels")
      )
      .get_matches();
    
    let mut context: Context = Context::new()?;
    let (mut device, mut handle) = open_device(&mut context, constants::VENDOR_ID, constants::DEVICE_ID).expect("Failed to open USB device");
    // check if device exists and hasn't been claimed
    // if device.is_some() {
    //     let (device, handle) = device.unwrap();
    //     if handle.kernel_driver_active(0)? {

    match matches.subcommand() {
        Some(("lightbuzz", sub_matches)) => {
          let color = sub_matches.get_one::<u8>("COLOR").expect("Color is required");
          let color_pattern = sub_matches.get_one::<u8>("COLORPATTERN").expect("Color Pattern is required");
          let buzzer_pattern = sub_matches.get_one::<u8>("BUZZERPATTERN").expect("Buzzer Pattern is required");
          let volume = sub_matches.get_one::<u8>("VOLUME").expect("Volume is required");
          let repetition = sub_matches.get_one::<u8>("REPETITION").expect("Repetition is required");

          println!("LightBuzz: \n\tColor: {}, \n\tColor Pattern: {}, \n\tBuzzer Pattern: {}, \n\tRepetition: {}, \n\tVolume: {}", color, color_pattern, buzzer_pattern, repetition, volume);
          set_master_controls_command(&mut handle, color, color_pattern, buzzer_pattern, repetition, volume)?;        
        },
        Some(("light", sub_matches)) => {
          let color = sub_matches.get_one::<u8>("COLOR").expect("Color is required");
          let pattern = sub_matches.get_one::<u8>("PATTERN").expect("Pattern is required");
          println!("Light: Color: {}, Pattern: {}", color, pattern);

          set_light_command(&mut handle, color, pattern)?;
        },
        Some(("buzz", sub_matches)) => {
          let buzzer_pattern = sub_matches.get_one::<u8>("PATTERN").expect("Buzzer Pattern is required");
          let volume = sub_matches.get_one::<u8>("VOLUME").expect("Volume is required");
          let repetition = sub_matches.get_one::<u8>("REPETITION").expect("Repetition is required");

          set_buzz_command(&mut handle, buzzer_pattern, repetition, volume)?;
        },
        Some(("volume", sub_matches)) => {
            let level = sub_matches.get_one::<u8>("LEVEL").expect("Level is required");
            println!("Volume: Level: {}", level);

            set_volume_command(&mut handle, level)?;
        },
        Some(("state", _)) => {
            println!("State");
            get_settings(&mut handle)?;
        },
        Some(("Off", _)) => {
            println!("Off");
            set_blank(&mut handle)?;
        },
        Some(("info", _)) => {
            println!("Info");
            // TODO: Implement this
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }



    print_device_info(&mut handle)?;
    let endpoints: Vec<Endpoint> = find_readable_endpoints(&mut device)?;
    // // println!("Endpoints: {:#?}", endpoints);
    // let endpoint: &Endpoint = endpoints.first().expect("No Configurable endpoint found on device");
    
    // // get endpoint with address 0x01
    // // let endpoint = endpoints.iter().find(|e| e.address == constants::ENDPOINT_ADDRESS_GET).expect("No Configurable endpoint found on device");
    // println!("Endpoint: {:#?}", endpoint);
    // // claim and configure device
    // configure_endpoint(&mut handle, endpoint)?;
    // // control device here

    // let red_on: [u8; 8] = [ 0x00, 0x00, 0x10, 0x00, 0x11, 0x00, 0x00, 0x00 ]; // Continous red light example
    // println!("\nSending command to turn light to red");
    // match send_command(&mut handle, red_on) {
    //   Ok(u) => println!("Command sent successfully: {:?}", u),
    //   Err(e) => println!("Failed to send command: {:?}", e),
    // }

    // // wait for 3 seconds
    // std::thread::sleep(Duration::from_secs(2));
    // // read_interrupt(&mut handle);
    
    // let purple_on: [u8; 8] = [ 0x00, 0x00, 0x10, 0x00, 0x51, 0x00, 0x00, 0x00 ]; // Continous red light example
    // print_data(purple_on);

    // println!("\nSending command to turn light to purple");
    // match send_command(&mut handle, purple_on) {
    //   Ok(u) => println!("Command sent successfully: {:?}", u),
    //   Err(e) => println!("Failed to send command: {:?}", e),
    // }

    // // wait for 3 seconds
    // std::thread::sleep(Duration::from_secs(2));
    
    // println!("\nSending command to turn off light");
    // let off: [u8; 8] = [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ]; // Off example
    // match send_command(&mut handle, off) {
    //   Ok(u) => println!("Command sent successfully: {:?}", u),
    //   Err(e) => println!("Failed to send command: {:?}", e),
    // }

    // println!("Completed command");

    // println!("\n\n\n******Releasing device******\n");
    // // cleanup after use
    // // handle.release_interface(endpoint.iface)?;
    println!("Completed!~");
    Ok(())
}

fn open_device<T: UsbContext>(context: &mut T, vid: u16, pid: u16) -> Option<(Device<T>, DeviceHandle<T>)> {
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

fn print_device_info<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<()> {
    let device_desc = handle.device().device_descriptor()?;
    let timeout = Duration::from_secs(1);
    let languages = handle.read_languages(timeout)?;

    println!("Active configuration: {}", handle.active_configuration()?);

    if !languages.is_empty() {
        let language = languages[0];
        println!("Language: {:?}", language);

        println!(
            "Manufacturer: {}",
            handle
                .read_manufacturer_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "Product: {}",
            handle
                .read_product_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "Serial Number: {}",
            handle
                .read_serial_number_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
    }
    Ok(())
}

fn find_readable_endpoints<T: UsbContext>(device: &mut Device<T>) -> Result<Vec<Endpoint>> {
  let device_desc = device.device_descriptor()?;
  let mut endpoints = vec![];

  for n in 0..device_desc.num_configurations() {
    println!("Configuration: {}", n);
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
  println!("\nFound {} endpoints", endpoints.len());

  Ok(endpoints)
}

fn configure_endpoint<T: UsbContext>(handle: &mut DeviceHandle<T>, endpoint: &Endpoint) -> Result<()> {
  handle.set_active_configuration(endpoint.config)?;
  handle.claim_interface(endpoint.iface)?;
  handle.set_alternate_setting(endpoint.iface, endpoint.setting)
}

fn print_data(data: [u8; 8]) {
  let byte1: u8 = data[0];
  let byte2: u8 = data[1];
  let byte3: u8 = data[2];
  let byte4: u8 = data[3];
  let byte5: u8 = data[4];
  let byte6: u8 = data[5];
  let byte7: u8 = data[6];
  let byte8: u8 = data[7];
  println!("Complete Bytes: {:?}", data);
  println!("Data: |{} {}|{} {}|{} {}|{} {}|", byte1, byte2, byte3, byte4, byte5, byte6, byte7, byte8);
}

fn send_command<T: UsbContext>(handle: &mut DeviceHandle<T>, data: Data) -> Result<usize> {
  println!("\tSending command");
  let timeout = Duration::from_millis(constants::SEND_TIMEOUT);
  // Send command
  handle.write_interrupt(constants::ENDPOINT_ADDRESS, &data.to_array(), timeout)
}

fn read_interrupt<T: UsbContext>(handle: &mut DeviceHandle<T>) {
  println!("\tReading interrupt");
  let timeout = Duration::from_millis(constants::SEND_TIMEOUT);
  let mut buf: [u8; 8] = [0u8; 8];
  let res = handle.read_interrupt(constants::ENDPOINT_ADDRESS_GET, &mut buf, timeout).map(|_| buf.to_vec());
  print!("Read interrupt: {:?}", res);
}

fn set_master_controls_command(handle: &mut DeviceHandle<rusb::Context>, color: &u8, color_pattern: &u8, buzzer_pattern: &u8, repetition: &u8, volume: &u8) -> Result<bool> {
  // Set controls for everything, i.e., alarm, buzzer and led controls
  let buzzer_control: u8 = repetition << 4 | buzzer_pattern;
  let led_control: u8 = color << 4 | color_pattern; // Combine color and pattern into a single byte

  let mut master_controls: Data = Data::default();
  master_controls.set_alarm_control(buzzer_control);
  master_controls.set_led_control(led_control);
  master_controls.set_volume(*volume);

  println!("Master Controls: {:?}", master_controls.to_array());
  match send_command(handle, master_controls) {
    Ok(u) => println!("Command sent successfully: {:?}", u),
    Err(e) => {
      println!("Failed to send command: {:?}", e);
      return Ok(false);
    },
  }
  
  Ok(true)
}

fn set_light_command(handle: &mut DeviceHandle<rusb::Context>, color: &u8, pattern: &u8) -> Result<bool> {
  // Ensure color is in the range 0-15 (4 bits) and pattern is in the range 0-15 (4 bits)
  let buzzer_control: u8 = constants::BUZZER_COUNT_KEEP << 4 | constants::BUZZER_KEEP;
  let led_control: u8 = (color & 0x0F) << 4 | (pattern & 0x0F); // Combine color and pattern into a single byte
  // let red_on: [u8; 8] = [ 0x00, 0x00, 0x10, 0x00, 0x11, 0x00, 0x00, 0x00 ]; // Continous red light example
  println!("buzzer_control: {}", buzzer_control);
  println!("led_control: {}", led_control);

  // let light_data: [u8; 8] = [
  //   constants::COMMAND_VERSION, // Command version
  //   constants::COMMAND_ID_CONTROL, // Command ID
  //   buzzer_control, // Alarm Control if commandId is 0, if commandId is 1, then its Setting
  //   constants::BUZZER_VOLUME_KEEP, // Buzzer volume
  //   led_control, // LED Control
  //   constants::BLANK, // Reserved
  //   constants::BLANK, // Reserved
  //   constants::BLANK, // Reserved
  // ];
  let mut light_data: Data = Data::default();
  light_data.set_led_control(led_control);

  println!("Light Data: {:?}", light_data.to_array());
  match send_command(handle, light_data) {
    Ok(u) => println!("Command sent successfully: {:?}", u),
    Err(e) => {
      println!("Failed to send command: {:?}", e);
      return Ok(false);
    },
  }
  return Ok(true);
}

fn set_buzz_command(handle: &mut DeviceHandle<rusb::Context>, pattern: &u8, repetition: &u8, volume: &u8) -> Result<bool> {
  // Specify the buzzer pattern, number of times to repeat the buzzer, and the volume
  // let buzz_data: [u8; 8] = [
  //   constants::COMMAND_VERSION, // Command version
  //   constants::COMMAND_ID_CONTROL, // Command ID
  //   constants::LED_COLOR_KEEP, // LED color
  //   constants::LED_PATTERN_KEEP, // LED pattern
  //   repetition, // Number of buzzers
  //   pattern, // Buzzer pattern
  //   volume, // Buzzer volume
  //   constants::BLANK, // Reserved
  // ];
  let mut buzz_data: Data = Data::default();
  buzz_data.set_alarm_control(repetition << 4 | pattern);
  buzz_data.set_volume(*volume);

  match send_command(handle, buzz_data) {
    Ok(u) => println!("Command sent successfully: {:?}", u),
    Err(e) => {
      println!("Failed to send command: {:?}", e);
      return Ok(false);
    },
  }
  return Ok(true);
}

fn set_blank(handle: &mut DeviceHandle<rusb::Context>) -> Result<bool> {
    // Turn off the light, buzzer, and volume to off
    // let off: [u8; 8] = [ constants::BLANK, constants::BLANK, constants::BLANK, constants::BLANK, constants::BLANK, constants::BLANK, constants::BLANK, constants::BLANK ]; // Off example
    let blank_data: Data = Data::blank();

    match send_command(handle, blank_data) {
      Ok(u) => println!("Command sent successfully: {:?}", u),
      Err(e) => {
        println!("Failed to send command: {:?}", e);
        return Ok(false);
      },
    }

    return Ok(true);
}

fn get_settings(handle: &mut DeviceHandle<rusb::Context>) -> Result<bool> {
  // Get the current settings of the device
  // let get_settings: [u8; 8] = [ constants::COMMAND_VERSION, constants::COMMAND_ID_SETTING, constants::BLANK, constants::BLANK, constants::BLANK, constants::BLANK, constants::BLANK, constants::BLANK ]; // Get settings example
  let get_settings: Data = Data::settings();

  match send_command(handle, get_settings) {
    Ok(u) => println!("Command sent successfully: {:?}", u),
    Err(e) => {
      println!("Failed to send command: {:?}", e);
      return Ok(false);
    },
  }

  return Ok(true);
}

fn set_volume_command(handle: &mut DeviceHandle<rusb::Context>, volume: &u8) -> Result<bool> {
  // Set the volume level of the buzzer
  // let set_volume: [u8; 8] = [ constants::COMMAND_VERSION, constants::COMMAND_ID_CONTROL, constants::BLANK, constants::BUZZER_VOLUME_MAX, constants::BLANK, constants::BLANK, constants::BLANK, constants::BLANK ]; // Set volume example
  let mut set_volume: Data = Data::default();
  set_volume.set_volume(*volume);

  match send_command(handle, set_volume) {
    Ok(u) => println!("Command sent successfully: {:?}", u),
    Err(e) => {
      println!("Failed to send command: {:?}", e);
      return Ok(false);
    },
  }

  return Ok(true);
}