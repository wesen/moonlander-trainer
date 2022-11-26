use hidapi::HidApi;

pub fn test_hidapi() {
    println!("Printing all available HID devices?");
    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                println!("{:04x}:{:04x} {} {}", device.vendor_id(), device.product_id(), device.manufacturer_string().unwrap(), device.product_string().unwrap());
            }
            match api.open(0x3297, 0x1969) {
                Ok(dev) => {
                    println!("Found device");
                    // once we open the device we should do the handshake to start receiving raw events
                    // for testing, I did this with the oryx tool in the browser and then started receiving raw HID reports:
                    // [6, 5, 9, 254, 0, 0, 0, 0, 5, 9, 1, 0, 5, 9, 0, 0, 122, 232, 0, 8, 56, 113, 0, 0, 186, 22, 0, 32, 186, 22, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    // [7, 5, 9, 254, 97, 71, 0, 8, 0, 0, 0, 0, 186, 22, 0, 32, 122, 232, 0, 8, 56, 113, 0, 0, 234, 22, 0, 32, 186, 22, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    // [6, 6, 11, 254, 0, 0, 0, 0, 6, 11, 1, 0, 6, 11, 0, 0, 122, 232, 0, 8, 1, 81, 0, 0, 88, 11, 0, 32, 186, 22, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    // [5, 1, 254, 32, 96, 23, 0, 32, 24, 11, 0, 32, 221, 33, 0, 8, 0, 0, 0, 0, 241, 161, 0, 0, 1, 0, 0, 0, 241, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    // [7, 6, 11, 254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 122, 232, 0, 8, 1, 81, 0, 0, 88, 11, 0, 32, 186, 22, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    // [5, 0, 254, 32, 96, 23, 0, 32, 24, 11, 0, 32, 221, 33, 0, 8, 2, 0, 0, 0, 241, 161, 0, 0, 0, 0, 0, 0, 241, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    // [6, 6, 11, 254, 0, 0, 0, 0, 6, 11, 1, 0, 6, 11, 0, 0, 122, 232, 0, 8, 1, 81, 0, 0, 88, 11, 0, 32, 186, 22, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    // [5, 1, 254, 32, 96, 23, 0, 32, 24, 11, 0, 32, 221, 33, 0, 8, 0, 0, 0, 0, 241, 161, 0, 0, 1, 0, 0, 0, 241, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    // [7, 6, 11, 254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 122, 232, 0, 8, 1, 81, 0, 0, 88, 11, 0, 32, 186, 22, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    // [5, 0, 254, 32, 96, 23, 0, 32, 24, 11, 0, 32, 221, 33, 0, 8, 2, 0, 0, 0, 241, 161, 0, 0, 0, 0, 0, 0, 241, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    // [6, 5, 9, 254, 0, 0, 0, 0, 5, 9, 1, 0, 5, 9, 0, 0, 122, 232, 0, 8, 56, 113, 0, 0, 186, 22, 0, 32, 186, 22, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    // [7, 5, 9, 254, 97, 71, 0, 8, 0, 0, 0, 0, 186, 22, 0, 32, 122, 232, 0, 8, 56, 113, 0, 0, 242, 22, 0, 32, 186, 22, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    // [6, 0, 5, 254, 0, 0, 0, 0, 0, 5, 1, 0, 0, 5, 0, 0, 122, 232, 0, 8, 44, 0, 0, 0, 88, 11, 0, 32, 186, 22, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    // [7, 0, 5, 254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 122, 232, 0, 8, 44, 0, 0, 0, 88, 11, 0, 32, 186, 22, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

                    let mut buf = [0u8; 64];
                    while let Ok(_) = dev.read(&mut buf) {
                        println!("{:?}", buf);
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}