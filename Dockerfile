FROM rust:alpine AS builder
RUN cargo build 
