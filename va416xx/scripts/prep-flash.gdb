target remote localhost:2331

echo Disabling watchdog\n
set *0x400210C0 = 0x1ACCE551
set *0x40021008 = 0x0

echo Disabling Instruction Memory protection\n
set *0x40010010 = 0x1
