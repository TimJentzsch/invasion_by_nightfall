use bevy::prelude::*;

use crate::core::{inventory::Inventory, CoreSystemSet};

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
    let header_style = TextStyle {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };
    let footer_style = TextStyle {
        font: font.clone(),
        font_size: 30.0,
        ..default()
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|child| {
            // Header bar
            child
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        padding: UiRect::all(Val::Px(10.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|child| {
                    child.spawn((
                        CoinText,
                        TextBundle::from_sections([
                            TextSection::new("0", header_style.clone()),
                            TextSection::new(" G", header_style.clone()),
                        ]),
                    ));
                });

            // Bottom bar
            child
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        padding: UiRect::all(Val::Px(10.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|child| {
                    child.spawn((TextBundle::from_sections([
                        TextSection::new("[Q] Farmer (", footer_style.clone()),
                        TextSection::new("100", footer_style.clone()),
                        TextSection::new(" G)", footer_style.clone()),
                    ]),));
                });
        });
}

fn update_coins(mut query: Query<&mut Text, With<CoinText>>, inventory: Res<Inventory>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("{:.0}", inventory.coins);
}