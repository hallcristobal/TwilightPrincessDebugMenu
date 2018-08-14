use libtp::system::mutex::Mutex;
use arrayvec::ArrayVec;
use {controller};
use core::cell::RefCell;

pub struct Command {
	pub buttons: u16,
	pub command: fn(),
}

pub fn process_inputs() {

}

lazy_static! {
	pub static ref COMMANDS: Mutex<ArrayVec<[Command; 20]>> = {
		let vec = ArrayVec::new();
		vec.push(
			Command {

			}
		)
		Mutex(RefCell::new(vec))
	};
}
