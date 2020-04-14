[![Build Status](https://github.com/dakom/shipyard-scenegraph/workflows/Test%2C%20Build%2C%20and%20Deploy/badge.svg)](https://github.com/dakom/shipyard-scenegraph/actions)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE-APACHE)
[![Crates.io](https://img.shields.io/crates/v/shipyard_scenegraph.svg)](https://crates.io/crates/shipyard_scenegraph)
[![Documentation](https://docs.rs/shipyard_scenegraph/badge.svg)](https://docs.rs/shipyard_scenegraph)
[![Demo](https://img.shields.io/badge/demo-launch-yellow)](https://dakom.github.io/shipyard-scenegraph)

# What is it?
Scenegraph crate for [shipyard ECS](https://github.com/leudz/shipyard)

Builds on and re-exports [shipyard-hierarchy](https://github.com/dakom/shipyard-hierarchy)

# Components and Systems

See [components.rs](crate/src/components.rs) for the full list of exported components. They are mainly just thin wrappers around math primitives. [systems.rs](crate/src/systems.rs) has the exported systems (these are not exported on the root, but rather as `systems::*`).

The way it all fits together is that `Translation`, `Rotation`, and `Scale` are in an "update pack". This allows for changes to these to be propagated to `LocalTransform` automatically and efficiently when the `TrsToLocal` system is run. 

Similarly, the changes to `LocalTransform` are efficiently propagated to `WorldTransform` when the `LocalToWorld` system is run. Currently this does not go through update pack and rather uses a separate `DirtyTransform` component, [though that may change soon](https://github.com/dakom/shipyard-scenegraph/issues/19)

It is possible to set `LocalTransform` directly. However, doing this will _not_ push the changes in the other direction (e.g. to the TRS components).

# Math lib interop

A minimal and efficient math library is included out of the box and there's no need to depend on anything else. However, it's _very_ minimal with only the operations needed to handle basic transform stuff (matrix multiplication, rotation from quaternion, etc.)

If you're using [nalgebra](https://nalgebra.org/), you can convert the math structs via `.into()` without doing anything special. However this comes at the cost of an allocation, so you may want to use the `nalgebra` structs directly with no conversion (but you do pay the cost of the heavier math lib, of course). To do that, simply enable the `nalgebra_transforms` feature here (this will use nalgebra _instead of_ the builtins)

Other math crates may be added in the future fairly easily since there are really only two steps:

1. Satisfy the [traits](crate/src/math/traits.rs)
2. Make type aliases for `Matrix4`, `Vec3`, and `Quat`

Currently it's assumed that all the math is done on `f64`'s, with helper methods to write to `&[f32]`'s when needed.

# Usage

See code in [test_transform.rs](crate/tests/test_transform.rs) as well as the [example](example), but it's essentially like this:

1. Get a `shipyard` world (or create a new one)

2. Call [init()](https://docs.rs/shipyard_scenegraph/latest/shipyard_scenegraph/fn.init.html)

3. Spawn children via [spawn_child()](https://docs.rs/shipyard_scenegraph/latest/shipyard_scenegraph/fn.spawn_child.html) (either via the borrowed storages or the world helper)

4. Update Translation, Rotation, and Scale however you want. For example, on a given entity `hero`:
```
let mut translation_storage = world.borrow::<&mut Translation>();
let translation = (&mut translation_storage).get(hero).unwrap();
translation.0.y = 200.0;
```

5. Run the systems `TrsToLocal` and `LocalToWorld`

# Extras

Since this builds on [shipyard-hierarchy](https://github.com/dakom/shipyard-hierarchy), the same methods for traversing and updating the hierarchy are available.

In general, methods are on the storages. They can be iterated and worked with more efficiently since the borrow only occurs once. Helpers like [set_trs()](https://docs.rs/shipyard_scenegraph/latest/shipyard_scenegraph/fn.set_trs.html) and [spawn_child()](https://docs.rs/shipyard_scenegraph/latest/shipyard_scenegraph/fn.spawn_child.html) that operate on the world, are really just for the sake of convenience and "one-offs".

# TODO

See issues. Also, ergonomics ;)