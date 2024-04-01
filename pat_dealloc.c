#include <linux/module.h>
#include <linux/printk.h>
#include <linux/pci.h>
#include <asm/io.h>

static char *pat_dealloc_pci_address = NULL;
module_param(pat_dealloc_pci_address, charp, 0);

static struct pci_dev *get_pci_dev(int domain, int busnr, unsigned int devfn) {
	struct pci_bus *bus = pci_find_bus(domain, busnr);
	if (bus == NULL) {
		return NULL;
	}

	return pci_get_slot(bus, devfn);
}

static void try_free_memtype(struct pci_dev *dev, resource_size_t start, resource_size_t end) {
	if (start >= end) return;

	// check if requested interval is within one of the devices' resources
	for (size_t i = 0; i < PCI_NUM_RESOURCES; i++) {
		struct resource *res = &dev->resource[i];

		// we don't care about these
		if (res->start == 0) continue;
		if ((res->flags & IORESOURCE_MEM_64) == 0) continue;

		// requested interval out of bounds of resource?
		if (start < res->start || end > res->end) continue;

		pr_info("trying to free 0x%016llx - 0x%016llx\n", start, end);
		arch_io_free_memtype_wc(start, end - start);
		return;
	}
}

int init_module(void)
{
	if (pat_dealloc_pci_address == NULL) {
		return -EINVAL;
	}

	// TODO grab correct device from passed address
	struct pci_dev *dev = get_pci_dev(0, 1, 0);
	if (dev == NULL) {
		pr_info("dev null\n");
		return -EINVAL;
	}

	pr_info("pci device; bus=%d, devfn=%d, vendor=%d, device=%d\n", dev->bus->number, dev->devfn, dev->vendor, dev->device);

	// TODO determine PAT entries automatically
	try_free_memtype(dev, 0x000000f81c680000, 0x000000f81c681000);

	pci_dev_put(dev);

	return 0;
}

void cleanup_module(void) { }

MODULE_LICENSE("GPL");
