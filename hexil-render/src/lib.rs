use std::sync::Arc;
use thiserror::Error;
use vulkano as vk;

#[allow(dead_code)]
#[cfg(target_os = "linux")]
const LINUX_TARGET: bool = true;
#[allow(dead_code)]
#[cfg(not(target_os = "linux"))]
const LINUX_TARGET: bool = false;
#[allow(dead_code)]
#[cfg(target_os = "windows")]
const WINDOWS_TARGET: bool = true;
#[allow(dead_code)]
#[cfg(not(target_os = "windows"))]
const WINDOWS_TARGET: bool = false;
#[allow(dead_code)]
#[cfg(target_os = "macos")]
const MACOS_TARGET: bool = true;
#[allow(dead_code)]
#[cfg(not(target_os = "macos"))]
const MACOS_TARGET: bool = false;

#[derive(Error, Debug)]
pub enum HexilRenderError {
	#[error("Failed to load vulkan library!")]
	VKInitError(#[from] vk::LoadingError),
	#[error("Vulkan validation error! {0}")]
	Validated(#[from] vk::Validated<vk::VulkanError>),
	#[error("Vulkan error! {0}")]
	VKError(#[from] vk::VulkanError),
	#[error("No permissible physical devices found!")]
	IncompatibleHardware,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum PowerProfile {
	HighPower,
	Efficient,
}

impl PowerProfile {
	const fn device_ordering(
		self,
		device1: vk::device::physical::PhysicalDeviceType,
		device2: vk::device::physical::PhysicalDeviceType,
	) -> std::cmp::Ordering {
		use std::cmp::Ordering::*;
		match (self, device1, device2) {
			(
				_,
				vulkano::device::physical::PhysicalDeviceType::IntegratedGpu,
				vulkano::device::physical::PhysicalDeviceType::IntegratedGpu,
			) => Equal,
			(
				PowerProfile::HighPower,
				vulkano::device::physical::PhysicalDeviceType::IntegratedGpu,
				vulkano::device::physical::PhysicalDeviceType::DiscreteGpu,
			) => Less,
			(
				PowerProfile::HighPower,
				vulkano::device::physical::PhysicalDeviceType::IntegratedGpu,
				vulkano::device::physical::PhysicalDeviceType::VirtualGpu,
			) => Less,
			(
				_,
				vulkano::device::physical::PhysicalDeviceType::IntegratedGpu,
				vulkano::device::physical::PhysicalDeviceType::Cpu,
			) => Greater,
			(
				_,
				vulkano::device::physical::PhysicalDeviceType::IntegratedGpu,
				vulkano::device::physical::PhysicalDeviceType::Other,
			) => Equal,
			(
				PowerProfile::HighPower,
				vulkano::device::physical::PhysicalDeviceType::DiscreteGpu,
				vulkano::device::physical::PhysicalDeviceType::IntegratedGpu,
			) => Greater,
			(
				_,
				vulkano::device::physical::PhysicalDeviceType::DiscreteGpu,
				vulkano::device::physical::PhysicalDeviceType::DiscreteGpu,
			) => Equal,
			(
				PowerProfile::HighPower,
				vulkano::device::physical::PhysicalDeviceType::DiscreteGpu,
				vulkano::device::physical::PhysicalDeviceType::VirtualGpu,
			) => Greater,
			(
				_,
				vulkano::device::physical::PhysicalDeviceType::DiscreteGpu,
				vulkano::device::physical::PhysicalDeviceType::Cpu,
			) => Greater,
			(
				PowerProfile::HighPower,
				vulkano::device::physical::PhysicalDeviceType::DiscreteGpu,
				vulkano::device::physical::PhysicalDeviceType::Other,
			) => Greater,
			(
				PowerProfile::HighPower,
				vulkano::device::physical::PhysicalDeviceType::VirtualGpu,
				vulkano::device::physical::PhysicalDeviceType::IntegratedGpu,
			) => Greater,
			(
				PowerProfile::HighPower,
				vulkano::device::physical::PhysicalDeviceType::VirtualGpu,
				vulkano::device::physical::PhysicalDeviceType::DiscreteGpu,
			) => Less,
			(
				_,
				vulkano::device::physical::PhysicalDeviceType::VirtualGpu,
				vulkano::device::physical::PhysicalDeviceType::VirtualGpu,
			) => Equal,
			(
				_,
				vulkano::device::physical::PhysicalDeviceType::VirtualGpu,
				vulkano::device::physical::PhysicalDeviceType::Cpu,
			) => Greater,
			(
				PowerProfile::HighPower,
				vulkano::device::physical::PhysicalDeviceType::VirtualGpu,
				vulkano::device::physical::PhysicalDeviceType::Other,
			) => Greater,
			(
				_,
				vulkano::device::physical::PhysicalDeviceType::Cpu,
				vulkano::device::physical::PhysicalDeviceType::IntegratedGpu,
			) => Less,
			(
				_,
				vulkano::device::physical::PhysicalDeviceType::Cpu,
				vulkano::device::physical::PhysicalDeviceType::DiscreteGpu,
			) => Less,
			(
				_,
				vulkano::device::physical::PhysicalDeviceType::Cpu,
				vulkano::device::physical::PhysicalDeviceType::VirtualGpu,
			) => Less,
			(
				_,
				vulkano::device::physical::PhysicalDeviceType::Cpu,
				vulkano::device::physical::PhysicalDeviceType::Cpu,
			) => Equal,
			(
				_,
				vulkano::device::physical::PhysicalDeviceType::Cpu,
				vulkano::device::physical::PhysicalDeviceType::Other,
			) => Less,
			(
				PowerProfile::HighPower,
				vulkano::device::physical::PhysicalDeviceType::Other,
				vulkano::device::physical::PhysicalDeviceType::IntegratedGpu,
			) => Equal,
			(
				PowerProfile::HighPower,
				vulkano::device::physical::PhysicalDeviceType::Other,
				vulkano::device::physical::PhysicalDeviceType::DiscreteGpu,
			) => Less,
			(
				PowerProfile::HighPower,
				vulkano::device::physical::PhysicalDeviceType::Other,
				vulkano::device::physical::PhysicalDeviceType::VirtualGpu,
			) => Less,
			(
				_,
				vulkano::device::physical::PhysicalDeviceType::Other,
				vulkano::device::physical::PhysicalDeviceType::Cpu,
			) => Greater,
			(
				_,
				vulkano::device::physical::PhysicalDeviceType::Other,
				vulkano::device::physical::PhysicalDeviceType::Other,
			) => Equal,
			(
				PowerProfile::Efficient,
				vulkano::device::physical::PhysicalDeviceType::IntegratedGpu,
				vulkano::device::physical::PhysicalDeviceType::DiscreteGpu,
			) => Greater,
			(
				PowerProfile::Efficient,
				vulkano::device::physical::PhysicalDeviceType::IntegratedGpu,
				vulkano::device::physical::PhysicalDeviceType::VirtualGpu,
			) => Less,
			(
				PowerProfile::Efficient,
				vulkano::device::physical::PhysicalDeviceType::DiscreteGpu,
				vulkano::device::physical::PhysicalDeviceType::IntegratedGpu,
			) => Less,
			(
				PowerProfile::Efficient,
				vulkano::device::physical::PhysicalDeviceType::DiscreteGpu,
				vulkano::device::physical::PhysicalDeviceType::VirtualGpu,
			) => Less,
			(
				PowerProfile::Efficient,
				vulkano::device::physical::PhysicalDeviceType::DiscreteGpu,
				vulkano::device::physical::PhysicalDeviceType::Other,
			) => Less,
			(
				PowerProfile::Efficient,
				vulkano::device::physical::PhysicalDeviceType::VirtualGpu,
				vulkano::device::physical::PhysicalDeviceType::IntegratedGpu,
			) => Greater,
			(
				PowerProfile::Efficient,
				vulkano::device::physical::PhysicalDeviceType::VirtualGpu,
				vulkano::device::physical::PhysicalDeviceType::DiscreteGpu,
			) => Greater,
			(
				PowerProfile::Efficient,
				vulkano::device::physical::PhysicalDeviceType::VirtualGpu,
				vulkano::device::physical::PhysicalDeviceType::Other,
			) => Greater,
			(
				PowerProfile::Efficient,
				vulkano::device::physical::PhysicalDeviceType::Other,
				vulkano::device::physical::PhysicalDeviceType::IntegratedGpu,
			) => Less,
			(
				PowerProfile::Efficient,
				vulkano::device::physical::PhysicalDeviceType::Other,
				vulkano::device::physical::PhysicalDeviceType::DiscreteGpu,
			) => Greater,
			(
				PowerProfile::Efficient,
				vulkano::device::physical::PhysicalDeviceType::Other,
				vulkano::device::physical::PhysicalDeviceType::VirtualGpu,
			) => Less,
			_ => unreachable!(),
		}
	}
}

#[derive(Debug)]
pub struct VKContext {
	instance: Arc<vk::instance::Instance>,
	device: Arc<vk::device::Device>,
	render_queue: Arc<vk::device::Queue>,
	transf_queue: Arc<vk::device::Queue>,
}

impl VKContext {
	fn device_is_permissible(device: &Arc<vk::device::physical::PhysicalDevice>) -> bool {
		device
			.supported_extensions()
			.contains(&Self::required_device_extensions())
	}
	fn required_device_extensions() -> vk::device::DeviceExtensions {
		vk::device::DeviceExtensions {
			khr_16bit_storage: true,
			khr_8bit_storage: true,
			khr_bind_memory2: false,
			khr_copy_commands2: false,
			khr_create_renderpass2: false,
			khr_dedicated_allocation: false,
			khr_deferred_host_operations: false,
			khr_descriptor_update_template: false,
			khr_display_swapchain: false,
			khr_draw_indirect_count: false,
			khr_driver_properties: false,
			khr_dynamic_rendering: false,
			khr_format_feature_flags2: false,
			khr_fragment_shader_barycentric: false,
			khr_get_memory_requirements2: false,
			khr_image_format_list: false,
			khr_imageless_framebuffer: false,
			khr_incremental_present: false,
			khr_maintenance1: false,
			khr_maintenance2: false,
			khr_maintenance3: false,
			khr_maintenance4: false,
			khr_map_memory2: false,
			khr_multiview: false,
			khr_pipeline_library: false,
			khr_portability_subset: false,
			khr_present_id: false,
			khr_present_wait: false,
			khr_push_descriptor: true,
			khr_shader_draw_parameters: false,
			khr_shader_float16_int8: false,
			khr_shader_float_controls: false,
			khr_shader_integer_dot_product: false,
			khr_shader_terminate_invocation: false,
			khr_shared_presentable_image: false,
			khr_spirv_1_4: true,
			khr_storage_buffer_storage_class: false,
			khr_swapchain: true,
			khr_swapchain_mutable_format: false,
			khr_synchronization2: false,
			khr_uniform_buffer_standard_layout: false,
			khr_vulkan_memory_model: false,
			ext_descriptor_buffer: false,
			ext_descriptor_indexing: false,
			ext_extended_dynamic_state: false,
			ext_extended_dynamic_state2: false,
			ext_extended_dynamic_state3: false,
			ext_hdr_metadata: false,
			ext_index_type_uint8: false,
			ext_inline_uniform_block: false,
			ext_line_rasterization: false,
			ext_load_store_op_none: false,
			ext_multi_draw: false,
			ext_multisampled_render_to_single_sampled: false,
			ext_pipeline_creation_cache_control: false,
			ext_pipeline_creation_feedback: false,
			ext_primitive_topology_list_restart: false,
			ext_primitives_generated_query: false,
			ext_swapchain_maintenance1: false, //
			img_filter_cubic: false,
			..Default::default()
		}
	}

	pub fn new(profile: PowerProfile) -> Result<Self, HexilRenderError> {
		let mut create_info = vk::instance::InstanceCreateInfo::application_from_cargo_toml();
		create_info.enabled_extensions = vk::instance::InstanceExtensions {
			khr_display: true,
			khr_get_display_properties2: true,
			khr_get_physical_device_properties2: true,
			khr_get_surface_capabilities2: true,
			khr_surface: true,
			khr_wayland_surface: LINUX_TARGET,
			khr_win32_surface: WINDOWS_TARGET,
			ext_surface_maintenance1: true,
			ext_swapchain_colorspace: true,
			..Default::default()
		};
		let instance =
			vk::instance::Instance::new(vk::library::VulkanLibrary::new()?, create_info)?;

		let physical_device = {
			let devices = instance.enumerate_physical_devices()?;

			let candidates = devices.filter(Self::device_is_permissible);

			Result::<_, HexilRenderError>::Ok(candidates.max_by(|device1, device2| {
				profile.device_ordering(
					device1.properties().device_type,
					device2.properties().device_type,
				)
			}))
		}?
		.ok_or(HexilRenderError::IncompatibleHardware)?;

		let (device, mut queues) = {
			let fams = physical_device.queue_family_properties();
			let render_family = fams
				.iter()
				.enumerate()
				.filter(|(_, queue)| queue.queue_flags.contains(vk::device::QueueFlags::GRAPHICS))
				.max_by_key(|(_, queue)| queue.queue_count)
				.map(|(idx, _)| idx)
				.ok_or(HexilRenderError::IncompatibleHardware)?;
			let transf_family = fams
				.iter()
				.enumerate()
				.filter(|(_, queue)| queue.queue_flags.contains(vk::device::QueueFlags::TRANSFER))
				.max_by_key(|(_, queue)| queue.queue_count)
				.map(|(idx, _)| idx)
				.ok_or(HexilRenderError::IncompatibleHardware)?;

			let infos = vk::device::DeviceCreateInfo {
				queue_create_infos: vec![
					vk::device::QueueCreateInfo {
						flags: vk::device::QueueCreateFlags::empty(),
						queue_family_index: render_family as u32,
						..Default::default()
					},
					vk::device::QueueCreateInfo {
						flags: vk::device::QueueCreateFlags::empty(),
						queue_family_index: transf_family as u32,
						..Default::default()
					},
				],
				enabled_extensions: Self::required_device_extensions(),
				enabled_features: vk::device::Features::empty(),
				private_data_slot_request_count: 0,
				..Default::default()
			};

			vk::device::Device::new(physical_device, infos)
		}?;
		let render_queue = queues.next().unwrap();
		let transf_queue = queues.next().unwrap();

		Ok(Self {
			instance,
			device,
			render_queue,
			transf_queue,
		})
	}
}
