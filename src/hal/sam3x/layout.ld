_data_load = LOADADDR(.data);

INCLUDE iomem.ld

ENTRY(main)

MEMORY
{
	rom (rx)    : ORIGIN = 0x00080000, LENGTH = 0x00080000 /* Flash, 512K */
	sram0 (rwx) : ORIGIN = 0x20000000, LENGTH = 0x00010000 /* sram0, 64K */
	sram1 (rwx) : ORIGIN = 0x20080000, LENGTH = 0x00008000 /* sram1, 32K */
	ram (rwx)   : ORIGIN = 0x20070000, LENGTH = 0x00018000 /* sram, 96K */
}

__STACK_BASE = ORIGIN(ram) + LENGTH(ram);

REGION_ALIAS("vectors", rom);

INCLUDE layout_common.ld
