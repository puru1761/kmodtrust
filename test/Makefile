#
# $Id$
#
# Copyright (c) 2022, Purushottam A. Kulkarni
# All rights reserved.
#

RM = /bin/rm -f
CERTSDIR = certs
OPENSSL_CONFIG = $(CERTSDIR)/x509.genkey
PRIVATEKEY = $(CERTSDIR)/kmodtrust_key.pem
CERTIFICATE = $(CERTSDIR)/kmodtrust.x509

MODULEDIR = $(PWD)
KERNEL_BUILD_DIR ?= /lib/modules/$(shell uname -r)/build

obj-m += module/hello.o

all:
	make -C $(KERNEL_BUILD_DIR) M=$(MODULEDIR) modules
	openssl req -new -nodes -utf8 -sha256 -days 7 -batch -x509 \
		-config $(OPENSSL_CONFIG) -outform DER -out $(CERTIFICATE) \
		-keyout $(PRIVATEKEY) \

clean:
	make -C $(KERNEL_BUILD_DIR) M=$(MODULEDIR) clean
	$(RM) $(CERTSDIR)/*.x509 $(CERTSDIR)/*.pem
