use bevy::prelude::*;

use crate::core::{game_state::GameState, CoreSystemSet, GameStats, Winner};

use super::UiSystemSet;

pub struct PostGameUiPlugin;

impl Plugin for PostGameUiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            OnEnter(GameState::PostGame),
            UiSystemSet.after(CoreSystemSet),
        )
        .add_systems(OnEnter(GameState::PostGame), spawn.in_set(UiSystemSet))
        .add_systems(OnExit(GameState::PostGame), despawn.in_set(UiSystemSet));
    }
}

#[derive(Debug, Component)]
struct PostGameUi;

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, stats: Res<GameStats>) {
    let font = asset_server.load("fonts/fira_sans/FiraSans-Medium.ttf");
    let header_style = TextStyle {
        font: font.clone(),
        font_size: 150.0,
        ..default()
    };

    let text = match stats.winner {
        Winner::Player => "You won!",
        Winner::Enemy => "You lost!",
    };

    commands
        .spawn((
            PostGameUi,
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|child| {
            // Header bar
            child.spawn(TextBundle::from_section(text, header_style.clone()));
        });
}

fn despawn(mut commands: Commands, in_game_ui_query: Query<Entity, With<PostGameUi>>) {
    for in_game_ui in in_game_ui_query.iter() {
        commands.entity(in_game_ui).despawn_recursive();
    }
}
