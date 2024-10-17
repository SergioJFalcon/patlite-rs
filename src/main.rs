use std::time::Duration;

use rusb::{ Device, DeviceDescriptor, DeviceHandle, Result, Speed, UsbContext, ConfigDescriptor };

pub mod constants;

fn main() {
    println!("Hello, world!");
    let dev: Device<rusb::GlobalContext> = get_patlite_device();
    inspect_device(&dev).unwrap();
    println!("\n\n");
    let descriptor = dev.device_descriptor().unwrap();
    println!("Device descriptor: {:?}", descriptor);
    print!("Our device: {:?}", dev);
    let dev_speed: Speed = dev.speed();
    println!("Device speed: {:?}", dev_speed);
    let opened_device: DeviceHandle<rusb::GlobalContext> = open_device(dev);
    println!("\n@@@@Handle: {:?}", opened_device);
    println!("\nGet active configuration: {:?}", opened_device.active_configuration().unwrap());
    
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
  println!("Send command Device: {:?}", dev);
  println!("Send command Data: {:?}", data);
  // Send command
  let timeout: Duration = Duration::from_millis(constants::SEND_TIMEOUT as u64);
  println!("Timeout: {:?}", timeout);
  // Lets get the device's endpoints
  let endpoints = dev.device().active_config_descriptor();
  println!("Endpoints: {:?}", endpoints);
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
    let buzzer_control = (constants::BUZZER_COUNT_KEEP << 4) | constants::BUZZER_KEEP;

    // LED control
    let led: u8 = (color << 4) | state;
    let format_bytes = b"BBBBBxxx";  // This creates a byte string

    // Now, create the full data array including the format bytes
    let mut data: Vec<u8> = Vec::with_capacity(format_bytes.len() + 8);
    data.extend_from_slice(format_bytes);
    data.extend_from_slice(&[
        constants::COMMAND_VERSION,    // Command version (0x00: fixed)
        constants::COMMAND_ID_CONTROL, // Command ID
        buzzer_control,                // Buzzer control
        constants::BUZZER_VOLUME_KEEP, // Buzzer volume (maintain the status quo)
        led,                           // LED control
    ]);
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
