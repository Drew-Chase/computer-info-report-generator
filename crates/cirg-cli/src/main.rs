use system_pause::pause;
use cirg::{computer_info::ComputerInfo, power_info::PowerInfo, security_info::SecurityInfo, ComputerInfoExt};

#[tokio::main]
async fn main() {
	let (computer, power, security) = tokio::join!(
		tokio::task::spawn_blocking(ComputerInfo::fetch),
		tokio::task::spawn_blocking(PowerInfo::fetch),
		tokio::task::spawn_blocking(SecurityInfo::fetch),
	);

	// Unwrap the JoinHandle results and propagate errors if needed
	let computer = computer.expect("Computer info task panicked").unwrap();
	let power = power.expect("Power info task panicked").unwrap();
	let security = security.expect("Security info task panicked").unwrap();
	println!("Computer: {}", serde_json::to_string_pretty(&computer).unwrap());
	println!("Power: {}", serde_json::to_string_pretty(&power).unwrap());
	println!("Security: {}", serde_json::to_string_pretty(&security).unwrap());

	pause!();
}
