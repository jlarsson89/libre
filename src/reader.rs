extern crate libusb;
use std::time::Duration;

struct UsbDevice<'a> {
    handle: libusb::DeviceHandle<'a>,
    language: libusb::Language,
    timeout: Duration
}

#[derive(Debug)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8
}

fn read_device(device: &mut libusb::Device, device_desc: &libusb::DeviceDescriptor, handle: &mut libusb::DeviceHandle) -> libusb::Result<()> {
    Ok(())
}

fn find_readable_endpoint(device: &mut libusb::Device, device_desc: &libusb::DeviceDescriptor, transfer_type: libusb::TransferType) -> Option<Endpoint> {
    None
}

fn read_endpoint(handle: &mut libusb::DeviceHandle, endpoint: Endpoint, transfer_type: libusb::TransferType) {
}

fn configure_endpoint<'a>(handle: &'a mut libusb::DeviceHandle, endpoint: &Endpoint) -> libusb::Result<()> {
    Ok(())
}

pub fn read_data() {
	let context = libusb::Context::new().unwrap();
    for mut device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        println!("Bus {:03} Device {:03} ID: {:04x}:{:04x}",
                 device.bus_number(),
                 device.address(),
                 device_desc.vendor_id(),
                 device_desc.product_id())
    }
    println!("reader");
}
