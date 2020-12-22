[![Build Status](https://github.com/dakom/shipyard-scenegraph/workflows/Test%2C%20Build%2C%20and%20Deploy/badge.svg)](https://github.com/dakom/shipyard-scenegraph/actions)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE-APACHE)
[![Crates.io](https://img.shields.io/crates/v/shipyard_scenegraph.svg)](https://crates.io/crates/shipyard_scenegraph)
[![Documentation](https://docs.rs/shipyard_scenegraph/badge.svg)](https://docs.rs/shipyard_scenegraph)
[![Demo](https://img.shields.io/badge/demo-launch-yellow)](https://dakom.github.io/shipyard-scenegraph)

# What is it?
Scenegraph crate for [shipyard ECS](https://github.com/leudz/shipyard)

Builds on and re-exports [shipyard-hierarchy](https://github.com/dakom/shipyard-hierarchy)

# How to use

1. First, decide on a math lib - out of the box is a small native lib, and interop support for `nalgebra`. Enable the appropriate feature (e.g. `native_math`)

2. Use `shipyard_scenegraph::prelude::*` everywhere

3. Call `init()` somewhere around main

4. To add entities to the tree, borrow `SceneGraphStoragesMut` and then call `spawn_child()` on that.

5. To update things - mutably borrow `Translation`, `Rotation`, `Scale`, and `Origin`.

6. Run `trs_to_local` and `local_to_world` systems (like on a tick), and all the Local and World transforms will be propogated.

It is possible to set `LocalTransform` directly. However, doing this will _not_ push the changes in the other direction (e.g. to the TRS components).

# Components and Systems

Components:

* TransformRoot(pub EntityId) - the root node of the tree
* DirtyTransform(pub bool) - tag component for marking dirty transforms
* Translation
* Rotation
* Scale
* Origin
* LocalTransform
* WorldTransform

Systems:

* trs_to_local 
* local_to_world

Custom Views:

* SceneGraphStoragesMut
* TrsStoragesMut

Almost all the above are generic over a container and primitive as type parameters, so that the library is completely agnostic about which math lib, or even precision, you choose to use. However, it's annoying to use that way once you know the concrete types, so aliases are provided for the supported math interop libs (and it's easy to add your own too). Effectively, just decide which math lib you want to use, and the concrete types will be available in the prelude. 

# Run tests

`cargo test --features native_math,nalgebra_math -- --nocapture`

# More math lib interop

A minimal and efficient math library is included via the `native_math` feature, and it's good enough for basic demos - however, it's _very_ minimal with only the operations needed to handle basic transform stuff (matrix multiplication, rotation from quaternion, etc.)

If you're using [nalgebra](https://nalgebra.org/), you can enable the `nalgebra_math` feature to get that as the underlying types.

In both cases, simply import relative `aliases::*` to get all the concrete components and systems 

Other math crates may be added in the future since they only need to satisfy a few simple traits (`SliceExt` + the container trait)

If using multiple math libs within scenegraph for some reason, don't import from the prelude, rather import from the math module directly.

# Extras

Since this builds on [shipyard-hierarchy](https://github.com/dakom/shipyard-hierarchy), the same methods for traversing and updating the hierarchy are available.

Scenegraph gives the Parent and Child components a tag struct `SceneGraph` and shouldn't conflict with any other hierarchies, even in the same world.

# TODO

See issues. Also, ergonomics ;)