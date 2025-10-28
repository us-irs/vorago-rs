#!/usr/bin/env python3
from typing import List, Tuple
from spacepackets.ecss.defs import PusService
from spacepackets.ecss.tm import PusTm
import toml
import struct
import logging
import argparse
import time
import enum
from com_interface import ComInterface
from com_interface.serial_base import SerialCfg
from com_interface.serial_cobs import SerialCobsComIF
from crcmod.predefined import PredefinedCrc
from spacepackets.ecss.tc import PusTc
from spacepackets.ecss.pus_verificator import PusVerificator, StatusField
from spacepackets.ecss.pus_1_verification import Service1Tm, UnpackParams
from spacepackets.seqcount import SeqCountProvider
from pathlib import Path
import dataclasses
from elftools.elf.elffile import ELFFile


BAUD_RATE = 115200

BOOTLOADER_START_ADDR = 0x0
BOOTLOADER_END_ADDR = 0x3000
BOOTLOADER_CRC_ADDR = BOOTLOADER_END_ADDR - 2
BOOTLOADER_MAX_SIZE = BOOTLOADER_END_ADDR - BOOTLOADER_START_ADDR - 2

APP_A_START_ADDR = 0x3000
APP_B_END_ADDR = 0x20000 - 8
IMG_SLOT_SIZE = (APP_B_END_ADDR - APP_A_START_ADDR) // 2

APP_A_END_ADDR = APP_A_START_ADDR + IMG_SLOT_SIZE
# The actual size of the image which is relevant for CRC calculation.
APP_A_SIZE_ADDR = APP_A_END_ADDR - 8
APP_A_CRC_ADDR = APP_A_END_ADDR - 4
APP_A_MAX_SIZE = APP_A_END_ADDR - APP_A_START_ADDR - 8

APP_B_START_ADDR = APP_A_END_ADDR
# The actual size of the image which is relevant for CRC calculation.
APP_B_SIZE_ADDR = APP_B_END_ADDR - 8
APP_B_CRC_ADDR = APP_B_END_ADDR - 4
APP_B_MAX_SIZE = APP_A_END_ADDR - APP_A_START_ADDR - 8


CHUNK_SIZE = 400

MEMORY_SERVICE = 6
ACTION_SERVICE = 8

RAW_MEMORY_WRITE_SUBSERVICE = 2
BOOT_NVM_MEMORY_ID = 1
PING_PAYLOAD_SIZE = 0


class ActionId(enum.IntEnum):
    CORRUPT_APP_A = 128
    CORRUPT_APP_B = 129
    SET_BOOT_SLOT = 130


_LOGGER = logging.getLogger(__name__)
SEQ_PROVIDER = SeqCountProvider(bit_width=14)


@dataclasses.dataclass
class LoadableSegment:
    name: str
    offset: int
    size: int
    data: bytes


class Target(enum.Enum):
    BOOTLOADER = 0
    APP_A = 1
    APP_B = 2


class AppSel(enum.IntEnum):
    APP_A = 0
    APP_B = 1


class ImageLoader:
    def __init__(self, com_if: ComInterface, verificator: PusVerificator) -> None:
        self.com_if = com_if
        self.verificator = verificator

    def handle_boot_sel_cmd(self, target: AppSel):
        _LOGGER.info("Sending ping command")
        action_tc = PusTc(
            apid=0x00,
            service=PusService.S8_FUNC_CMD,
            subservice=ActionId.SET_BOOT_SLOT,
            seq_count=SEQ_PROVIDER.get_and_increment(),
            app_data=bytes([target]),
        )
        self.verificator.add_tc(action_tc)
        self.com_if.send(bytes(action_tc.pack()))
        self.await_for_command_copletion("boot image selection command")

    def handle_ping_cmd(self):
        _LOGGER.info("Sending ping command")
        ping_tc = PusTc(
            apid=0x00,
            service=PusService.S17_TEST,
            subservice=1,
            seq_count=SEQ_PROVIDER.get_and_increment(),
            app_data=bytes(PING_PAYLOAD_SIZE),
        )
        self.verificator.add_tc(ping_tc)
        self.com_if.send(bytes(ping_tc.pack()))
        self.await_for_command_copletion("ping command")

    def await_for_command_copletion(self, context: str):
        done = False
        now = time.time()
        while time.time() - now < 2.0:
            if not self.com_if.data_available():
                time.sleep(0.2)
                continue
            for reply in self.com_if.receive():
                result = self.verificator.add_tm(
                    Service1Tm.from_tm(PusTm.unpack(reply, 0), UnpackParams(0))
                )
                if result is not None and result.completed:
                    _LOGGER.info(f"received {context} reply")
                    done = True
            if done:
                break
        if not done:
            _LOGGER.warning(f"no {context} reply received")

    def handle_corruption_cmd(self, target: Target):
        if target == Target.BOOTLOADER:
            _LOGGER.error("can not corrupt bootloader")
        if target == Target.APP_A:
            self.send_tc(
                PusTc(
                    apid=0,
                    service=ACTION_SERVICE,
                    subservice=ActionId.CORRUPT_APP_A,
                ),
            )
        if target == Target.APP_B:
            self.send_tc(
                PusTc(
                    apid=0,
                    service=ACTION_SERVICE,
                    subservice=ActionId.CORRUPT_APP_B,
                ),
            )

    def handle_flash_cmd(self, target: Target, file_path: Path) -> int:
        loadable_segments = []
        _LOGGER.info("Parsing ELF file for loadable sections")
        total_size = 0
        loadable_segments, total_size = create_loadable_segments(target, file_path)
        check_segments(target, total_size)
        print_segments_info(target, loadable_segments, total_size, file_path)
        result = self._perform_flashing_algorithm(loadable_segments)
        if result != 0:
            return result
        self._crc_and_app_size_postprocessing(target, total_size, loadable_segments)
        return 0

    def _perform_flashing_algorithm(
        self,
        loadable_segments: List[LoadableSegment],
    ) -> int:
        # Perform the flashing algorithm.
        for segment in loadable_segments:
            segment_end = segment.offset + segment.size
            current_addr = segment.offset
            pos_in_segment = 0
            while pos_in_segment < segment.size:
                next_chunk_size = min(segment_end - current_addr, CHUNK_SIZE)
                data = segment.data[pos_in_segment : pos_in_segment + next_chunk_size]
                next_packet = pack_memory_write_command(current_addr, data)
                _LOGGER.info(
                    f"Sending memory write command for address {current_addr:#08x} and data with "
                    f"length {len(data)}"
                )
                self.verificator.add_tc(next_packet)
                self.com_if.send(bytes(next_packet.pack()))
                current_addr += next_chunk_size
                pos_in_segment += next_chunk_size
                start_time = time.time()
                while True:
                    if time.time() - start_time > 1.0:
                        _LOGGER.error("Timeout while waiting for reply")
                        return -1
                    data_available = self.com_if.data_available(0.1)
                    done = False
                    if not data_available:
                        continue
                    replies = self.com_if.receive()
                    for reply in replies:
                        tm = PusTm.unpack(reply, 0)
                        if tm.service != 1:
                            continue
                        service_1_tm = Service1Tm.from_tm(tm, UnpackParams(0))
                        check_result = self.verificator.add_tm(service_1_tm)
                        # We could send after we have received the step reply, but that can
                        # somehow lead to overrun errors. I think it's okay to do it like
                        # this as long as the flash loader only uses polling..
                        if (
                            check_result is not None
                            and check_result.status.completed == StatusField.SUCCESS
                        ):
                            done = True

                        # This is an optimized variant, but I think the small delay is not an issue.
                        """
                        if (
                            check_result is not None
                            and check_result.status.step == StatusField.SUCCESS
                            and len(check_result.status.step_list) == 1
                        ):
                            done = True
                        """
                    self.verificator.remove_completed_entries()
                    if done:
                        break
        return 0

    def _crc_and_app_size_postprocessing(
        self,
        target: Target,
        total_size: int,
        loadable_segments: List[LoadableSegment],
    ):
        if target == Target.BOOTLOADER:
            _LOGGER.info("Blanking the bootloader checksum")
            # Blank the checksum. For the bootloader, the bootloader will calculate the
            # checksum itself on the initial run.
            checksum_write_packet = pack_memory_write_command(
                BOOTLOADER_CRC_ADDR, bytes([0x00, 0x00])
            )
            self.send_tc(checksum_write_packet)
        else:
            crc_addr = None
            size_addr = None
            if target == Target.APP_A:
                crc_addr = APP_A_CRC_ADDR
                size_addr = APP_A_SIZE_ADDR
            elif target == Target.APP_B:
                crc_addr = APP_B_CRC_ADDR
                size_addr = APP_B_SIZE_ADDR
            assert crc_addr is not None
            assert size_addr is not None
            _LOGGER.info(f"Writing app size {total_size} at address {size_addr:#08x}")
            size_write_packet = pack_memory_write_command(
                size_addr, struct.pack("!I", total_size)
            )
            self.com_if.send(bytes(size_write_packet.pack()))
            time.sleep(0.2)
            crc_calc = PredefinedCrc("crc-ccitt-false")
            for segment in loadable_segments:
                crc_calc.update(segment.data)
            checksum = crc_calc.digest()
            _LOGGER.info(
                f"Writing checksum 0x[{checksum.hex(sep=',')}] at address {crc_addr:#08x}"
            )
            self.send_tc(pack_memory_write_command(crc_addr, checksum))

    def send_tc(self, tc: PusTc):
        self.com_if.send(bytes(tc.pack()))


def main() -> int:
    print("Python VA108XX Image Loader Application")
    logging.basicConfig(
        format="[%(asctime)s] [%(levelname)s] %(message)s", level=logging.DEBUG
    )
    parser = argparse.ArgumentParser(
        prog="image-loader", description="Python VA416XX Image Loader Application"
    )
    parser.add_argument("-p", "--ping", action="store_true", help="Send ping command")
    parser.add_argument(
        "-s", "--sel", choices=["a", "b"], help="Set boot slot (Slot A or B)"
    )
    parser.add_argument("-c", "--corrupt", action="store_true", help="Corrupt a target")
    parser.add_argument(
        "-t",
        "--target",
        choices=["bl", "a", "b"],
        help="Target (Bootloader or slot A or B)",
    )
    parser.add_argument(
        "path", nargs="?", default=None, help="Path to the App to flash"
    )
    args = parser.parse_args()
    serial_port = None
    if Path("loader.toml").exists():
        with open("loader.toml", "r") as toml_file:
            parsed_toml = toml.loads(toml_file.read())
            if "serial_port" in parsed_toml:
                serial_port = parsed_toml["serial_port"]
    if serial_port is None:
        serial_port = input("Please specify the serial port manually: ")
    serial_cfg = SerialCfg(
        com_if_id="ser_cobs",
        serial_port=serial_port,
        baud_rate=BAUD_RATE,
        polling_frequency=0.1,
    )
    verificator = PusVerificator()
    com_if = SerialCobsComIF(serial_cfg)
    com_if.open()
    target = None
    if args.target == "bl":
        target = Target.BOOTLOADER
    elif args.target == "a":
        target = Target.APP_A
    elif args.target == "b":
        target = Target.APP_B

    boot_sel = None
    if args.sel:
        if args.sel == "a":
            boot_sel = AppSel.APP_A
        elif args.sel == "b":
            boot_sel = AppSel.APP_B

    image_loader = ImageLoader(com_if, verificator)
    file_path = None
    result = -1
    if args.ping:
        image_loader.handle_ping_cmd()
        com_if.close()
        return 0
    if args.sel and boot_sel is not None:
        image_loader.handle_boot_sel_cmd(boot_sel)
    if target:
        if not args.corrupt:
            if not args.path:
                _LOGGER.error("App Path needs to be specified for the flash process")
            file_path = Path(args.path)
            if not file_path.exists():
                _LOGGER.error("File does not exist")
    if args.corrupt:
        if not target:
            _LOGGER.error("target for corruption command required")
            com_if.close()
            return -1
        image_loader.handle_corruption_cmd(target)
    else:
        if file_path is not None:
            assert target is not None
            result = image_loader.handle_flash_cmd(target, file_path)

    com_if.close()
    return result


def create_loadable_segments(
    target: Target, file_path: Path
) -> Tuple[List[LoadableSegment], int]:
    loadable_segments = []
    total_size = 0
    with open(file_path, "rb") as app_file:
        elf_file = ELFFile(app_file)

        for idx, segment in enumerate(elf_file.iter_segments("PT_LOAD")):
            if segment.header.p_filesz == 0:
                continue
            # Basic validity checks of the base addresses.
            if idx == 0:
                if (
                    target == Target.BOOTLOADER
                    and segment.header.p_paddr != BOOTLOADER_START_ADDR
                ):
                    raise ValueError(
                        f"detected possibly invalid start address {segment.header.p_paddr:#08x} for "
                        f"bootloader, expected {BOOTLOADER_START_ADDR}"
                    )
                if (
                    target == Target.APP_A
                    and segment.header.p_paddr != APP_A_START_ADDR
                ):
                    raise ValueError(
                        f"detected possibly invalid start address {segment.header.p_paddr:#08x} for "
                        f"App A, expected {APP_A_START_ADDR}"
                    )
                if (
                    target == Target.APP_B
                    and segment.header.p_paddr != APP_B_START_ADDR
                ):
                    raise ValueError(
                        f"detected possibly invalid start address {segment.header.p_paddr:#08x} for "
                        f"App B, expected {APP_B_START_ADDR}"
                    )
            name = None
            for section in elf_file.iter_sections():
                if (
                    section.header.sh_offset == segment.header.p_offset
                    and section.header.sh_size > 0
                ):
                    name = section.name
            if name is None:
                _LOGGER.warning("no fitting section found for segment")
                continue
            # print(f"Segment Addr: {segment.header.p_paddr}")
            # print(f"Segment Offset: {segment.header.p_offset}")
            # print(f"Segment Filesize: {segment.header.p_filesz}")
            loadable_segments.append(
                LoadableSegment(
                    name=name,
                    offset=segment.header.p_paddr,
                    size=segment.header.p_filesz,
                    data=segment.data(),
                )
            )
            total_size += segment.header.p_filesz
    return loadable_segments, total_size


def check_segments(
    target: Target,
    total_size: int,
):
    # Set context string and perform basic sanity checks.
    if target == Target.BOOTLOADER and total_size > BOOTLOADER_MAX_SIZE:
        raise ValueError(
            f"provided bootloader app larger than allowed {total_size} bytes"
        )
    elif target == Target.APP_A and total_size > APP_A_MAX_SIZE:
        raise ValueError(f"provided App A larger than allowed {total_size} bytes")
    elif target == Target.APP_B and total_size > APP_B_MAX_SIZE:
        raise ValueError(f"provided App B larger than allowed {total_size} bytes")


def print_segments_info(
    target: Target,
    loadable_segments: List[LoadableSegment],
    total_size: int,
    file_path: Path,
):
    # Set context string and perform basic sanity checks.
    if target == Target.BOOTLOADER:
        context_str = "Bootloader"
    elif target == Target.APP_A:
        context_str = "App Slot A"
    elif target == Target.APP_B:
        context_str = "App Slot B"
    _LOGGER.info(f"Flashing {context_str} with image {file_path} (size {total_size})")
    for idx, segment in enumerate(loadable_segments):
        _LOGGER.info(
            f"Loadable section {idx} {segment.name} with offset {segment.offset:#08x} and "
            f"size {segment.size}"
        )


def pack_memory_write_command(addr: int, data: bytes) -> PusTc:
    app_data = bytearray()
    app_data.append(BOOT_NVM_MEMORY_ID)
    # N parameter is always 1 here.
    app_data.append(1)
    app_data.extend(struct.pack("!I", addr))
    app_data.extend(struct.pack("!I", len(data)))
    app_data.extend(data)
    return PusTc(
        apid=0,
        service=MEMORY_SERVICE,
        subservice=RAW_MEMORY_WRITE_SUBSERVICE,
        seq_count=SEQ_PROVIDER.get_and_increment(),
        app_data=bytes(app_data),
    )


if __name__ == "__main__":
    main()
