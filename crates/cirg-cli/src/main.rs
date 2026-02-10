#[tokio::main]
async fn main() {
	let info = cirg::computer_info::ComputerInfo::fetch().await.unwrap();
	println!("{:#?}", info);
}
