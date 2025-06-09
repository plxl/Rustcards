//! This example shows how to create a node with a shadow

use argh::FromArgs;
use bevy::color::palettes::css::DEEP_SKY_BLUE;
use bevy::color::palettes::css::RED;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::input::keyboard::Key;
use bevy::input::InputPlugin;
use bevy::math::vec3;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::sprite::Material2d;
use bevy::sprite::Material2dPlugin;
use bevy::{prelude::*};
use bevy::window::WindowMode;
use bevy::winit::WinitSettings;
use bevy::window::{Window, PrimaryWindow};
use ron::de::Position;
use std::fs;

#[derive(FromArgs, Resource)]
/// `box_shadow` example
struct Args {
    /// number of samples
    #[argh(option, default = "4")]
    samples: u32,
}

#[derive(Component)]
struct Card;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rustcards".to_string(), // Window title
                    resolution: (780.0, 980.0).into(), // Set the resolution
                    // resizable: false, // Lock the window size
                    ..Default::default()
                }),
                ..Default::default()
            }),
            FrameTimeDiagnosticsPlugin,
            // Material2dPlugin::<CustomMaterial>::default(),
        ))
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        // .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, (setup, adjust_window_size))
        .add_systems(Update, (fps_update, fps_update_color, handle_card_hover))
        .run();
}

fn adjust_window_size(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    // if let Ok(mut window) = windows.get_single_mut() {
    //     window.mode = WindowMode::Fullscreen(MonitorSelection::Current);
    // }
}

// struct CustomShader {
//     shader: Handle<Shader>,
// }

// #[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
// struct CustomMaterial {
//     // #[texture(0)]
//     // #[sampler(1)]
//     #[uniform(2)]
//     u_shadow_params: ShadowParams,
// }

// #[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
// struct ShadowParams {
//     shadow_offset: Vec2,
//     shadow_color: Vec4,
//     shadow_opacity: f32,
//     shadow_spread: f32,
//     shadow_blur: f32,
// }

// impl Material2d for CustomMaterial {
//     fn fragment_shader() -> ShaderRef {
//         "shaders/drop_shadow.wgsl".into()
//     }
// }

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {:?}", current_dir);

    match fs::read_dir(current_dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(file) => println!("File: {:?}", file.path()),
                    Err(err) => eprintln!("Error reading entry: {:?}", err),
                }
            }
        }
        Err(err) => eprintln!("Error reading directory: {:?}", err),
    }


    // shaders
    // let shadow_shader_handle = asset_server.load("shaders/shadow_shader.wgsl");
    // shaders.insert(
    //     &CustomShader,
    //     Shader::from_wgsl(shadow_shader_handle, file!()),
    // );

    
    

    // Load font
    let font_pikmin: Handle<Font> = asset_server.load("fonts/dfcraftsumistdw9.otf");
    // Load textures
    let img_board_a: Handle<Image> = asset_server.load("images/BoardA.png");
    // Get all image file paths in the folder
    let folder_path = "images/cards/";
    let cards = fs::read_dir(format!("assets/{}", folder_path))
        .expect("Failed to read images folder")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()?.to_str()? == "png" || path.extension()?.to_str()? == "jpg" {
                let filename = format!(
                    "{}/{}",
                    folder_path,
                    path.file_name()?.to_str()?
                );
                Some(asset_server.load(filename))
            } else {
                None
            }
        })
        .collect::<Vec<Handle<Image>>>();
    
    

    // ui camera
    commands.spawn((Camera2d, UiBoxShadowSamples(4)));
    
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(80.),
            padding: UiRect::all(Val::Px(20.)),
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        BackgroundColor(DEEP_SKY_BLUE.into()),
        ZIndex(-1),
    )).with_child(
        (
            ImageNode {
                image: img_board_a,
                ..default()
            },
            BorderRadius::all(Val::Percent(2.)),
            BoxShadow {
                color: Color::BLACK,
                x_offset: Val::Percent(0.),
                y_offset: Val::Percent(0.),
                spread_radius: Val::Percent(5.),
                blur_radius: Val::Px(12.),
            },
        )
    );

    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(20.),
            flex_wrap: FlexWrap::Wrap,
            top: Val::Percent(80.),
            // justify_self: JustifySelf::Center,
            ..default()
        },
        BackgroundColor(RED.into()),
        ZIndex(0),
    )).with_children(|commands| {
        commands.spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::horizontal(Val::Px(20.0)),
            justify_content: JustifyContent::SpaceAround,
            column_gap: Val::Percent(-20.),
            // justify_content: JustifyContent::SpaceAround,
            ..default()
        }).with_children(|commands| {
            for i in 0..5 {
                commands.spawn((
                    ImageNode {
                        image: cards[i].clone(),
                        ..default()
                    },
                    BorderRadius::all(Val::Percent(6.)),
                    Card,
                    // BoxShadow {
                    //     color: DEEP_SKY_BLUE.into(),
                    //     x_offset: Val::Percent(0.),
                    //     y_offset: Val::Percent(0.),
                    //     spread_radius: Val::Px(20.),
                    //     blur_radius: Val::Px(12.),
                    // },
                ));
                // .with_child((
                //     ImageNode {
                //         image: cards[i].clone(),
                //         ..default()
                //     },
                //     BorderRadius::all(Val::Percent(6.)),
                //     BoxShadow {
                //         color: Color::WHITE,
                //         x_offset: Val::Percent(0.),
                //         y_offset: Val::Percent(0.),
                //         spread_radius: Val::Px(4.),
                //         blur_radius: Val::Px(4.),
                //     },
                // ));
            }
        });
    });

    // let mutcustom_materials: ResMut<Assets<CustomMaterial>>;
    commands.spawn((
        // Mesh2d(),
        // MeshMaterial2d(materials.add(CustomMaterial {
        //     u_shadow_params: ShadowParams {
        //         shadow_offset: Vec2 { x: 0.0, y: 0.0 },
        //         shadow_color: Vec4::splat(1.0),
        //         shadow_opacity: 1.0,
        //         shadow_spread: 6.0,
        //         shadow_blur: 10.0,
        //     }
        // })),
        Text::new("FPS: "),
        TextFont {
            font: font_pikmin.clone(),
            font_size: 20.,
            ..default()
        },
        // BoxShadow {
        //     color: Color::BLACK,
        //     x_offset: Val::Percent(0.),
        //     y_offset: Val::Percent(0.),
        //     spread_radius: Val::Px(10.0),
        //     blur_radius: Val::Px(6.),
        //     ..default()
        // },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.),
            left: Val::Px(10.),
            ..default()
        },
        ZIndex(1),
    )).with_child((
        TextSpan::default(),
        (
            TextFont {
                font: font_pikmin.clone(),
                font_size: 20.,
                ..default()
            },
            // TextColor(DEEP_SKY_BLUE.into()),
        ),
        FpsText,
    ));

}

fn handle_card_hover(
    mut query: Query<(&Interaction, &mut Transform, &mut BoxShadow), (Changed<Interaction>, With<Card>)>,
) {
    for (interaction, mut transform, mut box_shadow) in query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                // On hover, increase the size and change the shadow color
                transform.scale = Vec3::new(1.1,1.1,1.0);
                box_shadow.blur_radius = Val::Px(4.0);
                box_shadow.spread_radius = Val::Px(10.0);
                println!("On hover!");
            }
            Interaction::None => {
                // Revert to original size and color when not hovered
                transform.scale = Vec3::new(1.0,1.0,1.0);
                box_shadow.blur_radius = Val::Px(0.0);
                box_shadow.spread_radius = Val::Px(0.0);
                println!("no hover!");
            }
            _ => (),
        }
    }
}





#[derive(Component)]
struct FpsText;
fn fps_update(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut span in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **span = format!("{value:.0}");
            }
        }
    }
}

fn fps_update_color(
    time: Res<Time>,
    mut query: Query<&mut TextColor, With<FpsText>>,
) {
    for mut text_color in &mut query {
        let seconds = time.elapsed_secs();

        text_color.0 = Color::srgb(
            ops::sin(1.25 * seconds) / 2. + 0.5,
            ops::sin(0.75 * seconds) / 2. + 0.5,
            ops::sin(0.50 * seconds) / 2. + 0.5,
        );
    }
}