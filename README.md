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

1. First, decide on your math container types - out of the box is a small native lib, and interop support for a few `nalgebra` f64 primitives. Enable the appropriate feature (e.g. `native_math` or `nalgebra_math`).

2. Use `shipyard_scenegraph::prelude::*` everywhere

3. Call `init()` somewhere around main

4. To add entities to the tree, borrow `SceneGraphStoragesMut` and then call `spawn_child_*()` on that.

5a. To update things - mutably borrow `Translation`, `Rotation`, `Scale`, and `Origin`. There's a helper view for updating single entities too (`TrsStoragesMut`).
5b. Alternatively - work with LocalTransform directly (but note that it currently does not backpropogate. see https://github.com/dakom/shipyard-scenegraph/issues/22) 

6. Run `local_transform_sys` and `world_transform_sys` systems (like on a tick), and all the Local and World transforms will be propogated.


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

* local_transform_sys 
* world_transform_sys 

Custom Views:

* SceneGraphStoragesMut
* TrsStoragesMut

Almost all the above are generic over a container and primitive as type parameters, so that the library is completely agnostic about which math lib, or even precision, you choose to use. However, it's annoying to use that way once you know the concrete types, so aliases are provided for the supported math interop libs (and it's easy to add your own too). Effectively, just decide which math lib you want to use, and the concrete types will be available in the prelude. 

# Run tests

Core tests: `cargo test --features native_math -- --nocapture`
Nalgebra compat tests: `cargo test --features nalgebra_math -- --nocapture`

# More math lib interop

A minimal and efficient math library is included via the `native_math` feature, and it's good enough for little demos - however, it's _very_ minimal with only the operations needed to handle basic transform stuff (matrix multiplication, rotation from quaternion, etc.)

If you're using [nalgebra](https://nalgebra.org/), you can enable the `nalgebra_math` feature to get that as the underlying types.

Other libraries can be added very easily.

If using multiple math libs within scenegraph for some reason, don't import from the prelude, rather import from the math module directly (this is very uncommon - not even done in tests)

# Extras

Since this builds on [shipyard-hierarchy](https://github.com/dakom/shipyard-hierarchy), the same methods for traversing and updating the hierarchy are available.

Scenegraph gives the Parent and Child components a tag struct `SceneGraph` and shouldn't conflict with any other hierarchies, even in the same world.