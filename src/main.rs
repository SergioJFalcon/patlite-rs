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
        .arg(arg!([COLOR]))
        .arg(arg!([COLORPATTERN]))
        .arg(arg!([BUZZERPATTERN]))
        .arg(arg!([VOLUME]))
        .arg(arg!([REPETITION])),
      )
      .subcommand(
    Command::new("light")
        .about("Light control")
        .arg(arg!([COLOR]))
        .arg(arg!([PATTERN])),
      )
      .subcommand(
        Command::new("buzz")
        .about("Create a buzzer")
        .arg(arg!([NAME])),
      )
      .subcommand(
        Command::new("volume")
        .about("Set the volume level")
        .arg(arg!([LEVEL])),
      )
      .subcommand(
        Command::new("state")
        .about("Get the current state of the device")
      )
      .subcommand(
        Command::new("Reset")
        .about("Set the device to default state")
      )
      .get_matches();
    
    let mut context: Context = Context::new()?;
    let (mut device, mut handle) = open_device(&mut context, constants::VENDOR_ID, constants::DEVICE_ID).expect("Failed to open USB device");
    // check if device exists and hasn't been claimed
    // if device.is_some() {
    //     let (device, handle) = device.unwrap();
    //     if handle.kernel_driver_active(0)? {

    match matches.subcommand() {
        Some(("add", sub_matches)) => println!(
            "'myapp add' was used, name is: {:?}",
            sub_matches.get_one::<String>("NAME")
        ),
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

fn send_command<T: UsbContext>(handle: &mut DeviceHandle<T>, data: [u8; 8]) -> Result<usize> {
  println!("\tSending command");
  let timeout = Duration::from_millis(constants::SEND_TIMEOUT);
  // Send command
  handle.write_interrupt(constants::ENDPOINT_ADDRESS, &data, timeout)
}

fn read_interrupt<T: UsbContext>(handle: &mut DeviceHandle<T>) {
  println!("\tReading interrupt");
  let timeout = Duration::from_millis(constants::SEND_TIMEOUT);
  let mut buf: [u8; 8] = [0u8; 8];
  let res = handle.read_interrupt(constants::ENDPOINT_ADDRESS_GET, &mut buf, timeout).map(|_| buf.to_vec());
  print!("Read interrupt: {:?}", res);
}

fn set_light(handle: &mut DeviceHandle<rusb::Context>, color: u32, pattern: u32) -> Result<bool> {
  // Specify the LED color and LED pattern
  // Paramters:
  //  Colors: 
  //    0. 0x00 - Off
  //    1. 0x01 - Red
  //    2. 0x02 - Green
  //    3. 0x03 - Yellow
  //    4. 0x04 - Blue
  //    5. 0x05 - l
  //    6. 0x06 - Sky blue
  //    7. 0x07 - White
  //    8. 0x08 ~ 0x0F - Keep the current settings
  // Pattern: 
  //    1. 0x00 - Off
  //    2. 0x01 - On
  //    3. 0x02 - LED pattern1
  //    4. 0x03 - LED pattern2
  //    5. 0x04 - LED pattern3
  //    6. 0x05 - LED pattern4
  //    7. 0x06 - LED pattern5
  //    8. 0x07 - LED pattern6
  //    9. 0x08 ~ 0x0F - Keep the current settings

  return Ok(true);
}