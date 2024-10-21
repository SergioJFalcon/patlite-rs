// Vendor ID
pub const VENDOR_ID: u16 = 0x191A;
// Device ID
pub const DEVICE_ID: u16 = 0x6001;
// Command version
pub const COMMAND_VERSION: u8 = 0x0;

// Command ID
pub const COMMAND_ID_CONTROL: u8 = 0x0;
pub const COMMAND_ID_SETTING: u8 = 0x1;
pub const COMMAND_ID_GETSTATE: u8 = 0x80;

// Endpoint address for sending to host -> USB controlled multicolor indicator
pub const ENDPOINT_ADDRESS: u8 = 0x01;
// Endpoint address for sending to USB -> host controlled multicolor indicator
pub const ENDPOINT_ADDRESS_GET: u8 = 0x81;
// Time-out time when sending a commandw
pub const SEND_TIMEOUT: u64 = 3000;

// LED colors
pub const LED_COLOR_OFF: u8 = 0; // Off
pub const LED_COLOR_RED: u8 = 1; // Red
pub const LED_COLOR_GREEN: u8 = 2; // green
pub const LED_COLOR_YELLOW: u8 = 3; // yellow
pub const LED_COLOR_BLUE: u8 = 4; // Blue
pub const LED_COLOR_PURPLE: u8 = 5; // purple
pub const LED_COLOR_LIGHTBLUE: u8 = 6; // Sky blue
pub const LED_COLOR_WHITE: u8 = 7; // White
pub const LED_COLOR_KEEP: u8 = 0xF; // Keep the current settings

// LED pattern
pub const LED_OFF: u8 = 0x0; // Off
pub const LED_ON: u8 = 0x1; // On
pub const LED_PATTERN1: u8 = 0x2; // LED pattern1
pub const LED_PATTERN2: u8 = 0x3; // LED pattern2
pub const LED_PATTERN3: u8 = 0x4; // LED pattern3
pub const LED_PATTERN4: u8 = 0x5; // LED pattern4
pub const LED_PATTERN5: u8 = 0x6; // LED pattern5
pub const LED_PATTERN6: u8 = 0x7; // LED pattern6
pub const LED_PATTERN_KEEP: u8 = 0xF; // Keep the current settings

// Number of buzzers
pub const BUZZER_COUNT_CONTINUE: u8 = 0x0; // Continuous operation
pub const BUZZER_COUNT_KEEP: u8 = 0xF; // Keep the current settings

// Buzzer pattern
pub const BUZZER_OFF: u8 = 0x0; // Stop
pub const BUZZER_ON: u8 = 0x1; // Blow (continuous)
pub const BUZZER_SWEEP: u8 = 0x2; // Sweep sound
pub const BUZZER_INTERMITTENT: u8 = 0x3; // Intermittent sound
pub const BUZZER_WEEK_ATTENTION: u8 = 0x4; // Weak caution sound
pub const BUZZER_STRONG_ATTENTION: u8 = 0x5; // Strong attention sound
pub const BUZZER_SHINING_STAR: u8 = 0x6; // shining star
pub const BUZZER_LONDON_BRIDGE: u8 = 0x7; // London bridge
pub const BUZZER_KEEP: u8 = 0xF; // Keep the current settings

// Buzzer volume
pub const BUZZER_VOLUME_OFF: u8 = 0x0; // Mute
pub const BUZZER_VOLUME_MAX: u8 = 0xA; // Maximum volume
pub const BUZZER_VOLUME_KEEP: u8 = 0xF; // Keep the current settings

// Setting
pub const SETTING_OFF: u8 = 0x0; // OFF
pub const SETTING_ON: u8 = 0x1; // ON

// others
pub const BLANK: u8 = 0x0; // openings

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

			// Red Light on
			// | 0x00 | 0x00 | 0x00 | 0x00 | 0x11 | 0x00 | 0x00 | 0x00 |