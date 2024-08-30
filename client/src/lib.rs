use ash::{
    khr,
    vk::{self, PhysicalDevice, Queue, QueueFlags},
    Device, Entry, Instance,
};
use std::collections::HashSet;

pub struct App {
    _entry: Entry,
    instance: Instance,
    device: Device,
    graphics_queue: Queue,
}

struct QueueFamilies {
    graphics: Option<u32>,
}

impl QueueFamilies {
    fn default() -> Self {
        Self { graphics: None }
    }

    fn valid(&self) -> bool {
        self.graphics != None
    }
}

/// Core implementation
impl App {
    pub fn new() -> Result<Self, &'static str> {
        let entry = Entry::linked();

        let instance = Self::vk_create_instance(&entry)?;
        let physical_device = Self::vk_pick_device(&instance)?;
        let families = Self::vk_get_queue_families(&instance, physical_device);
        let device = Self::vk_create_logical_device(&instance, physical_device, &families)?;
        let graphics_queue = unsafe { device.get_device_queue(families.graphics.unwrap(), 0) };

        Ok(Self {
            _entry: entry,
            instance,
            device,
            graphics_queue,
        })
    }
}

/// Graphics implementation
impl App {
    fn vk_create_instance(entry: &Entry) -> Result<Instance, &'static str> {
        let app_info = vk::ApplicationInfo {
            p_application_name: c"Bullet Blaster".as_ptr(),
            api_version: vk::make_api_version(0, 1, 0, 0),
            ..Default::default()
        };

        let extensions = [khr::surface::NAME.as_ptr()];
        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            pp_enabled_extension_names: extensions.as_ptr(),
            /*#[cfg(debug_assertions)]
            enabled_layer_count: 1,
            #[cfg(debug_assertions)]
            pp_enabled_layer_names: [
                c"VK_LAYER_LUNARG_standard_validation".as_ptr()
            ].as_ptr(),*/
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

    fn vk_get_queue_families(instance: &Instance, device: PhysicalDevice) -> QueueFamilies {
        let properties = unsafe { instance.get_physical_device_queue_family_properties(device) };

        let mut families = QueueFamilies::default();

        for (i, prop) in properties.iter().enumerate() {
            if prop.queue_flags & QueueFlags::GRAPHICS == QueueFlags::GRAPHICS {
                families.graphics = Some(i as u32);
            }
        }

        families
    }

    fn vk_pick_device(instance: &Instance) -> Result<PhysicalDevice, &'static str> {
        let devices = match unsafe { instance.enumerate_physical_devices() } {
            Ok(devices) => Ok(devices),
            Err(_) => Err("No suitable GPUs for this application found"),
        }?;

        let mut selected: (Option<PhysicalDevice>, u32) = (None, 0);
        let mut fc = ash::vk::PhysicalDeviceFeatures2::default();
        let mut pc = ash::vk::PhysicalDeviceProperties2::default();
        for device in devices {
            let families = Self::vk_get_queue_families(instance, device);
            if !families.valid() {
                continue;
            }

            unsafe {
                instance.get_physical_device_features2(device, &mut fc);
                instance.get_physical_device_properties2(device, &mut pc);
            }

            let mut score = 0;

            if pc.properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
                score += 3000;
            }
            score += pc.properties.limits.max_image_dimension2_d;

            if score > selected.1 {
                selected = (Some(device), score);
            }
        }

        match selected.0 {
            Some(device) => Ok(device),
            None => Err("No suitable GPUs for this application found"),
        }
    }

    fn vk_create_logical_device(
        instance: &Instance,
        physical_device: PhysicalDevice,
        families: &QueueFamilies,
    ) -> Result<Device, &'static str> {
        /* Prep queue creation info. */
        let mut queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = Vec::new();

        /* Get the list of unique indices. */
        let mut indices = HashSet::new();
        indices.insert(families.graphics.unwrap());

        /* Create queue infos for each unique index. */
        for i in indices {
            queue_create_infos.push(vk::DeviceQueueCreateInfo {
                queue_family_index: i,
                queue_count: 1,
                ..Default::default()
            });
        }

        /* Create logical device. */
        let create_info = vk::DeviceCreateInfo {
            queue_create_info_count: 1,
            p_queue_create_infos: queue_create_infos.as_ptr(),
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

impl Drop for App {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
            self.device.destroy_device(None);
            // TODO Leave out the device deletion and see if a message is
            //      shown in console warning about doing so.
        }
    }
}
