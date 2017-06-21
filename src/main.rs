extern crate serial;
extern crate portmidi as pm;

use std::env;
use std::io;
use std::time::Duration;

use std::thread;
use std::time;
use std::io::prelude::*;
use serial::prelude::*;


fn main() {
	let ten_milis = time::Duration::from_millis(1000);
	let context = pm::PortMidi::new().unwrap();
	let midi_timeout = Duration::from_millis(10);
	let info = context.device(0).unwrap();
	println!("Listening on: {}) {}", info.id(), info.name());
	let in_port = context.input_port(info, 1024).unwrap();

	for arg in env::args_os().skip(1) {
        let mut port = serial::open(&arg).unwrap();
		thread::sleep(ten_milis);
		loop {
			while let Ok(_) = in_port.poll() {
				if let Ok(Some(event)) = in_port.read_n(1024) {
					interact(&mut port, event[0].message.data1, event[0].message.data2).unwrap();
				}
				thread::sleep(midi_timeout);
		  }
		}
  }

	
}

fn interact<T: SerialPort>(port: &mut T, num: u8, force: u8) -> io::Result<()> {
    try!(port.reconfigure(&|settings| {
        try!(settings.set_baud_rate(serial::Baud9600));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }));	

    let mut color: String;
    let chord = num % 12;
    match chord {
      1 ... 4 => {
        color = String::from("#FF0000");
      }

      5 ... 8 => {
        color = String::from("#00FF00");
      }

      _ => {
        color = String::from("#0000FF");
      } 
    }

    println!("color: {}", color);

    try!(port.set_timeout(Duration::from_millis(3000)));
	thread::sleep(Duration::from_millis(350));

	//let color = "#FFFFFF");
	let color_vec = color.into_bytes();

    try!(port.write(&color_vec[..]));

    Ok(())
}