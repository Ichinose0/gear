use crate::{Destroy, Device, FrameBuffer, Instance};
use ash::vk::{
    MemoryAllocateInfo, MemoryPropertyFlags, MemoryRequirements, PhysicalDeviceMemoryProperties,
};

pub struct DeviceMemory {
    pub(crate) memory: ash::vk::DeviceMemory,
}

impl DeviceMemory {
    fn alloc(
        device: &ash::Device,
        mem_props: PhysicalDeviceMemoryProperties,
        mem_req: MemoryRequirements,
    ) -> ash::vk::DeviceMemory {
        let mut info = MemoryAllocateInfo::builder().allocation_size(mem_req.size);
        let mut mem_found = false;

        for i in 0..mem_props.memory_type_count {
            if (mem_req.memory_type_bits & (1 << i)) != 0
                && (mem_props.memory_types[i as usize].property_flags
                    & MemoryPropertyFlags::HOST_VISIBLE)
                    .as_raw()
                    != 0
            {
                info.memory_type_index = i;
                mem_found = true;
            }
        }

        if !mem_found {
            panic!("No suitable memory found");
        }

        unsafe { device.allocate_memory(&info.build(), None) }.unwrap()
    }

    pub fn alloc_image_memory(
        device: &ash::Device,
        image: ash::vk::Image,
        mem_props: PhysicalDeviceMemoryProperties,
        mem_req: MemoryRequirements,
    ) -> Self {
        let memory = Self::alloc(device, mem_props, mem_req);
        unsafe {
            device.bind_image_memory(image, memory, 0).unwrap();
        }
        Self { memory }
    }

    pub fn alloc_buffer_memory(
        device: &ash::Device,
        buffer: ash::vk::Buffer,
        mem_props: PhysicalDeviceMemoryProperties,
        mem_req: MemoryRequirements,
    ) -> Self {
        let memory = Self::alloc(device, mem_props, mem_req);
        unsafe {
            device.bind_buffer_memory(buffer, memory, 0).unwrap();
        }
        Self { memory }
    }
}

impl Destroy for DeviceMemory {
    fn instance(&self, instance: &Instance) {}

    fn device(&self, device: &Device) {
        unsafe {
            device.device.free_memory(self.memory, None);
        }
    }
}
