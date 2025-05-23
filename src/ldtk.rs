pub mod ldtk_loader {
    use bevy::prelude::*;
    use bevy_ecs_ldtk::prelude::*;

    use crate::phys::components::WallBundle;
    use crate::player::player::Player;
    use crate::player::player::PlayerPlugin;

    pub struct LDTKPlugin;

    const ASPECT_RATIO: f32 = 16. / 9.;
    const CAMERA_SCALE: f32 = 1.0;

    pub fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            Camera2d,
            OrthographicProjection {
                scale: CAMERA_SCALE,
                ..OrthographicProjection::default_2d()
            },
            Transform::from_xyz(1280.0 / 4.0, 720.0 / 4.0, 0.0),
        ));

        commands.spawn(LdtkWorldBundle {
            ldtk_handle: asset_server.load("maps/test_map/test.ldtk").into(),
            ..Default::default()
        });
    }

    #[allow(clippy::type_complexity)]
    pub fn camera_fit_inside_current_level(
        mut camera_query: Query<
            (
                &mut bevy::render::camera::OrthographicProjection,
                &mut Transform,
            ),
            Without<Player>,
        >,
        player_query: Query<&Transform, With<Player>>,
        level_query: Query<
            (&Transform, &LevelIid),
            (Without<OrthographicProjection>, Without<Player>),
        >,
        ldtk_projects: Query<&LdtkProjectHandle>,
        level_selection: Res<LevelSelection>,
        ldtk_project_assets: Res<Assets<LdtkProject>>,
    ) {
        if let Ok(Transform {
            translation: player_translation,
            ..
        }) = player_query.get_single()
        {
            let player_translation = *player_translation;

            let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

            for (level_transform, level_iid) in &level_query {
                let ldtk_project = ldtk_project_assets
                    .get(ldtk_projects.single())
                    .expect("Project should be loaded if level has spawned");

                let level = ldtk_project
                    .get_raw_level_by_iid(&level_iid.to_string())
                    .expect("Spawned level should exist in LDtk project");

                if level_selection.is_match(&LevelIndices::default(), level) {
                    let level_ratio = level.px_wid as f32 / level.px_hei as f32;
                    orthographic_projection.viewport_origin = Vec2::ZERO;
                    if level_ratio > ASPECT_RATIO {
                        // level is wider than the screen
                        let height = (level.px_hei as f32 / 9.).round() * 9.;
                        let width = height * ASPECT_RATIO;
                        orthographic_projection.scaling_mode =
                            bevy::render::camera::ScalingMode::Fixed { width, height };
                        camera_transform.translation.x =
                            (player_translation.x - level_transform.translation.x - width / 2.)
                                .clamp(0., level.px_wid as f32 - width);
                        camera_transform.translation.y = 0.;
                    } else {
                        // level is taller than the screen
                        let width = (level.px_wid as f32 / 16.).round() * 16.;
                        let height = width / ASPECT_RATIO;
                        orthographic_projection.scaling_mode =
                            bevy::render::camera::ScalingMode::Fixed { width, height };
                        camera_transform.translation.y =
                            (player_translation.y - level_transform.translation.y - height / 2.)
                                .clamp(0., level.px_hei as f32 - height);
                        camera_transform.translation.x = 0.;
                    }

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;
                }
            }
        }
    }

    impl Plugin for LDTKPlugin {
        fn build(&self, app: &mut App) {
            app
                .add_plugins(LdtkPlugin)
                .register_ldtk_int_cell::<WallBundle>(1)
                .register_ldtk_int_cell::<WallBundle>(2)
                .add_plugins(PlayerPlugin)
                .insert_resource(LevelSelection::index(0))
                .add_systems(Startup, spawn_level)
                .add_systems(Update, camera_fit_inside_current_level);
        }
    }
}
