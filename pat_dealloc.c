#include <linux/module.h>
#include <asm/io.h>

MODULE_LICENSE("GPL");

int init_module(void)
{
	// arch_io_free_memtype_wc(0x000000f81c680000, 0x000000f81c681000 - 0x000000f81c680000);
	return 0;
}

void cleanup_module(void) {

}
