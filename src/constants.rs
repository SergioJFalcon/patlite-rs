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
// Off
pub const LED_COLOR_OFF: u8 = 0;
// Red
pub const LED_COLOR_RED: u8 = 1;
// green
pub const LED_COLOR_GREEN: u8 = 2;
// yellow
pub const LED_COLOR_YELLOW: u8 = 3;
// Blue
pub const LED_COLOR_BLUE: u8 = 4;
// purple
pub const LED_COLOR_PURPLE: u8 = 5;
// Sky blue
pub const LED_COLOR_LIGHTBLUE: u8 = 6;
// White
pub const LED_COLOR_WHITE: u8 = 7;
// Keep the current settings
pub const LED_COLOR_KEEP: u8 = 0xF;

// LED pattern
// Off
pub const LED_OFF: u8 = 0x0;
// Lit
pub const LED_ON: u8 = 0x1;
// LED pattern1
pub const LED_PATTERN1: u8 = 0x2;
// LED pattern2
pub const LED_PATTERN2: u8 = 0x3;
// LED pattern3
pub const LED_PATTERN3: u8 = 0x4;
// LED pattern4
pub const LED_PATTERN4: u8 = 0x5;
// LED pattern5
pub const LED_PATTERN5: u8 = 0x6;
// LED pattern6
pub const LED_PATTERN6: u8 = 0x7;
// Keep the current settings
pub const LED_PATTERN_KEEP: u8 = 0xF;

// Number of buzzers
// Continuous operation
pub const BUZZER_COUNT_CONTINUE: u8 = 0x0;
// Keep the current settings
pub const BUZZER_COUNT_KEEP: u8 = 0xF;

// Buzzer pattern
// Stop
pub const BUZZER_OFF: u8 = 0x0;
// Blow (continuous)
pub const BUZZER_ON: u8 = 0x1;
// Sweep sound
pub const BUZZER_SWEEP: u8 = 0x2;
// Intermittent sound
pub const BUZZER_INTERMITTENT: u8 = 0x3;
// Weak caution sound
pub const BUZZER_WEEK_ATTENTION: u8 = 0x4;
// Strong attention sound
pub const BUZZER_STRONG_ATTENTION: u8 = 0x5;
// shining star
pub const BUZZER_SHINING_STAR: u8 = 0x6;
// London bridge
pub const BUZZER_LONDON_BRIDGE: u8 = 0x7;
// Keep the current settings
pub const BUZZER_KEEP: u8 = 0xF;

// Buzzer volume
// Mute
pub const BUZZER_VOLUME_OFF: u8 = 0x0;
// Maximum volume
pub const BUZZER_VOLUME_MAX: u8 = 0xA;
// Keep the current settings
pub const BUZZER_VOLUME_KEEP: u8 = 0xF;

// Setting
// OFF
pub const SETTING_OFF: u8 = 0x0;
// ON
pub const SETTING_ON: u8 = 0x1;

// others
// openings
pub const BLANK: u8 = 0x0;
