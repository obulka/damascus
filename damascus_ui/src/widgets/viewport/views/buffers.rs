use eframe::egui_wgpu::wgpu;

pub struct Buffer {
    pub buffer: wgpu::Buffer,
    pub visibility: wgpu::ShaderStages,
}
