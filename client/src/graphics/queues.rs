use ash::{
    vk::{PhysicalDevice, QueueFlags},
    Instance,
};

pub struct QueueFamilies {
    pub graphics: Option<u32>,
}

impl QueueFamilies {
    pub fn new(instance: &Instance, device: PhysicalDevice) -> Self {
        let mut families = Self::default();

        let properties = unsafe { instance.get_physical_device_queue_family_properties(device) };

        for (i, prop) in properties.iter().enumerate() {
            if prop.queue_flags & QueueFlags::GRAPHICS == QueueFlags::GRAPHICS {
                families.graphics = Some(i as u32);
            }
        }

        families
    }

    pub fn is_valid(&self) -> bool {
        self.graphics != None
    }
}

impl Default for QueueFamilies {
    fn default() -> Self {
        Self { graphics: None }
    }
}
