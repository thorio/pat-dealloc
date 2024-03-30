#include <linux/module.h>
#include <linux/printk.h>

int init_module(void)
{
	pr_info("helo from kernel\n");

	return 0;
}

void cleanup_module(void)
{

}

MODULE_LICENSE("GPL");
