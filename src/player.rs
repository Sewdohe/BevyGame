pub mod player {
    use bevy::prelude::*;
    use bevy_ecs_ldtk::prelude::*;

    use crate::actions::Actions;

    pub struct PlayerPlugin;

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
    pub struct Player;

    #[derive(Default, Bundle, LdtkEntity)]
    pub struct PlayerBundle {
        #[sprite_sheet]
        sprite_sheet: Sprite,
        transform: Transform,
        player: Player,
    }

    fn move_player(
        time: Res<Time>,
        actions: Res<Actions>,
        mut player_query: Query<&mut Transform, With<Player>>,
    ) {
        if actions.player_movement.is_none() {
            return;
        }
        let speed = 150.;
        let movement = Vec3::new(
            actions.player_movement.unwrap().x * speed * time.delta_secs(),
            actions.player_movement.unwrap().y * speed * time.delta_secs(),
            0.,
        );
        for mut player_transform in &mut player_query {
            player_transform.translation += movement;
        }
    }

    impl Plugin for PlayerPlugin {
        fn build(&self, app: &mut App) {
            app
                .add_systems(Update, move_player)
                .register_ldtk_entity::<PlayerBundle>("Player");
        }
    }
}
