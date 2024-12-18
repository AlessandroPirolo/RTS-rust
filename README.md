# RTS-Rust

An hard Real-Time System (RTS) implementation in Rust for embedded development on the STM32F407VGT microcontroller using the `stm32f4xx_hal`. This repository showcases real-time scheduling techniques, and task management in Rust on embedded hardware.
This project is a **reproduction of the [SRT repository](https://github.com/jiayihu/SRT)**, originally written in Ada, but implemented in Rust to evaluate how it behaves in a real-time system and whether it is suited for such environments. Unlike the original, this implementation does not use the MAST analysis tool for scheduling analysis.

## Features

- **Real-Time Scheduling**: Implements real-time scheduling algorithms.
- **Task Management**: Efficient task handling for time-critical embedded applications.
- **Hardware Abstraction Layer**: Uses `stm32f4xx_hal` to interface with STM32 peripherals.
- **Rust**: Powered by Rust for safe and performant embedded development.

## Getting Started

### Prerequisites

- **Hardware**: STM32F407VGT microcontroller
- **Software**:
  - [Rust](https://www.rust-lang.org/)
  - [STM32F4xx HAL](https://docs.rs/stm32f4xx-hal/latest/stm32f4xx_hal/)
  - [RTIC](https://rtic.rs/2/book/en/)
  - [Cargo-Embed](https://github.com/knurling-rs/cargo-embed)

### Installation

1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add thumbv7em-none-eabihf
   ```

2. Clone the repository:
   ```bash
   git clone https://github.com/AlessandroPirolo/RTS-rust.git
   cd RTS-rust
   ```

3. Install dependencies:
   ```bash
   cargo install cargo-embed
   ```

4. Build the project:
   ```bash
   cargo build --release
   ```

5. Run the binary to the STM32:
   ```bash
   cargo run
   ```

### Usage

Once the binary is run, the real-time system will be up and running on the STM32F407VGT.

### Configuration

Modify `Cargo.toml` to add or remove dependencies as required. You can also configure your real-time scheduling algorithms or peripherals within the source code.

## License

This project is licensed under the GNU AGP Licence - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [SRT](https://github.com/jiayihu/SRT)
- [STM32F4xx HAL](https://docs.rs/stm32f4xx-hal/latest/stm32f4xx_hal/)
