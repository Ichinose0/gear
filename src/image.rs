use crate::{Device, DeviceConnecter, DeviceMemory, Extent3d};
use ash::vk::{
    Format, ImageCreateInfo, ImageLayout, ImageTiling, ImageUsageFlags, SampleCountFlags,
    SharingMode,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageType {
    e2D,
    e3D,
}

impl Into<ash::vk::ImageType> for ImageType {
    fn into(self) -> ash::vk::ImageType {
        match self {
            ImageType::e2D => ash::vk::ImageType::TYPE_2D,
            ImageType::e3D => ash::vk::ImageType::TYPE_3D,
        }
    }
}

pub struct ImageDescriptor {
    image_type: ImageType,
    extent: Extent3d,
    mip_levels: u32,
    array_layers: u32,
}

impl ImageDescriptor {
    pub fn new() -> Self {
        Self {
            image_type: ImageType::e2D,
            extent: Extent3d::new(100, 100, 1),
            mip_levels: 1,
            array_layers: 1,
        }
    }

    pub fn image_type(mut self, image_type: ImageType) -> Self {
        self.image_type = image_type;
        self
    }

    pub fn extent(mut self, extent: Extent3d) -> Self {
        self.extent = extent;
        self
    }
}

pub struct Image<'a> {
    image: ash::vk::Image,
    memory: DeviceMemory,
    device: &'a Device,
}

impl<'a> Image<'a> {
    pub fn create(
        device: &'a Device,
        connecter: DeviceConnecter,
        descriptor: &ImageDescriptor,
    ) -> Self {
        let create_info = ImageCreateInfo::builder()
            .image_type(descriptor.image_type.into())
            .extent(descriptor.extent.into())
            .mip_levels(descriptor.mip_levels)
            .array_layers(descriptor.array_layers)
            .format(Format::R8G8B8A8_UNORM)
            .tiling(ImageTiling::LINEAR)
            .initial_layout(ImageLayout::UNDEFINED)
            .usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .samples(SampleCountFlags::TYPE_1)
            .build();
        let image = unsafe { device.device.create_image(&create_info, None) }.unwrap();
        let mem_props = connecter.get_memory_properties();
        let mem_req = unsafe { device.device.get_image_memory_requirements(image) };
        let memory = DeviceMemory::alloc_image_memory(&device.device, image, mem_props, mem_req);
        Self {
            image,
            device,
            memory,
        }
    }
}
