/* Bootloader occupies the first two 16 KB sectors of STM32F401RE flash.
 * Must never exceed 32 KB or it overlaps the application at 0x0800_8000.
 */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 32K
  RAM   : ORIGIN = 0x20000000, LENGTH = 96K
}
