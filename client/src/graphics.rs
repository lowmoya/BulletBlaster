// change to priv
pub mod queues;

use ash::{
    khr::surface::Instance as SInstance,
    vk::{self, PhysicalDevice, Queue, SurfaceKHR},
    Device, Entry, Instance,
};
use std::{
    collections::HashSet,
    ffi::{c_char, CStr}
};

use queues::QueueFamilies;

use glfw::{Glfw, PWindow};

pub struct Graphics {
    _entry: Entry,
    instance: Instance,
    s_instance: SInstance,
    surface: SurfaceKHR,
    device: Device,
    graphics_queue: Queue,
    presentation_queue: Queue,
}

impl Graphics {
    pub fn new(glfw: &Glfw, window: &PWindow) -> Result<Self, &'static str> {
        let entry = Entry::linked();

        let instance = Self::create_instance(&entry, &glfw)?;
        let s_instance = SInstance::new(&entry, &instance);
        let surface = Self::create_surface(&window, &instance)?;
        let physical_device = Self::pick_device(&instance, &s_instance, surface)?;
        let families = QueueFamilies::new(&instance, &s_instance, physical_device, surface);
        let device = Self::create_logical_device(&instance, physical_device, &families)?;
        let graphics_queue = unsafe { device.get_device_queue(families.graphics.unwrap(), 0) };
        let presentation_queue =
            unsafe { device.get_device_queue(families.presentation.unwrap(), 0) };

        Ok(Self {
            _entry: entry,
            instance,
            s_instance,
            surface,
            device,
            graphics_queue,
            presentation_queue,
        })
    }

    fn create_instance(entry: &Entry, glfw: &glfw::Glfw) -> Result<Instance, &'static str> {
        let app_info = vk::ApplicationInfo {
            p_application_name: c"Bullet Blaster".as_ptr(),
            api_version: vk::make_api_version(0, 1, 0, 0),
            ..Default::default()
        };

        let extensions = glfw
            .get_required_instance_extensions()
            .unwrap_or(Vec::default());
        let extension_pointers = extensions
            .iter()
            .map(|s| s.as_ptr() as *const i8)
            .collect::<Vec<*const i8>>();

        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            pp_enabled_extension_names: extension_pointers.as_ptr(),
            enabled_extension_count: extension_pointers.len() as u32,
            #[cfg(debug_assertions)]
            enabled_layer_count: 1,
            #[cfg(debug_assertions)]
            pp_enabled_layer_names: [c"VK_LAYER_KHRONOS_validation".as_ptr()].as_ptr(),
            ..Default::default()
        };

        unsafe {
            match entry.create_instance(&create_info, None) {
                Ok(instance) => Ok(instance),
                Err(error) => {
                    println!("{error}");
                    Err("Failed to create Vulkan instance.")
                }
            }
        }
    }

    fn create_surface(window: &PWindow, instance: &Instance) -> Result<SurfaceKHR, &'static str> {
        let mut surface = std::mem::MaybeUninit::uninit();
        if window.create_window_surface(instance.handle(), std::ptr::null(), surface.as_mut_ptr())
            != ash::vk::Result::SUCCESS
        {
            return Err("Failed to create Surface for Vulkan.");
        }
        Ok(unsafe { surface.assume_init() })
    }

    fn get_required_device_extensions() -> Vec<CStr> {
        vec![ash::khr::swapchain::NAME]
    }

    fn pick_device(
        instance: &Instance,
        s_instance: &SInstance,
        surface: SurfaceKHR,
    ) -> Result<PhysicalDevice, &'static str> {
        let devices = unsafe { instance.enumerate_physical_devices() }.unwrap_or(Vec::default());

        let required_extensions = Self::get_required_device_extensions();
        let required_extension_ptrs = required_extensions
            .iter()
            .map(|s| s.as_ptr())
            .collect::<Vec<*const i8>>();
        let mut selected: (Option<PhysicalDevice>, u32) = (None, 0);
        'device: for device in devices {
            let families = QueueFamilies::new(instance, s_instance, device, surface);
            if !families.is_valid() {
                continue;
            }

            let supported_extensions = unsafe {
                instance.enumerate_device_extension_properties(device)
                    .unwrap_or(Vec::default())
            };
            let supported_extension_ptrs = supported_extensions
                    .iter()
                    .map(|e| e.extension_name.as_ptr() )
                    .collect::<Vec<*const i8>>();

            'r_ext: for r_ext in &required_extension_ptrs {
                for s_ext in &supported_extension_ptrs {
                    /*if *r_ext == s_ext {
                        continue 'r_ext;
                    }*/
                }
                continue 'device;
            }


            let _fc = unsafe { instance.get_physical_device_features(device) };
            let pc = unsafe { instance.get_physical_device_properties(device) };

            let mut score = 0;

            if pc.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
                score += 3000;
            }
            score += pc.limits.max_image_dimension2_d;

            if score > selected.1 {
                selected = (Some(device), score);
            }
        }

        match selected.0 {
            Some(device) => Ok(device),
            None => Err("No suitable GPUs for this application found"),
        }
    }

    fn create_logical_device(
        instance: &Instance,
        physical_device: PhysicalDevice,
        families: &QueueFamilies,
    ) -> Result<Device, &'static str> {
        /* Prep queue creation info. */
        let mut queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = Vec::new();

        /* Get the list of unique indices. */
        let mut indices = HashSet::new();
        indices.insert(families.graphics.unwrap());
        indices.insert(families.presentation.unwrap());

        /* Create queue infos for each unique index. */
        let priority = 1.0;
        for i in indices {
            queue_create_infos.push(vk::DeviceQueueCreateInfo {
                queue_family_index: i,
                queue_count: 1,
                p_queue_priorities: &priority,
                ..Default::default()
            });
        }

        /* Create logical device. */
        let extensions = Self::get_required_device_extensions();
        let extension_ptrs = extensions
            .iter()
            .map(|s| s.as_ptr() as *const i8)
            .collect::<Vec<*const i8>>();
        let create_info = vk::DeviceCreateInfo {
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            enabled_extension_count: extensions.len() as u32,
            pp_enabled_extension_names: extension_ptrs.as_ptr(),
            ..Default::default()
        };

        unsafe {
            match instance.create_device(physical_device, &create_info, None) {
                Ok(device) => Ok(device),
                Err(_) => Err("Failed to create vulkan device"),
            }
        }
    }
}

impl Drop for Graphics {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.s_instance.destroy_surface(self.surface, None);
            self.instance.destroy_instance(None);
        }
    }
}
