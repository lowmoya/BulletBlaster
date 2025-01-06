use ash::{
    khr::surface::Instance as SInstance,
    vk::{PhysicalDevice, QueueFlags, SurfaceKHR},
    Instance,
};

pub struct QueueFamilies {
    pub graphics: Option<u32>,
    pub presentation: Option<u32>,
}

impl QueueFamilies {
    pub fn new(
        instance: &Instance,
        s_instance: &SInstance,
        device: PhysicalDevice,
        surface: SurfaceKHR,
    ) -> Self {
        let mut families = Self::default();

        let properties = unsafe { instance.get_physical_device_queue_family_properties(device) };

        for (i, prop) in properties.iter().enumerate() {
            if prop.queue_flags & QueueFlags::GRAPHICS == QueueFlags::GRAPHICS {
                families.graphics = Some(i as u32);
            }

            let present_support = unsafe {
                s_instance.get_physical_device_surface_support(device, i as u32, surface)
            };
            if present_support == Ok(true) {
                families.presentation = Some(i as u32);
            }
        }

        families
    }

    pub fn is_valid(&self) -> bool {
        self.graphics != None && self.presentation != None
    }
}

impl Default for QueueFamilies {
    fn default() -> Self {
        Self {
            graphics: None,
            presentation: None,
        }
    }
}
