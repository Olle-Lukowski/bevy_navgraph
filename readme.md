# bevy_navgraph

`bevy_navgraph` is a Rust crate that provides navigation graph functionality for the Bevy game engine, using the Bevy and Bevy Rapier3D crates.

## Usage

To use `bevy_navgraph`, you must have the following dependencies in your `Cargo.toml` file:

```toml
[dependencies]
bevy = "0.10.1"
bevy_rapier3d = "0.21.0"
bevy_navgraph = "0.0.1"
```

Also, please make sure the Rapier plugin is added to your app.

```Rust
use bevy::prelude::*;
use bevy_navgraph::NavGraphPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(NavGraphPlugin { starting_position: Vec3::new(0.0, 0.0, 0.0),   max_bounces: 5, splits_per_bounce: 5})
        .run();
}
```

For `bevy_navgraph` to work, you need to have colliders in your world.
