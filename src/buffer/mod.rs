use std::ffi::c_void;
use ash::vk::{BufferCreateInfo, BufferUsageFlags, MappedMemoryRange, MemoryMapFlags, SharingMode};
use crate::{Device, DeviceConnecter, Instance};
use crate::mem::DeviceMemory;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum BufferUsage {
    Vertex
}

impl Into<ash::vk::BufferUsageFlags> for BufferUsage {
    fn into(self) -> BufferUsageFlags {
        match self {
            BufferUsage::Vertex => BufferUsageFlags::VERTEX_BUFFER
        }
    }
}

pub struct BufferDescriptor {
    size: usize,
    usage: BufferUsage
}

impl BufferDescriptor {
    pub fn empty() -> Self {
        Self {
            size: 0,
            usage: BufferUsage::Vertex
        }
    }

    pub fn size(mut self,size: usize) -> Self {
        self.size = size;
        self
    }

    pub fn usage(mut self,usage: BufferUsage) -> Self {
        self.usage = usage;
        self
    }
}

pub struct Buffer {
    buffer: ash::vk::Buffer,
    memory: DeviceMemory,
    size: usize
}

impl Buffer {
    pub fn new(instance: &Instance,connecter: DeviceConnecter, device: &Device,descriptor: &BufferDescriptor) -> Self {
        let create_info = BufferCreateInfo::builder().size(descriptor.size as u64).usage(descriptor.usage.into()).sharing_mode(SharingMode::EXCLUSIVE).build();
        let buffer = unsafe { device.device.create_buffer(&create_info,None) }.unwrap();
        let mem_props = connecter.get_memory_properties();
        let mem_req = unsafe { device.device.get_buffer_memory_requirements(buffer) };
        let memory = DeviceMemory::alloc_buffer_memory(&device.device,buffer,mem_props,mem_req);

        Self {
            buffer,
            memory,
            size: descriptor.size
        }
    }

    pub fn write(&self,device: &Device,data: *const c_void) {
        let mapped_memory = unsafe {
            device.device.map_memory(self.memory.memory,0,self.size as u64,MemoryMapFlags::empty()).unwrap()
        };

        mem_copy(mapped_memory,data,self.size);
        let flush_memory_range = MappedMemoryRange::builder().memory(self.memory.memory).offset(0).size(self.size as u64).build();
        unsafe {
            device.device.flush_mapped_memory_ranges(&[flush_memory_range]).unwrap();
        }
    }

    pub fn lock(&self,device: &Device) {
        unsafe {
            device.device.unmap_memory(self.memory.memory);
        }
    }
}

pub(crate) fn mem_copy<T>(dst: *mut T,src: *const T,count: usize) {
    unsafe {
        std::ptr::copy_nonoverlapping(src,dst,count);
    }
}