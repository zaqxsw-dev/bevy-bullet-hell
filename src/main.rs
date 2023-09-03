// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::render::render_resource::WgpuFeatures;
use bevy::render::RenderPlugin;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use bevy::{prelude::*, render::settings::WgpuSettings};
use bevy_game::{GameData, GamePlugin, MainCamera, Mouse};
use bevy_hanabi::HanabiPlugin;
use std::io::Cursor;
use winit::window::Icon;

fn main() {
	let mut wgpu_settings = WgpuSettings::default();
	wgpu_settings.features.set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

	App::new()
		//.insert_resource(Msaa::Off)
		.init_resource::<GameData>()
		.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
		.insert_resource(Mouse {
			position: Vec2::ZERO,
			area: Vec2::ZERO,
		})
		.add_plugins(
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: "Bevy game".to_string(), // ToDo
						resolution: (800., 600.).into(),
						canvas: Some("#bevy".to_owned()),
						..default()
					}),
					..default()
				})
				.set(RenderPlugin { wgpu_settings }),
		)
		.add_plugins((HanabiPlugin, GamePlugin))
		.add_startup_system(init)
		.add_system(set_window_icon.on_startup())
		.add_system(my_cursor_system)
		.run();
}

fn init(mut commands: Commands) {
	let camera = Camera2dBundle::default();
	commands.spawn(camera).insert(MainCamera);
}

//fn window_system(resize_event: Res<Events<WindowResized>>) {
//	let mut reader = resize_event.get_reader();
//	for e in reader.iter(&resize_event) {
//		println!("width = {} height = {}", e.width, e.height);
//	}
//}

fn my_cursor_system(
	// need to get window dimensions
	windows: Query<&mut Window>,
	// query to get camera transform
	camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
	mut mouse: ResMut<Mouse>,
) {
	// get the camera info and transform
	// assuming there is exactly one main camera entity, so query::single() is OK
	let (camera, camera_transform) = camera_q.single();

	// get the window that the camera is displaying to (or the primary window)
	let window = windows.get_single().unwrap();
	mouse.area.x = window.width();
	mouse.area.y = window.height();

	// check if the cursor is inside the window and get its position
	// then, ask bevy to convert into world coordinates, and truncate to discard Z
	if let Some(world_position) = window
		.cursor_position()
		.and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
		.map(|ray| ray.origin.truncate())
	{
		mouse.position.x = world_position.x;
		mouse.position.y = world_position.y;
	}
}

// Sets the icon on windows and X11
fn set_window_icon(
	windows: NonSend<WinitWindows>,
	primary_window: Query<Entity, With<PrimaryWindow>>,
) {
	let primary_entity = primary_window.single();
	let primary = windows.get_window(primary_entity).unwrap();
	primary.set_cursor_icon(winit::window::CursorIcon::Crosshair);
	let icon_buf =
		Cursor::new(include_bytes!("../build/macos/AppIcon.iconset/icon_256x256.png"));
	if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
		let image = image.into_rgba8();
		let (width, height) = image.dimensions();
		let rgba = image.into_raw();
		let icon = Icon::from_rgba(rgba, width, height).unwrap();
		primary.set_window_icon(Some(icon));
	};
}
