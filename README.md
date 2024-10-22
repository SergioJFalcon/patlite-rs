# WORK IN PROGRESS

    Only supports device model NE-SN-USB currently to turn on the light, have it buzz, change the volume level, and set light strobing patterns

## How to use CLI

    For now: 
        git clone
        cargo build
    
    Example use: Turn on the LED light to red continuously
        .\patlite-rs light 1 1

    For help using the CLI use the helper arg -h or --help

        .\patlite-rs --help

## Documentations

    https://docs.rs/rusb/latest/rusb/struct.Device.html

    https://github.com/a1ien/rusb/blob/master/examples/read_device.rs

    https://github.com/PATLITE-Corporation/NE-USB_windows_python_example/blob/main/NE-USB%20(windows%20python)%2020230817.pdf

    https://gill.net.in/posts/reverse-engineering-a-usb-device-with-rust/
    

## Requirements

  Must have gcc installed and have its path set
  if you're like me and you can't edit your env var

    $env:PATH = "C:\msys64\ucrt64\bin;" + $env:PATH
