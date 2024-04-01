obj-m += chardev_test.o
PWD := $(CURDIR)

build:
	make -C /usr/src/linux M=$(PWD) modules

clean: unload
	make -C /usr/src/linux M=$(PWD) clean

load: build unload
	sudo insmod chardev_test.ko

unload:
	-sudo rmmod chardev_test
