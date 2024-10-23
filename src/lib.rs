mod constants;


use constants::*;
use rusb::{Context, Device, DeviceHandle, Result, UsbContext};
use std::time::Duration;
use std::fmt;
use tabled::{builder::Builder, settings::Style};

#[derive(Debug)]
pub struct Endpoint {
    pub config: u8,
    pub iface: u8,
    pub setting: u8,
    pub address: u8,
}

#[derive(Debug)]
pub enum LEDColors {
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
        LED_COLOR_OFF => LEDColors::Off,
        LED_COLOR_RED => LEDColors::Red,
        LED_COLOR_GREEN => LEDColors::Green,
        LED_COLOR_YELLOW => LEDColors::Yellow,
        LED_COLOR_BLUE => LEDColors::Blue,
        LED_COLOR_PURPLE => LEDColors::Purple,
        LED_COLOR_LIGHTBLUE => LEDColors::LightBlue,
        LED_COLOR_WHITE => LEDColors::White,
        LED_COLOR_KEEP => LEDColors::Keep,
        0x8..=0xF => LEDColors::Keep,
        _ => LEDColors::Off,
    }
}

pub struct Data {
  pub command_version: u8,
  pub command_id: u8,
  pub alarm_control: u8,
  pub volume: u8,
  pub led_control: u8,
  pub reserved_first: u8,
  pub reserved_second: u8,
  pub reserved_third: u8,
}

impl Data {
    pub fn default() -> Self {
        Data {
            command_version: COMMAND_VERSION,
            command_id: COMMAND_ID_CONTROL,
            alarm_control: BUZZER_COUNT_KEEP << 4 | BUZZER_KEEP,
            volume: BUZZER_VOLUME_KEEP,
            led_control: LED_COLOR_KEEP << 4 | LED_PATTERN_KEEP,
            reserved_first: BLANK,
            reserved_second: BLANK,
            reserved_third: BLANK,
        }
    }
    pub fn blank() -> Self {
        Data {
            command_version: COMMAND_VERSION,
            command_id: COMMAND_ID_CONTROL,
            alarm_control: BLANK,
            volume: BLANK,
            led_control: BLANK,
            reserved_first: BLANK,
            reserved_second: BLANK,
            reserved_third: BLANK,
        }
    }
    pub fn settings() -> Self {
        Data {
            command_version: COMMAND_VERSION,
            command_id: COMMAND_ID_SETTING,
            alarm_control: SETTING_ON, // or SETTING_OFF
            volume: BLANK,
            led_control: BLANK,
            reserved_first: BLANK,
            reserved_second: BLANK,
            reserved_third: BLANK,
        }
    }
    pub fn new() -> Self {
        Data::default()
    }
    pub fn from_array(data: [u8; 8]) -> Self {
        Data {
            command_version: data[0],
            command_id: data[1],
            alarm_control: data[2],
            volume: data[3],
            led_control: data[4],
            reserved_first: data[5],
            reserved_second: data[6],
            reserved_third: data[7],
        }
    }
    pub fn from_settings() -> Self {
        Data::settings()
    }
    pub fn from_blank() -> Self {
        Data::blank()
    }
    pub fn from_default() -> Self {
        Data::default()
    }
    // pub fn from_led_control(led_control: u8) -> Self {
    // 		let mut data = Data::default();
    // 		data.set_led_control(led_control);
    // 		data
    // }
    // pub fn from_alarm_control(alarm_control: u8) -> Self {
    // 		let mut data = Data::default();
    // 		data.set_alarm_control(alarm_control);
    // 		data
    // }
    // pub fn from_volume(volume: u8) -> Self {
    // 		let mut data = Data::default();
    // 		data.set_volume(volume);
    // 		data
    // }
    pub fn set_led_control(&mut self, led_control: u8) {
        self.led_control = led_control;
    }
    pub fn set_alarm_control(&mut self, alarm_control: u8) {
        self.alarm_control = alarm_control;
    }
    pub fn set_volume(&mut self, volume: u8) {
        self.volume = volume;
    }
    pub fn to_array(&self) -> [u8; 8] {
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
    // pub fn get_led_color(&self) -> LEDColors {
    //     get_led_color(self.led_control >> 4)
    // }
    // pub fn get_led_pattern(&self) -> u8 {
    //     self.led_control & 0x0F
    // }
    // pub fn get_alarm_pattern(&self) -> u8 {
    //     self.alarm_control & 0x0F
    // }
    // pub fn get_alarm_count(&self) -> u8 {
    //     self.alarm_control >> 4
    // }
    // pub fn get_alarm_volume(&self) -> u8 {
    //     self.volume & 0x0F
    // }
    // pub fn get_alarm_volume_setting(&self) -> u8 {
    //     self.volume >> 4
    // }
    // pub fn get_command_id(&self) -> u8 {
    //     self.command_id
    // }
    // pub fn get_command_version(&self) -> u8 {
    //     self.command_version
    // }
}

// trait States {
//     fn blank() -> Self;
//     fn default() -> Self;
//     fn settings() -> Self;
// }

// impl States for Data {
//     fn default() -> Self {
//         Data {
//             command_version: COMMAND_VERSION,
//             command_id: COMMAND_ID_CONTROL,
//             alarm_control: BUZZER_COUNT_KEEP << 4 | BUZZER_KEEP,
//             volume: BUZZER_VOLUME_KEEP,
//             led_control: LED_COLOR_KEEP << 4 | LED_PATTERN_KEEP,
//             reserved_first: BLANK,
//             reserved_second: BLANK,
//             reserved_third: BLANK,
//         }
//     }
//     fn blank() -> Self {
//         Data {
//             command_version: COMMAND_VERSION,
//             command_id: COMMAND_ID_CONTROL,
//             alarm_control: BLANK,
//             volume: BLANK,
//             led_control: BLANK,
//             reserved_first: BLANK,
//             reserved_second: BLANK,
//             reserved_third: BLANK,
//         }
//     }
//     fn settings() -> Self {
//         Data {
//             command_version: COMMAND_VERSION,
//             command_id: COMMAND_ID_SETTING,
//             alarm_control: SETTING_ON, // or SETTING_OFF
//             volume: BLANK,
//             led_control: BLANK,
//             reserved_first: BLANK,
//             reserved_second: BLANK,
//             reserved_third: BLANK,
//         }
//     }
// }

// trait Setters {
//     fn set_led_control(&mut self, led_control: u8);
//     fn set_alarm_control(&mut self, alarm_control: u8);
//     fn set_volume(&mut self, volume: u8);
// }
// impl Setters for Data {
//     fn set_alarm_control(&mut self, alarm_control: u8) {
//         self.alarm_control = alarm_control;
//     }
//     fn set_led_control(&mut self, led_control: u8) {
//         self.led_control = led_control;
//     }
//     fn set_volume(&mut self, volume: u8) {
//         self.volume = volume;
//     }
// }

// trait Getters {
//     fn get_led_control(&self) -> u8;
//     fn get_alarm_control(&self) -> u8;
//     fn get_volume(&self) -> u8;
// }
// impl Getters for Data {
//     fn get_alarm_control(&self) -> u8 {
//         self.alarm_control
//     }
//     fn get_led_control(&self) -> u8 {
//         self.led_control
//     }
//     fn get_volume(&self) -> u8 {
//         self.volume
//     }
// }

// trait Transform {
//     fn to_array(&self) -> [u8; 8];
// }
// impl Transform for Data {
//     fn to_array(&self) -> [u8; 8] {
//         [
//             self.command_version,
//             self.command_id,
//             self.alarm_control,
//             self.volume,
//             self.led_control,
//             self.reserved_first,
//             self.reserved_second,
//             self.reserved_third,
//         ]
//     }
// }


fn setup_device() -> Result<DeviceHandle<rusb::Context>> {
	let mut context: Context = match Context::new() {
			Ok(c) => c,
			Err(_) => {
					println!("Failed to create USB context");
					return Err(rusb::Error::NotFound);
			}
	};
	let (mut device, mut handle) =
			open_device(&mut context, VENDOR_ID, DEVICE_ID)
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
	// let endpoint = endpoints.iter().find(|e| e.address == ENDPOINT_ADDRESS_GET).expect("No Configurable endpoint found on device");
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
	let device_desc: rusb::DeviceDescriptor = device.device_descriptor()?;
	let mut endpoints: Vec<Endpoint> = vec![];

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
	let timeout = Duration::from_millis(SEND_TIMEOUT);
	// Send command
	handle.write_interrupt(ENDPOINT_ADDRESS, &data.to_array(), timeout)
}

fn read_interrupt<T: UsbContext>(handle: &mut DeviceHandle<T>) {
	println!("\tReading interrupt");
	let timeout = Duration::from_millis(SEND_TIMEOUT);
	let mut buf: [u8; 8] = [0u8; 8];
	let res = handle
			.read_interrupt(ENDPOINT_ADDRESS_GET, &mut buf, timeout)
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
