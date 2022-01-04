#
# $Id$
#
# Copyright (c) 2022, Purushottam A. Kulkarni
# All rights reserved.
#

PROG = kmodtrust
RM = /bin/rm -f
CERTSDIR = test/certs
OPENSSL_CONFIG = $(CERTSDIR)/x509.genkey
PRIVATEKEY = $(CERTSDIR)/kmodtrust_key.pem
CERTIFICATE = $(CERTSDIR)/kmodtrust.x509
MODULEDIR = $(PWD)/test
INSTALL_PREFIX ?= $(PWD)

all:
	cargo build --release
	make -C /lib/modules/$(shell uname -r)/build M=$(MODULEDIR) modules
	openssl req -new -nodes -utf8 -sha256 -days 7 -batch -x509 \
		-config $(OPENSSL_CONFIG) -outform DER -out $(CERTIFICATE) \
		-keyout $(PRIVATEKEY) \

install:
	cargo install --bin $(PROG) --root $(INSTALL_PREFIX) --path $(PWD)


clean:
	cargo clean
	make -C /lib/modules/$(shell uname -r)/build M=$(MODULEDIR) clean
	$(RM) $(CERTSDIR)/*.x509 $(CERTSDIR)/*.pem
	$(RM) -r $(INSTALL_PREFIX)/bin