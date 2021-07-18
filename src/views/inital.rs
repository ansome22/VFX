// Build the UI
self.platform
    .prepare_frame(self.imgui.io_mut(), &display.window)
    .expect("Failed to prepare frame!");
let ui = self.imgui.frame();
{
    let window = imgui::Window::new(im_str!("Hello Imgui from WGPU!"));
    window
        .size([300.0, 100.0], Condition::FirstUseEver)
        .build(&ui, || {
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("This is a demo of imgui-rs using imgui-wgpu!"));
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(im_str!(
                "Mouse Position: ({:.1}, {:.1})",
                mouse_pos[0],
                mouse_pos[1],
            ));
        });
}

// Prepare to render
let mut encoder = display.device.create_command_encoder(&Default::default());
let output = match display.swap_chain.get_current_frame() {
    Ok(frame) => frame,
    Err(e) => {
        eprintln!("Error getting frame: {:?}", e);
        return;
    }
}.output;

// Render the scene
self.canvas.render(
    &display.queue, 
    &mut encoder, 
    &output.view, 
    display.sc_desc.width as f32, 
    display.sc_desc.height as f32
);

// Render the UI
if self.last_cursor != ui.mouse_cursor() {
    self.last_cursor = ui.mouse_cursor();
    self.platform.prepare_render(&ui, &display.window);
}

let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    label: Some("UI RenderPass"),
    color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
        attachment: &output.view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Load,
            store: true,
        },
    }],
    depth_stencil_attachment: None,
});
self.renderer
    .render(ui.render(), &display.queue, &display.device, &mut pass)
    .expect("Failed to render UI!");
drop(pass);

display.queue.submit(Some(encoder.finish()));
