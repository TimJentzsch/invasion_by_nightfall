use bevy::prelude::*;

use crate::core::{CoreSystemSet, Resources};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, UiSystemSet.after(CoreSystemSet))
            .configure_sets(Update, UiSystemSet.after(CoreSystemSet))
            .add_systems(Startup, setup.in_set(UiSystemSet))
            .add_systems(Update, update_coins.in_set(UiSystemSet));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct UiSystemSet;

#[derive(Debug, Component)]
struct CoinText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/fira_sans/FiraSans-Medium.ttf");
    let style = TextStyle {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };

    commands
        .spawn(NodeBundle { ..default() })
        .with_children(|child| {
            child.spawn((
                CoinText,
                TextBundle::from_sections([
                    TextSection::new("Coins: ", style.clone()),
                    TextSection::new("0", style.clone()),
                ]),
            ));
        });
}

fn update_coins(mut query: Query<&mut Text, With<CoinText>>, resources: Res<Resources>) {
    let mut text = query.single_mut();
    text.sections[1].value = format!("{:.0}", resources.coins);
}
