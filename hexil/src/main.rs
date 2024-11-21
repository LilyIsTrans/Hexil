use hexil_render;

fn main() {
	let context = hexil_render::VKContext::new(hexil_render::PowerProfile::HighPower);

	println!("{:?}", context);
}
