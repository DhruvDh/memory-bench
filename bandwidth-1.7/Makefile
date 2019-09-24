#============================================================================
# bandwidth, a benchmark to estimate memory transfer bandwidth.
# Copyright (C) 2005-2017 by Zack T Smith.
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program; if not, write to the Free Software
# Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
#
# The author may be reached at veritas@comcast.net.
#============================================================================

CFLAGS= -g -Wno-conditional-type-mismatch
CC=gcc -Wall
SRC=main.c 
OBJ=main.o
LIB= 
AS=nasm
ARM_AS=as
ARM_CC=gcc

# Note: Mac 32-bit no longer supported by Mac OS.

message:
	@echo ""
	@echo "To compile for x86 Linux:             make bandwidth32"
	@echo "To compile for x86_64 Linux:          make bandwidth64"
	@echo "To compile for x86_64 Mac OS/X:       make bandwidth-mac64"
	@echo "To compile for Raspberry pi 3 32-bit: make bandwidth-rpi32"
	@echo "To compile for generic ARM Linux:     make bandwidth-arm32"
	@echo "To compile for x86 Win32/Cygwin:      make bandwidth-win32"
	@echo "Note! For the Mac you will need to install the latest NASM; Apple's is insufficient."
	@echo ""

OOC32:
	( cd OOC; make OOC32.a )

OOC64:
	( cd OOC; make OOC64.a )

OOCLIB32=OOC/OOC32.a
OOCLIB64=OOC/OOC64.a

bandwidth-rpi32:  main.c routines-arm-rpi3-32bit.asm OOC32
	${ARM_AS} -march=armv7 -mfpu=neon routines-arm-rpi3-32bit.asm -mcpu=xscale -o routines-arm-rpi3-32bit.o
	${ARM_CC} routines-arm-rpi3-32bit.o ${SRC} ${CFLAGS} ${OOCLIB32} -lm -o bandwidth-rpi32

bandwidth-arm32:  main.c routines-arm-32bit.asm OOC32
	${ARM_AS} routines-arm-generic-32bit.asm -mcpu=xscale -o routines-arm-generic-32bit.o
	${ARM_CC} ${CFLAGS} routines-arm-generic-32bit.o ${SRC} ${CFLAGS} ${OOCLIB32} -lm -o bandwidth-arm32

bandwidth64:	main.c routines-x86-64bit.asm OOC64
	${AS} -f elf64 utility-x86-64bit.asm -o utility-x86-64bit.o
	${AS} -f elf64 routines-x86-64bit.asm -o routines-x86-64bit.o
	${CC} ${CFLAGS} -m64 ${SRC} utility-x86-64bit.o routines-x86-64bit.o ${OOCLIB64} -lm -o bandwidth64 

bandwidth32:	main.c routines-x86-32bit.asm OOC32
	${AS} -f elf utility-x86-32bit.asm -o utility-x86-32bit.o
	${AS} -f elf routines-x86-32bit.asm -o routines-x86-32bit.o
	${CC} ${CFLAGS} -m32 utility-x86-32bit.o routines-x86-32bit.o ${SRC} ${OOCLIB32} -lm -o bandwidth32 

bandwidth-mac64:	main.c routines-x86-64bit.asm OOC64
	${AS} -f macho64 utility-x86-64bit.asm -o utility-x86-64bit.o
	${AS} -f macho64 routines-x86-64bit.asm -o routines-x86-64bit.o
	${CC} -m64 ${CFLAGS} ${SRC} utility-x86-64bit.o routines-x86-64bit.o ${OOCLIB64} -lm -o bandwidth-mac64

bandwidth-mac32:	main.c routines-x86-32bit.asm OOC32
	${AS} -f macho32 utility-x86-32bit.asm -o utility-x86-32bit.o
	${AS} -f macho32 routines-x86-32bit.asm -o routines-x86-32bit.o
	${CC} -m32 ${CFLAGS} ${SRC} utility-x86-32bit.o routines-x86-32bit.o ${OOCLIB32} -lm -o bandwidth-mac32

bandwidth-win32:	main.c routines-x86-32bit.asm OOC32
	${AS} -f macho32 routines-x86-32bit.asm -o routines-x86-32bit.o
	${CC} ${CFLAGS} -D__WIN32__ -DWINVER=0x0600 -m32 -lm ${SRC} routines-x86-32bit.o ${OOCLIB32} -o bandwidth-win32

clean:
	rm -f main.o bandwidth bandwidth32 bandwidth64 routines-x86-32bit.o routines-x86-64bit.o 
	rm -f utility-x86-32bit.o utility-x86-64bit.o
	rm -f bandwidth-win32.exe bandwidth.bmp bandwidth-mac32 bandwidth-mac64 bandwidth-rpi32 bandwidth-arm32 routines-arm-rpi3-32bit.o routines-x86-64bit.o
	rm -rf bandwidth-mac64.dSYM bandwidth-mac32.dSYM
	( cd OOC; make clean )
