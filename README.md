# damascus

![workflow](https://github.com/obulka/damascus/actions/workflows/rust.yml/badge.svg?event=push)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Damascus is a ray marcher/path tracer with a node-based gui, written entirely in Rust and WGSL.

This is very much still a work in progress, and future breaking changes are all but guaranteed.

![damascus](https://github.com/obulka/damascus/assets/21975584/9b8b2996-f842-4704-906e-4dba9c68208e)

![mandelbox_0](https://github.com/obulka/damascus/assets/21975584/84f1a6b1-6a42-42ea-9881-e98c46b41a36)

![damascus_testing](https://github.com/obulka/damascus/assets/21975584/d3bd9ea1-a73e-4d58-aff8-3c606db46753)

### Running the project

Simply clone the repo and run `cargo run --release` to build and launch the application.

#### Ubuntu

If you haven't already, install rust:

```
sudo apt install curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

The following are required packages:

```
sudo apt install build-essential
sudo apt install cmake
sudo apt install libglib2.0-dev
sudo apt install libghc-gi-atk-dev
sudo apt install libgtk-3-dev
```

### Usage

If this is your first time using the application, you can go to `file->load` in the toolbar and load the scene at `assets/basic_scene.dam`. Click `Set active` on the `ray marcher` node to view the scene. You can middle click and drag to pan over the node graph, and scroll to zoom in and out. Click and drag on the background to box-select nodes, and you can hold down `shift` to expand the current selection. You can then drag the selected nodes around. The `delete` key will remove all selected nodes. `ctrl+c` to copy the selected nodes, and `ctrl+v` to paste. `ctrl+n` will clear the scene. If any node prior to the `scene` node is selected you can pan the camera by left/middle clicking and dragging, and rotate the camera with right click and drag. You can scroll the mouse wheel over the viewer to move the camera in and out. 

Now try playing with the various parameters, such as the `shape` on the primitive node, and `light_type` on the light node. Hover over the parameter labels to see a tooltip describing the function of the parameter. Then you can also try adding more nodes, such as a new `primitive`. Right click in the node graph panel to bring up the node selection dialog. Plug the new `primitive` into the existing one's `children` input and use the various `blend_type`s to create interesting composite shapes.
