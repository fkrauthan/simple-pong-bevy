use bevy::ecs::{Local, Res, QuerySet, Query};
use bevy::app::{EventReader, Events};
use bevy::window::{WindowResized, WindowCreated, Windows, Window};
use bevy::render::camera::Camera;
use bevy::prelude::{Entity, Transform, Vec3, Added};

#[derive(Debug)]
pub enum ScaleType {
    Fit,
    Stretch,
}
impl Default for ScaleType {
    fn default() -> Self {
        ScaleType::Fit
    }
}

#[derive(Default, Debug)]
pub struct VirtualScreen {
    pub width: f32,
    pub height: f32,
    pub scale_type: ScaleType,
}

#[derive(Default)]
pub struct CameraSystemScreenState {
    window_resized_event_reader: EventReader<WindowResized>,
    window_created_event_reader: EventReader<WindowCreated>,
}

pub fn camera_virtual_screen_system(
    mut state: Local<CameraSystemScreenState>,
    window_resized_events: Res<Events<WindowResized>>,
    window_created_events: Res<Events<WindowCreated>>,
    windows: Res<Windows>,
    mut queries: QuerySet<(
        Query<(Entity, &Camera, &VirtualScreen, &mut Transform)>,
        Query<Entity, Added<Camera>>,
    )>,
) {
    let mut changed_window_ids = Vec::new();
    // handle resize events. latest events are handled first because we only want to resize each window once
    for event in state
        .window_resized_event_reader
        .iter(&window_resized_events)
        .rev()
    {
        if changed_window_ids.contains(&event.id) {
            continue;
        }

        changed_window_ids.push(event.id);
    }

    // handle resize events. latest events are handled first because we only want to resize each window once
    for event in state
        .window_created_event_reader
        .iter(&window_created_events)
        .rev()
    {
        if changed_window_ids.contains(&event.id) {
            continue;
        }

        changed_window_ids.push(event.id);
    }

    let mut added_cameras = vec![];
    for entity in &mut queries.q1().iter() {
        added_cameras.push(entity);
    }
    for (entity, camera, virtual_screen, mut transform) in queries.q0_mut().iter_mut() {
        if let Some(window) = windows.get(camera.window) {
            if changed_window_ids.contains(&window.id()) || added_cameras.contains(&entity) {
                transform.scale = calculate_scale_factor(virtual_screen, window);
            }
        }
    }
}

fn calculate_scale_factor(config: &VirtualScreen, window: &Window) -> Vec3 {
    let x_scale = config.width / window.width() as f32;
    let y_scale = config.height / window.height() as f32;

    match config.scale_type {
        ScaleType::Stretch => Vec3::new(x_scale, y_scale, 1.0),
        ScaleType::Fit => Vec3::new(x_scale.max(y_scale), x_scale.max(y_scale), 1.0)
    }
}
