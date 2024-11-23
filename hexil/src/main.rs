use hexil_render;

use hexil_window;
use structured_logger::{json::new_writer, unix_ms, Builder};

fn main() -> anyhow::Result<()> {
	let mut logger =
		Builder::with_level("debug").with_target_writer("*", new_writer(std::io::stderr()));

	let context = hexil_render::VKContext::new(hexil_render::PowerProfile::HighPower);

	let mut queue = hexil_window::wayland_context()?;

	let mut app = hexil_window::WindowManager {};

	queue.roundtrip(&mut app)?;

	Ok(())
}
