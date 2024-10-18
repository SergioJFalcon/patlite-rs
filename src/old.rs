use std::time::Duration;

use rusb::{ Device, DeviceDescriptor, DeviceHandle, Result, Speed, UsbContext, ConfigDescriptor };

pub mod constants;

fn main() {
    let dev: Device<rusb::GlobalContext> = get_patlite_device();
    inspect_device(&dev).unwrap();
    println!("\n\n");
    // let descriptor = dev.device_descriptor().unwrap();
    // println!("Device descriptor: {:?}", descriptor);
    // print!("Our device: {:?}", dev);
    // let dev_speed: Speed = dev.speed();
    // println!("Device speed: {:?}", dev_speed);
    let opened_device: DeviceHandle<rusb::GlobalContext> = open_device(dev);
    println!("\n@@@@Handle: {:?}", opened_device);
    // println!("\nGet active configuration: {:?}", opened_device.active_configuration().unwrap());
    
    // let send_command = send_command(handle, constants::COMMAND_ID_CONTROL, constants::LED_COLOR_RED);
    // print!("Send command: {:?}", send_command);
    let set_light = set_light(&opened_device, constants::LED_COLOR_RED, constants::LED_ON);
    match set_light {
        Ok(true) => println!("Set light success"),
        Ok(false) => println!("Set light failed"),
        Err(e) => println!("Set light error: {:?}", e),
    }
}

fn get_patlite_device() -> Device<rusb::GlobalContext> {
    for device in rusb::devices().unwrap().iter() {
      let device_desc: DeviceDescriptor = device.device_descriptor().unwrap();
      if device_desc.vendor_id() == constants::VENDOR_ID && device_desc.product_id() == constants::DEVICE_ID {
          println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id());
          return device;
      }
    }
    panic!("PatLite not found!");
}

fn open_device(dev: Device<rusb::GlobalContext>) -> DeviceHandle<rusb::GlobalContext> {
    match dev.open() {
        Ok(opened_dev) => opened_dev,
        Err(e) => panic!("Device found but failed to open: {}", e),
    }
}

fn send_command(dev: &DeviceHandle<rusb::GlobalContext>, data: &[u8]) -> Result<bool> {
  println!("\n\n\nSend command Device: {:?}", dev);
  println!("Send command Data: {:?}", data);
  // Send command
  let timeout: Duration = Duration::from_millis(constants::SEND_TIMEOUT as u64);
  println!("Timeout: {:?}", timeout);
  // Lets get the device's endpoints
  let endpoints = dev.device().active_config_descriptor();
  println!("Endpoints: {:?}", endpoints);
  dev.write_control(request_type, request, value, index, buf, timeout)
  let write_length: usize = dev.write_bulk(constants::ENDPOINT_ADDRESS, data, timeout)?;
  println!("Orig Write length: {:?}", write_length);
  
  #[cfg(target_os = "windows")]
  let write_length: usize = write_length.saturating_sub(1);
  println!("Windows targeting Write length: {:?}", write_length);
  if write_length != data.len() {
    println!("Failed to send");
    Ok(false)
  } else {
    Ok(true)
  }
}

fn set_light(dev: &DeviceHandle<rusb::GlobalContext>, color: u8, state: u8) -> Result<bool> {
    // Argument range check
    if color > 0xF || state > 0xF {
        return Ok(false);
    }
    println!("Setting light to color: {:?} and state: {:?}", color, state);

    // Buzzer control (maintain the status quo)
    let buzzer_control: u8 = (constants::BUZZER_COUNT_KEEP << 4) | constants::BUZZER_KEEP;

    // LED control
    let led: u8 = (color << 4) | state;

    // USB Communication Protocol
      // 1st byte: Command version
        // 0x00: fixed
      // 2nd byte: Command ID
        // 0x00: Control command
        // 0x01: Command to switch connection display setting
      // 3rd byte: Alarm Control
        // 7th-4th bits: Continuous operation / Number of operations
          // 0x0: Continuous operation
          // 0x1 ~ 0xE: Number of operations
            // 1 to 14 times
        // 3rd-0th bits: Alarm Pattern
          // 0x0: Off
          // 0x1: Continuous
          // 0x2: Sweep
          // 0x3: Call Sign
          // 0x4: Low Urgency
          // 0x5: High Urgency
          // 0x6: Twinkle Star (Melody)
          // 0x7: London Bridge (Melody)
          // 0x8~0xF: Maintain current status
      // 4th byte: Alarm volume
          // 7th-4th bits: Open
            // 0x0: Fixed
          // 3rd-0th bits: Volume
            // 0x0: Silent
            // 0x1 ~ 0x9: Stepped volume from maximum to silent
            // 0xA: Maximum volume
            // 0xB ~ 0xF: Maintain current status
          // if command ID is 0x1, 0x00: fixed
      // 5th byte: LED control
        // 7th-4th bits: Color
          // 0x0: Off
          // 0x1: Red
          // 0x2: Green
          // 0x3: Yellow
          // 0x4: Blue
          // 0x5: Purple
          // 0x6: Sky Blue
          // 0x7: White
          // 0x8 ~ 0xF: Maintain current status
        // 3rd-0th bits: Pattern
          // 0x0: Off
          // 0x1: On
          // 0x2: Pattern 1
          // 0x3: Pattern 2
          // 0x4: Pattern 3
          // 0x5: Pattern 4
          // 0x6: Pattern 5
          // 0x7: Pattern 6
          // 0x8 ~ 0xF: Maintain current status
        // if the command ID is 0x01, 0x00: Fixed
      // 6th, 7th, and 8th byte: Open
        // 0x00: Fixed
    
    // Protocol Example
      // Red light on, continuous sound
      // | 0x00 | 0x00 | 0x01 | 0x06 | 0x11 | 0x00 | 0x00 | 0x00 |

      // Purple Light on, turn alarm off
      // | 0x00 | 0x00 | 0x00 | 0x00 | 0x51 | 0x00 | 0x00 | 0x00 |

    // Now, create the full data array including the format bytes
    let mut data: Vec<u8> = Vec::new();
    data.extend_from_slice(&[
        constants::COMMAND_VERSION,    // 1st byte: Command version (0x00: fixed)
        constants::COMMAND_ID_CONTROL, // 2nd byte: Command ID
        0x00,                // 3rd byte: Buzzer control
        0x00, // Buzzer volume (maintain the status quo)
        0x51,                           // LED control
        0x00,
        0x00,
        0x00,
    ]);

    println!("\nData: {:?}\n", data);

    println!("Data: {:?}", data);
    // Send command
    match send_command(dev, &data) {
        Ok(true) => Ok(true),
        Ok(false) => {
            println!("Failed to send data");
            Ok(false)
        },
        Err(e) => Err(e),
    }
}

fn inspect_device<T: UsbContext>(device: &Device<T>) -> rusb::Result<()> {
    let device_desc = device.device_descriptor()?;
    println!("Device Descriptor:");
    println!("  bDeviceClass       : {:#04x}", device_desc.class_code());
    println!("  bDeviceSubClass    : {:#04x}", device_desc.sub_class_code());
    println!("  bDeviceProtocol    : {:#04x}", device_desc.protocol_code());
    println!("  bMaxPacketSize0    : {:#04x}", device_desc.max_packet_size());
    println!("  idVendor           : {:04X}", device_desc.vendor_id());
    println!("  idProduct          : {:04X}", device_desc.product_id());

    let config_desc = device.config_descriptor(0)?;
    println!("\nConfig Descriptor:");
    println!("  bNumInterfaces     : {:02X}", config_desc.num_interfaces());
    
    for interface in config_desc.interfaces() {
        for interface_desc in interface.descriptors() {
            println!("\n  Interface Descriptor:");
            println!("    bInterfaceNumber   : {:02X}", interface_desc.interface_number());
            println!("    bAlternateSetting  : {:02X}", interface_desc.setting_number());
            println!("    bNumEndpoints      : {:02X}", interface_desc.num_endpoints());
            println!("    bInterfaceClass    : {:02X}", interface_desc.class_code());
            println!("    bInterfaceSubClass : {:02X}", interface_desc.sub_class_code());
            println!("    bInterfaceProtocol : {:02X}", interface_desc.protocol_code());
            
            for endpoint_desc in interface_desc.endpoint_descriptors() {
                println!("\n    Endpoint Descriptor:");
                println!("      bDescriptorType    : {:02x}", endpoint_desc.descriptor_type());
                println!("      bEndpointAddress   : {:02X}", endpoint_desc.address());
                println!("      number             : {:02X}", endpoint_desc.number());
                println!("      wMaxPacketSize     : {:04X}", endpoint_desc.max_packet_size());
                println!("      bInterval          : {:02X}", endpoint_desc.interval());
                println!("      transfer_type      : {:?}", endpoint_desc.transfer_type());
                println!("      bLength            : {:?}", endpoint_desc.sync_type());
            }
        }
    }

    Ok(())
}

fn send_test_data<T: UsbContext>(handle: &DeviceHandle<T>, endpoint: u8, data: &[u8]) -> rusb::Result<usize> {
    handle.write_bulk(endpoint, data, Duration::from_secs(1))
}

fn read_response<T: UsbContext>(handle: &DeviceHandle<T>, endpoint: u8) -> rusb::Result<Vec<u8>> {
    let mut buf = vec![0u8; 64];  // Adjust buffer size as needed
    let timeout = Duration::from_secs(1);
    let len = handle.read_bulk(endpoint, &mut buf, timeout)?;
    buf.truncate(len);
    Ok(buf)
}
