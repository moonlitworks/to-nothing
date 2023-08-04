FROM rust:bookworm

RUN apt update \
  && apt install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev \
  libwayland-dev libxkbcommon-dev mesa-vulkan-drivers gnome-tweaks

ENV DISPLAY=localhost:0
ENV PULSE_SERVER=unix:/run/user/1000/pulse/native
