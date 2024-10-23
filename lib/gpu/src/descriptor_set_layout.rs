use std::sync::Arc;

use ash::vk;

use crate::*;

pub struct DescriptorSetLayoutBuilder {
    pub uniform_buffer_bindings: Vec<usize>,
    pub storage_buffer_bindings: Vec<usize>,
}

/// `DescriptorSetLayout` defines the linkage to the shader.
/// It describes which resources are defined in the shader and how they must be binded.
/// This structure does not need shader directly, it defines only linking rules.
/// It can be reused between different pipelines and shaders with the same layout.
#[derive(Clone)]
pub struct DescriptorSetLayout {
    // Device that owns the descriptor set layout.
    pub device: Arc<Device>,

    // Bindings for uniform buffers.
    // It contains index defined in the shader.
    pub uniform_buffer_bindings: Vec<usize>,

    // Bindings for storage buffers.
    // It contains index defined in the shader.
    pub storage_buffer_bindings: Vec<usize>,

    // Native Vulkan descriptor set layout handle.
    pub vk_descriptor_set_layout: vk::DescriptorSetLayout,
}

impl DescriptorSetLayoutBuilder {
    pub fn add_uniform_buffer(mut self, binding: usize) -> Self {
        self.uniform_buffer_bindings.push(binding);
        self
    }

    pub fn add_storage_buffer(mut self, binding: usize) -> Self {
        self.storage_buffer_bindings.push(binding);
        self
    }

    pub fn build(&self, device: Arc<Device>) -> GpuResult<Arc<DescriptorSetLayout>> {
        let mut descriptor_set_layout_bindings = Vec::new();
        for binding in &self.uniform_buffer_bindings {
            descriptor_set_layout_bindings.push(
                vk::DescriptorSetLayoutBinding::builder()
                    .binding(*binding as u32)
                    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                    .descriptor_count(1)
                    .stage_flags(vk::ShaderStageFlags::COMPUTE)
                    .build(),
            );
        }

        for binding in &self.storage_buffer_bindings {
            descriptor_set_layout_bindings.push(
                vk::DescriptorSetLayoutBinding::builder()
                    .binding(*binding as u32)
                    .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                    .descriptor_count(1)
                    .stage_flags(vk::ShaderStageFlags::COMPUTE)
                    .build(),
            );
        }

        let descriptor_set_layout_create_info = vk::DescriptorSetLayoutCreateInfo::builder()
            .bindings(&descriptor_set_layout_bindings)
            .build();

        let vk_descriptor_set_layout = unsafe {
            device.vk_device.create_descriptor_set_layout(
                &descriptor_set_layout_create_info,
                device.cpu_allocation_callbacks(),
            )?
        };

        Ok(Arc::new(DescriptorSetLayout {
            device,
            uniform_buffer_bindings: self.uniform_buffer_bindings.clone(),
            storage_buffer_bindings: self.storage_buffer_bindings.clone(),
            vk_descriptor_set_layout,
        }))
    }
}

impl Drop for DescriptorSetLayout {
    fn drop(&mut self) {
        unsafe {
            if self.vk_descriptor_set_layout != vk::DescriptorSetLayout::null() {
                self.device.vk_device.destroy_descriptor_set_layout(
                    self.vk_descriptor_set_layout,
                    self.device.cpu_allocation_callbacks(),
                );
                self.vk_descriptor_set_layout = vk::DescriptorSetLayout::null();
            }
            self.storage_buffer_bindings.clear();
            self.uniform_buffer_bindings.clear();
        }
    }
}

impl DescriptorSetLayout {
    pub fn builder() -> DescriptorSetLayoutBuilder {
        DescriptorSetLayoutBuilder {
            uniform_buffer_bindings: Vec::new(),
            storage_buffer_bindings: Vec::new(),
        }
    }
}