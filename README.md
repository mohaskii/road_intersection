# ROAD-INTERSECTION

This project is a traffic control simulation implemented in Rust. The goal is to create a traffic control strategy for an intersection in your capital city and visualize it using a simulation.

## Objectives

The aim of this project is to model traffic control at an intersection, using traffic lights to avoid collisions between vehicles.

## Features

- Modeling of roads and intersection.
- Implementation of traffic lights with toggling logic between red and green.
- Modeling of vehicles with movement and behavior rules.
- User interaction to create vehicles and control the simulation.

## Project Architecture

The project is structured into modules for better code organization:

- **shapes**: Module to manage geometric shapes necessary for building the map.
- **map**: Module to construct the map using geometric shapes defined in `shapes`.
- **traffic_light**: Module to manage traffic lights and their toggling logic.
- **transport**: Module to model vehicles and their behavior in the simulation.
- **driver**: Module to handle user interaction and actions in the simulation.

## Usage

1. Clone the repository: `git clone https://learn.zone01dakar.sn/git/abdouksow/road_intersection.git`
2. Navigate to the project directory: `cd road_intersection`
3. Compile and run the project: `cargo run`

## User Commands

- ↑ : Create a vehicle moving towards the intersection from the south.
- ↓ : Create a vehicle moving towards the intersection from the north.
- → : Create a vehicle moving towards the intersection from the west.
- ← : Create a vehicle moving towards the intersection from the east.
- r : Create a vehicle moving towards the intersection from a random direction.
- Esc : Quit the simulation.

## Dependencies

This project uses the SDL2 library for visualization. For more information on SDL2, see the documentation: [SDL2 Documentation](https://docs.rs/sdl2/0.34.3/sdl2/)

## Contributions

Contributions are welcome! Feel free to open an issue to report a bug or propose a new feature.

## License

This project is licensed under the MIT License. For more information, see the LICENSE file.
