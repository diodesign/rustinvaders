# Rustinvaders

This is a simple 3D game written in Rust, and inspired by the arcade classic _Space Invaders_. This was created purely for fun while taking my first steps in graphics and games programming. It uses the extremely handy [Kiss3D](http://kiss3d.org/) engine.

## Building

I recommend building this in an official [Debian-based Rust Docker container](https://hub.docker.com/_/rust/) with the following extra packages installed: `xorg-dev libglu1-mesa-dev`

Make sure you have the above in place, checkout this project's code from GitHub, compile, and run using:
```
git clone https://github.com/diodesign/rustinvaders.git
cd rustinvaders
cargo run --release
```

## Playing

Press `z` to move to the left, `x` to go right, `Return` to fire. You can only have one bullet on screen at a time: that's a deliberate restriction to keep the gameplay faithful to the original. The aliens also drop bombs, and speed up as you destroy more of them – again, like the original. If you're hit by an alien or one of their bombs, you'll lose a life. You have three lives until it's game over. If the aliens manage to get below your ship, it's immediately game over.

## Feedback

This is a work in progrss - there are many little things to add and improve. If you have any suggestions, patches, complaints, etc, then submit an issue or pull request, or [try emailing me](mailto:diodesign@gmail.com). Cheers for taking an interesting.
