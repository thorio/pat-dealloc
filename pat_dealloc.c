#include <linux/cdev.h>
#include <linux/device.h>
#include <linux/fs.h>
#include <linux/init.h>
#include <linux/kernel.h>
#include <linux/module.h>
#include <linux/printk.h>
#include <linux/types.h>
#include <asm/io.h>

#include <asm/errno.h>

#define DEVICE_NAME "pat_dealloc"
#define ADDR_STR_SIZE 18
#define ADDR_STR_BASE 16

static int device_open(struct inode *inode, struct file *file) {
	try_module_get(THIS_MODULE);

	return 0;
}

static int device_release(struct inode *inode, struct file *file) {
	module_put(THIS_MODULE);

	return 0;
}

static inline int read_addr(const char __user *user_buf, resource_size_t *out) {
	char buf[ADDR_STR_SIZE + 1] = {0};
	buf[ADDR_STR_SIZE] = '\0';

	if (copy_from_user(buf, user_buf, ADDR_STR_SIZE)) return -EFAULT;
	if (kstrtoul(buf, ADDR_STR_BASE, (unsigned long*) out)) return -EINVAL;

	return 0;
}

static ssize_t device_write(struct file *filp, const char __user *user_buf, size_t len, loff_t *off) {
	// start + end address plus - separator and \0 terminator
	if (len != ADDR_STR_SIZE * 2 + 2) return -EINVAL;

	resource_size_t start;
	resource_size_t end;

	if (read_addr(user_buf, &start)) return -EINVAL;
	if (read_addr(user_buf + ADDR_STR_SIZE + 1, &end)) return -EINVAL;

	pr_info("pat_dealloc: attempting to free PAT 0x%016llx - 0x%016llx\n", start, end);
	arch_io_free_memtype_wc(start, end - start);

	return len;
}

static int major;

static struct class *cls;

static struct file_operations chardev_fops = {
	.write = device_write,
	.open = device_open,
	.release = device_release,
};

static int __init pat_dealloc_init(void) {
	major = register_chrdev(0, DEVICE_NAME, &chardev_fops);

	if (major < 0) {
		return major;
	}

	cls = class_create(DEVICE_NAME);
	device_create(cls, NULL, MKDEV(major, 0), NULL, DEVICE_NAME);

	return 0;
}

static void __exit pat_dealloc_exit(void) {
	device_destroy(cls, MKDEV(major, 0));
	class_destroy(cls);

	unregister_chrdev(major, DEVICE_NAME);
}

module_init(pat_dealloc_init);
module_exit(pat_dealloc_exit);

MODULE_LICENSE("GPL");
