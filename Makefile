#
# $Id$
#
# Copyright (c) 2022, Purushottam A. Kulkarni
# All rights reserved.
#

PROG = kmodtrust
RM = /bin/rm -f
INSTALL_PREFIX ?= $(PWD)

$(PROG):
	cargo build --release

all: $(PROG)

install:
	cargo install --bin $(PROG) --root $(INSTALL_PREFIX) --path $(PWD)

clean:
	cargo clean
	$(RM) -r $(INSTALL_PREFIX)/bin