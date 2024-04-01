obj-m += pat_dealloc.o
PWD := $(CURDIR)

build:
	make -C /usr/src/linux M=$(PWD) modules

clean: unload
	make -C /usr/src/linux M=$(PWD) clean

load: build unload
	sudo insmod pat_dealloc.ko pat_dealloc_pci_address=01:00.0

unload:
	-sudo rmmod pat_dealloc
