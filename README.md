# damascus

![example workflow](https://github.com/obulka/damascus/actions/workflows/rust.yml/badge.svg?event=push)

Damascus is a ray marcher/path tracer with a node-based gui, written entirely in Rust and WGSL.

This is very much still a work in progress, and future breaking changes are all but guaranteed.

![damascus_test](https://github.com/obulka/damascus/assets/21975584/d7e2d5af-1f1d-4943-8857-a3ce69a707f6)

![damascus_testing](https://github.com/obulka/damascus/assets/21975584/d3bd9ea1-a73e-4d58-aff8-3c606db46753)

![noise test](https://github.com/obulka/damascus/assets/21975584/184488b5-954c-41b3-8936-3d827fad7f8c)

### Running the project

Simply clone the repo and run `cargo run --release` to build and launch the application.

### Usage

Right click in the node graph panel to bring up the node selection dialog. Once a node is placed you can middle click and drag to pan over the node graph.

If this is your first time using the application the following is a very minimal scene that you can copy as a starting point. Note that the only value that was changed from the default was the z component of the translation on the camera's axis node. If you do not change this, the camera will be inside of the primitive sphere, while the directional light shines from outside, resulting in nothing but black, which is not particularly exciting.

![minimal](https://github.com/obulka/damascus/assets/21975584/5221d94d-1d9f-47f4-a3cf-2f9b932c889b)

Now try playing with the various parameters, such as the "shape" on the primitive node, and "light_type" on the light node. Hover over the parameter labels to see a tooltip describing the function of the parameter. Then you can also try adding more nodes, such as a material for the primitive, or an axis to move it. Child primitives combined with the various blend types can result in interesting shapes. Have fun!
