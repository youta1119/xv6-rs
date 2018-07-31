
target/i386-unknown-none/release/bootloader:	file format ELF32-i386

Disassembly of section .text:
start:
    7c00:	fa 	cli
    7c01:	fc 	cld
    7c02:	31 c0 	xorl	%eax, %eax
    7c04:	8e d8 	movl	%eax, %ds
    7c06:	8e c0 	movl	%eax, %es
    7c08:	8e d0 	movl	%eax, %ss

seta20.1:
    7c0a:	e4 64 	inb	$100, %al
    7c0c:	a8 02 	testb	$2, %al
    7c0e:	75 fa 	jne	-6 <seta20.1>
    7c10:	b0 d1 	movb	$-47, %al
    7c12:	e6 64 	outb	%al, $100

seta20.2:
    7c14:	e4 64 	inb	$100, %al
    7c16:	a8 02 	testb	$2, %al
    7c18:	75 fa 	jne	-6 <seta20.2>
    7c1a:	b0 df 	movb	$-33, %al
    7c1c:	e6 60 	outb	%al, $96
    7c1e:	67 0f 01 15 	lgdtl	(%di)
    7c22:	68 7c 00 00 0f 	pushl	$251658364
    7c27:	20 c0 	andb	%al, %al
    7c29:	66 83 c8 01 	orw	$1, %ax
    7c2d:	0f 22 c0 	movl	%eax, %cr0
    7c30:	ea 35 7c 08 00 66 b8 	ljmpl	$-18330, $556085

protcseg:
    7c35:	66 b8 10 00 	movw	$16, %ax
    7c39:	8e d8 	movl	%eax, %ds
    7c3b:	8e c0 	movl	%eax, %es
    7c3d:	8e d0 	movl	%eax, %ss
    7c3f:	8e e0 	movl	%eax, %fs
    7c41:	8e e8 	movl	%eax, %gs
    7c43:	bc 00 7c 00 00 	movl	$31744, %esp
    7c48:	e8 23 00 00 00 	calll	35 <bootmain>

spin:
    7c4d:	f4 	hlt
    7c4e:	eb fd 	jmp	-3 <spin>

gdt:
    7c50:	00 00 	addb	%al, (%eax)
    7c52:	00 00 	addb	%al, (%eax)
    7c54:	00 00 	addb	%al, (%eax)
    7c56:	00 00 	addb	%al, (%eax)
    7c58:	ff ff  <unknown>
    7c5a:	00 00 	addb	%al, (%eax)
    7c5c:	00 9a cf 00 ff ff 	addb	%bl, -65329(%edx)
    7c62:	00 00 	addb	%al, (%eax)
    7c64:	00 92 cf 00 17 00 	addb	%dl, 1507535(%edx)

gdtdesc:
    7c68:	17 	popl	%ss
    7c69:	00 50 7c 	addb	%dl, 124(%eax)
    7c6c:	00 00 	addb	%al, (%eax)
    7c6e:	cc 	int3
    7c6f:	cc 	int3

bootmain:
    7c70:	55 	pushl	%ebp
    7c71:	c6 05 00 80 0b 00 48 	movb	$72, 753664
    7c78:	c6 05 01 80 0b 00 0b 	movb	$11, 753665
    7c7f:	c6 05 02 80 0b 00 65 	movb	$101, 753666
    7c86:	c6 05 03 80 0b 00 0b 	movb	$11, 753667
    7c8d:	c6 05 04 80 0b 00 6c 	movb	$108, 753668
    7c94:	c6 05 05 80 0b 00 0b 	movb	$11, 753669
    7c9b:	c6 05 06 80 0b 00 6c 	movb	$108, 753670
    7ca2:	c6 05 07 80 0b 00 0b 	movb	$11, 753671
    7ca9:	c6 05 08 80 0b 00 6f 	movb	$111, 753672
    7cb0:	c6 05 09 80 0b 00 0b 	movb	$11, 753673
    7cb7:	c6 05 0a 80 0b 00 20 	movb	$32, 753674
    7cbe:	c6 05 0b 80 0b 00 0b 	movb	$11, 753675
    7cc5:	c6 05 0c 80 0b 00 57 	movb	$87, 753676
    7ccc:	c6 05 0d 80 0b 00 0b 	movb	$11, 753677
    7cd3:	c6 05 0e 80 0b 00 6f 	movb	$111, 753678
    7cda:	c6 05 0f 80 0b 00 0b 	movb	$11, 753679
    7ce1:	c6 05 10 80 0b 00 72 	movb	$114, 753680
    7ce8:	c6 05 11 80 0b 00 0b 	movb	$11, 753681
    7cef:	c6 05 12 80 0b 00 6c 	movb	$108, 753682
    7cf6:	c6 05 13 80 0b 00 0b 	movb	$11, 753683
    7cfd:	c6 05 14 80 0b 00 64 	movb	$100, 753684
    7d04:	c6 05 15 80 0b 00 0b 	movb	$11, 753685
    7d0b:	c6 05 16 80 0b 00 21 	movb	$33, 753686
    7d12:	89 e5 	movl	%esp, %ebp
    7d14:	c6 05 17 80 0b 00 0b 	movb	$11, 753687
    7d1b:	0f 1f 44 00 00 	nopl	(%eax,%eax)
    7d20:	eb fe 	jmp	-2 <bootmain+0xb0>
Disassembly of section .sign:
.sign:
    7dfe:	55 	pushl	%ebp
    7dff:	aa 	stosb	%al, %es:(%edi)
