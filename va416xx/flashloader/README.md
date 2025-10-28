VA416xx Flashloader Application
========

This flashloader shows a minimal example for a self-updatable Rust software which exposes
a simple PUS (CCSDS) interface to update the software. It also provides a Python application
called the `image-loader.py` which can be used to upload compiled images to the flashloader
application to write them to the NVM.

Please note that the both the application and the image loader are tailored towards usage
with the [bootloader provided by this repository](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/bootloader).

The software can quickly be adapted to interface with a real primary on-board software instead of
the Python script provided here to upload images because it uses a low-level CCSDS based packet
interface.

## Using the Python image loader

The Python image loader communicates with the Rust flashload application using a dedicated serial
port with a baudrate of 115200.

It is recommended to run the script in a dedicated virtual environment. For example, on UNIX
systems you can use `python3 -m venv venv` and then `source venv/bin/activate` to create
and activate a virtual environment.

After that, you can use

```sh
pip install -r requirements.txt
```

to install all required dependencies.

After that, it is recommended to use `./image-load.py -h` to get an overview of some options.
The flash loader uses the UART0 interface of the VA416xx board to perform CCSDS based
communication. The Python image loader application will search for a file named `loader.toml` and
use the `serial_port` key to determine the serial port to use for serial communication.

### Examples

You can use

```sh
./image-loader.py -p
```

to send a ping an verify the connection.

You can use

```sh
cd flashloader/slot-a-blinky
cargo build --release
cd ../..
./image-loader.py -t a ./slot-a-blinky/target/thumbv7em-none-eabihf/release/slot-a-blinky
```

to build the slot A sample application and upload it to a running flash loader application
to write it to slot A.

You can use

```sh
./image-loader.py -c -t a
```

to corrupt the image A and test that it switches to image B after a failed CRC check instead.
