<p align="center">
<img src="assets/cloud_rider_banner.png" alt="Logo"/>
</p>

# Cloud Rider

Cloud Rider is a proof-of-concept (PoC) Ground Control Station (GCS) built with a modern tech stack:

- 🚀 Rust backend for performance and safety
- 💻 React frontend for an interactive user interface
- 🔄 WebSocket-based real-time communication between frontend and backend


It enables seamless streaming of telemetry and command data, making it ideal for UAV control, drone simulations, or robotic applications.

## Cloud Rider Frontend

<img src="assets/cloud_rider_ui.png" alt="Logo"/>

<video src="assets/cloud_rider_tracking_mode.mp4" controls="controls" style="max-width: 730px;">
</video>

## Cloud Rider Backend

<img src="assets/high-level-server.drawio.png" alt="Logo"/>



## Feature

- [x] Simple Websocket Server written in Rust which provides telemetry data from PX4 using Mavlink protocol
- [ ] Simple React frontend consuming telemetry data and display them
- [ ] Enable sending commands from Frontend
- [ ] Advanced session management (Graceful shutdown)
- [ ] User authentication (advanced session management)
- [ ] Display drone position in a map
- [ ] Advanced mission planning capabilities
