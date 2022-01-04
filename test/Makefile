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

obj-m += module/hello.o

all:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules
	openssl req -new -nodes -utf8 -sha256 -days 7 -batch -x509 \
		-config $(OPENSSL_CONFIG) -outform DER -out $(CERTIFICATE) \
		-keyout $(PRIVATEKEY) \


clean:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean
	$(RM) certs/*.x509 certs/*.pem