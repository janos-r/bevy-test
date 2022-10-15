use bevy::{prelude::*, render::texture::ImageSettings};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy - My testing app ^_^".into(),
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(CurrentWorld(InWorld::W1Main))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(update_transform_from_velocity)
        .add_system(animate_sprite_system_velocity)
        .add_system(switch_world)
        .run();
}

// for binding the player with the camera
// a camera as a child doesn't show a relative scale, nor probably other movement like jumping
#[derive(Component)]
struct MovePlayer;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Default)]
struct Velocity(Vec3);

// For switching between visible worlds
#[derive(Component, PartialEq)]
enum InWorld {
    W1Main,
    W2,
}
struct CurrentWorld(InWorld);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Camera
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MovePlayer)
        .insert(Velocity::default());

    // Player
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.08, true)))
        .insert(MovePlayer)
        .insert(Velocity::default());

    // Sprite
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                custom_size: Some(Vec2::new(200.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., -100., 0.)),
            ..default()
        })
        .insert(InWorld::W1Main);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.1),
                custom_size: Some(Vec2::new(50.0, 200.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(200., 0., 0.)),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(InWorld::W2);

    // Bottom text box
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(200.0)),
                // makes space bellow the box
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                ..default()
            },
            // transparent container
            color: Color::rgba(0.65, 0.65, 0.65, 0.).into(),
            ..default()
        })
        .with_children(|parent| {
            // box size, border thickness and color
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(80.0), Val::Percent(90.0)),
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    color: Color::MIDNIGHT_BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // text background
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexEnd,
                                ..default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn_bundle(
                                TextBundle::from_section(
                                    // "Text Example",
                                    "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularized in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 25.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(10.0)),
                                    size: Size{
                                        // `Val::Percent` doesn't work currently for wrapping
                                        width: Val::Px(1000.),
                                        ..default()
                                    },
                                    ..default()
                                }),
                            );
                        });
                });
        });
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<MovePlayer>>,
    mut current_world: ResMut<CurrentWorld>,
) {
    for mut velocity in &mut query {
        const SPEED: f32 = 5.;

        let default = Vec3::default();
        if velocity.0 != default {
            velocity.0 = default;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            velocity.0 += Vec3::new(-SPEED, 0., 0.);

            // TODO:
            // after collision detection, create doors to change the current world
            // enter W1
            current_world.0 = InWorld::W1Main;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            velocity.0 += Vec3::new(SPEED, 0., 0.);

            // enter W2
            current_world.0 = InWorld::W2;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            velocity.0 += Vec3::new(0., SPEED, 0.);
        }

        if keyboard_input.pressed(KeyCode::Down) {
            velocity.0 += Vec3::new(0., -SPEED, 0.);
        }
    }
}

fn update_transform_from_velocity(
    mut query: Query<(&mut Transform, &Velocity), Changed<Velocity>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0;
    }
}

fn animate_sprite_system_velocity(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &Velocity,
        ),
        Changed<Velocity>,
    >,
) {
    for (mut timer, mut sprite, texture_atlas_handle, velocity) in &mut query {
        timer.tick(time.delta());
        if velocity.0 == Vec3::default() {
            sprite.index = 0;
        } else if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn switch_world(current_world: Res<CurrentWorld>, mut query: Query<(&mut Visibility, &InWorld)>) {
    if current_world.is_changed() {
        for (mut visibility, in_world) in &mut query {
            visibility.is_visible = in_world == &current_world.0
        }
    }
}
