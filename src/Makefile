#=======================================================================
#  Makefile for the program `makeindex'
#
#  Copyright (C) 1987	Pehong Chen	(phc@renoir.berkeley.edu)
#  Computer Science Division
#  University of California, Berkeley
#
# On many UNIX systems, do
#	make
# to build MakeIndex, and
#	make install
# to install it.  You may need to modify BINDIR, MANDIR, and MANEXT
# for your system.
#
# For systems that require different compile flags, there are
# additional targets apollo .. vms that can be used, e.g.
#	make hp720
#
# Current target list:
#	all
#	$(BINDIR)/$(PROGRAM)
#	distclean
#	depend
#	dist
#	index
#	install
#	man
#	mostlyclean
#	print
#	program
#	$(PROGRAM)
#	realclean
#	reallyclean
#	shar
#	tar
#	tags
#	TAGS
#	test
#	uninstall
#	update
#
# [30-Jan-2025] -- Updated for Rust integration
# [20-Apr-1993] -- remove references to system include files
# [07-Aug-1991]
#=======================================================================

ALL		= $(MAKEFILE) $(DOC) $(MANUAL) $(HDRS) $(SRCS)

BINDIR		= /usr/local/bin

CFLAGS		= -O

CHMOD		= chmod

CP		= /bin/cp

DOC		= makeindex.tex

#EXTHDRS	= /usr/include/ctype.h \
#		/usr/include/stdio.h

HDRS		= genind.h \
		mkind.h \

LDFLAGS		= $(CFLAGS)

LIBS		= -lpthread -ldl -lm

# Rust/Cargo settings for qqsort
CARGO		= cargo
CARGO_FLAGS	= --release
RUST_LIB_DIR	= ../makeindex-rs/target/release
RUST_LIB	= $(RUST_LIB_DIR)/libmakeindex_rs.a

LINKER		= $(CC)

MAKEFILE	= Makefile

MANDIR		= /usr/local/man/man1

MANEXT		= 1

MANUAL		= makeindex.l

OBJS		= genind.o \
        mkind.o \

# Note: qsort.o is replaced by Rust static library
QSLIB		= libqsort.a

PRINT		= psgrind

PROGRAM		= makeindex

RM		= /bin/rm -f

SHAR		= $(PROGRAM).shar

SHELL		= /bin/sh

SRCS		= genind.c \
		mkind.c \

STRIP		= strip

TAR		= $(PROGRAM).tar

#=======================================================================

.PHONY: all clean test rust-lib

# Main targets
all:		$(PROGRAM)

$(PROGRAM):	$(OBJS) rust-lib
	@echo "Linking $(PROGRAM) with Rust qsort library..."
	$(LINKER) $(LDFLAGS) $(OBJS) $(RUST_LIB) $(LIBS) -o $(PROGRAM)
	@echo "Build complete: $(PROGRAM)"
	@ls -l $(PROGRAM)

# Rust library build target
rust-lib:
	@echo "Building Rust qsort library..."
	cd ../makeindex-rs && $(CARGO) build $(CARGO_FLAGS)
	@echo "Rust library built: $(RUST_LIB)"

# Test target
test:		$(PROGRAM)
	cd ../test && $(MAKE)

# Install targets
$(BINDIR)/$(PROGRAM): $(SRCS) $(HDRS) $(EXTHDRS)
	@$(MAKE) -f $(MAKEFILE) BINDIR=$(BINDIR) install

# Clean targets
clean mostlyclean:
	-$(RM) $(OBJS)
	-$(RM) $(PROGRAM)
	-$(RM) $(QSLIB)
	-$(RM) *.out
	-$(RM) *~
	-$(RM) \#*
	-$(RM) core
	-$(RM) test_temp.*

# Clean Rust artifacts too
distclean realclean reallyclean: clean
	@echo "Deep cleaning (including Rust artifacts)..."
	-cd ../makeindex-rs && $(CARGO) clean
	-$(RM) $(ALL)

# Development targets
depend:
	@echo "Generating dependencies..."
	mkmf -f $(MAKEFILE) PROGRAM=$(PROGRAM) BINDIR=$(BINDIR)

dist:
	$(CP) $(PROGRAM) $(BINDIR)
	rcp $(PROGRAM) monet:$(BINDIR)
	rcp $(PROGRAM) arpa:$(BINDIR)
	rcp $(PROGRAM) harrison@vangogh:bin

index:
	@ctags -wx $(HDRS) $(SRCS)

print:
	@$(PRINT) $(HDRS) $(SRCS)

man:
	ptroff -man $(MANUAL)

program:	$(PROGRAM)

shar:
	@$(RM) $(SHAR)
	shar $(ALL) > $(SHAR)

tags:		$(HDRS) $(SRCS)
	@ctags $(HDRS) $(SRCS)

TAGS:		$(HDRS) $(SRCS)
	@etags $(HDRS) $(SRCS)

tar:
	@$(RM) $(TAR)
	tar chf $(TAR) $(ALL)

uninstall:
	@echo "Uninstalling $(PROGRAM)..."
	-$(RM) $(BINDIR)/$(PROGRAM)
	-$(RM) $(MANDIR)/makeindex.$(MANEXT)

update:		$(BINDIR)/$(PROGRAM)


#=======================================================================
# Dependencies
#=======================================================================

genind.o: mkind.h genind.h
mkind.o: mkind.h

# Rust library dependency
$(RUST_LIB): ../makeindex-rs/src/qsort.rs ../makeindex-rs/src/sortid.rs ../makeindex-rs/src/scanid.rs ../makeindex-rs/Cargo.toml
	@$(MAKE) rust-lib
