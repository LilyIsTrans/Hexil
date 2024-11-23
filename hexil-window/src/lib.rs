use smithay_client_toolkit;
use smithay_client_toolkit::reexports::client;
use smithay_client_toolkit::reexports::client::Proxy;
use smithay_client_toolkit::reexports::protocols;
use smithay_client_toolkit::shell::WaylandSurface;

#[derive(serde::Serialize)]
pub struct WindowManager {}

pub struct UserData;

impl
	client::Dispatch<client::protocol::wl_registry::WlRegistry, client::globals::GlobalListContents>
	for WindowManager
{
	fn event(
		state: &mut Self,
		proxy: &client::protocol::wl_registry::WlRegistry,
		event: <client::protocol::wl_registry::WlRegistry as client::Proxy>::Event,
		data: &client::globals::GlobalListContents,
		conn: &client::Connection,
		qhandle: &client::QueueHandle<Self>,
	) {
		log::debug!(state:serde, proxy:?, event:?, data:?, conn:?, qhandle:?; "A global event happened!");
	}
}

impl
	client::Dispatch<
		protocols::xdg::shell::client::xdg_wm_base::XdgWmBase,
		smithay_client_toolkit::globals::GlobalData,
	> for WindowManager
{
	fn event(
		state: &mut Self,
		proxy: &protocols::xdg::shell::client::xdg_wm_base::XdgWmBase,
		event: protocols::xdg::shell::client::xdg_wm_base::Event,
		data: &smithay_client_toolkit::globals::GlobalData,
		conn: &client::Connection,
		qhandle: &client::QueueHandle<Self>,
	) {
		log::debug!(state:serde, proxy:?, event:?, data:?, conn:?, qhandle:?; "An event happened!");
	}
}

impl
	    client::Dispatch<
		    protocols::xdg::decoration::zv1::client::zxdg_decoration_manager_v1::ZxdgDecorationManagerV1,
		    smithay_client_toolkit::globals::GlobalData,
	    > for WindowManager
{
	    fn event(
		    state: &mut Self,
		    proxy: &protocols::xdg::decoration::zv1::client::zxdg_decoration_manager_v1::ZxdgDecorationManagerV1,
		    event: protocols::xdg::decoration::zv1::client::zxdg_decoration_manager_v1::Event,
		    data: &smithay_client_toolkit::globals::GlobalData,
		    conn: &client::Connection,
		    qhandle: &client::QueueHandle<Self>,
	    ) {
		    log::debug!(state:serde, proxy:?, event:?, data:?, conn:?, qhandle:?; "An event happened!");
	    }
}

impl
	client::Dispatch<
		protocols::xdg::shell::client::xdg_positioner::XdgPositioner,
		smithay_client_toolkit::globals::GlobalData,
	> for WindowManager
{
	fn event(
		state: &mut Self,
		proxy: &protocols::xdg::shell::client::xdg_positioner::XdgPositioner,
		event: protocols::xdg::shell::client::xdg_positioner::Event,
		data: &smithay_client_toolkit::globals::GlobalData,
		conn: &client::Connection,
		qhandle: &client::QueueHandle<Self>,
	) {
		log::debug!(state:serde, proxy:?, event:?, data:?, conn:?, qhandle:?; "An event happened!");
	}
}

impl
	client::Dispatch<
		client::protocol::wl_compositor::WlCompositor,
		smithay_client_toolkit::globals::GlobalData,
	> for WindowManager
{
	fn event(
		state: &mut Self,
		proxy: &client::protocol::wl_compositor::WlCompositor,
		event: client::protocol::wl_compositor::Event,
		data: &smithay_client_toolkit::globals::GlobalData,
		conn: &client::Connection,
		qhandle: &client::QueueHandle<Self>,
	) {
		log::debug!(state:serde, proxy:?, event:?, data:?, conn:?, qhandle:?; "An event happened!");
	}
}

impl
	client::Dispatch<
		client::protocol::wl_surface::WlSurface,
		smithay_client_toolkit::compositor::SurfaceData,
	> for WindowManager
{
	fn event(
		state: &mut Self,
		proxy: &client::protocol::wl_surface::WlSurface,
		event: client::protocol::wl_surface::Event,
		data: &smithay_client_toolkit::compositor::SurfaceData,
		conn: &client::Connection,
		qhandle: &client::QueueHandle<Self>,
	) {
		log::debug!(state:serde, proxy:?, event:?, data:?, conn:?, qhandle:?; "An event happened!");
	}
}

impl
	client::Dispatch<
		protocols::xdg::shell::client::xdg_surface::XdgSurface,
		smithay_client_toolkit::shell::xdg::window::WindowData,
	> for WindowManager
{
	fn event(
		state: &mut Self,
		proxy: &protocols::xdg::shell::client::xdg_surface::XdgSurface,
		event: protocols::xdg::shell::client::xdg_surface::Event,
		data: &smithay_client_toolkit::shell::xdg::window::WindowData,
		conn: &client::Connection,
		qhandle: &client::QueueHandle<Self>,
	) {
		log::debug!(state:serde, proxy:?, event:?, data:?, conn:?, qhandle:?; "An event happened!");
	}
}

impl
	client::Dispatch<
		protocols::xdg::shell::client::xdg_toplevel::XdgToplevel,
		smithay_client_toolkit::shell::xdg::window::WindowData,
	> for WindowManager
{
	fn event(
		state: &mut Self,
		proxy: &protocols::xdg::shell::client::xdg_toplevel::XdgToplevel,
		event: protocols::xdg::shell::client::xdg_toplevel::Event,
		data: &smithay_client_toolkit::shell::xdg::window::WindowData,
		conn: &client::Connection,
		qhandle: &client::QueueHandle<Self>,
	) {
		log::debug!(state:serde, proxy:?, event:?, data:?, conn:?, qhandle:?; "An event happened!");
	}
}

impl
	    client::Dispatch<
		    protocols::xdg::decoration::zv1::client::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1,
		    smithay_client_toolkit::shell::xdg::window::WindowData,
	    > for WindowManager
{
	    fn event(
		    state: &mut Self,
		    proxy: &protocols::xdg::decoration::zv1::client::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1,
		    event: protocols::xdg::decoration::zv1::client::zxdg_toplevel_decoration_v1::Event,
		    data: &smithay_client_toolkit::shell::xdg::window::WindowData,
		    conn: &client::Connection,
		    qhandle: &client::QueueHandle<Self>,
	    ) {
		    log::debug!(state:serde, proxy:?, event:?, data:?, conn:?, qhandle:?; "An event happened!");
	    }
}

impl smithay_client_toolkit::shell::xdg::window::WindowHandler for WindowManager {
	fn request_close(
		&mut self,
		conn: &client::Connection,
		qh: &client::QueueHandle<Self>,
		window: &smithay_client_toolkit::shell::xdg::window::Window,
	) {
		log::error!("Window close requested!!")
	}

	fn configure(
		&mut self,
		conn: &client::Connection,
		qh: &client::QueueHandle<Self>,
		window: &smithay_client_toolkit::shell::xdg::window::Window,
		configure: smithay_client_toolkit::shell::xdg::window::WindowConfigure,
		serial: u32,
	) {
		log::error!("Window change requested!!")
	}
}

pub fn wayland_context() -> Result<client::EventQueue<WindowManager>, anyhow::Error> {
	let connection = client::Connection::connect_to_env()?;
	let (globals, mut queue) = client::globals::registry_queue_init::<WindowManager>(&connection)?;
	let shell = smithay_client_toolkit::shell::xdg::XdgShell::bind(&globals, &queue.handle())?;
	let compositor_state =
		smithay_client_toolkit::compositor::CompositorState::bind(&globals, &queue.handle())?;
	let surface = compositor_state.create_surface(&queue.handle());
	let surface = shell.create_window(
		surface,
		smithay_client_toolkit::shell::xdg::window::WindowDecorations::RequestServer,
		&queue.handle(),
	);
	let beans = surface.wl_surface().id().as_ptr();
	Ok(queue)
}
