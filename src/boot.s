.section .multiboot
.align 4
.long 0x1BADB002  # MAGIC
.long 0x00000003  # FLAGS (ALIGN | MEMINFO)
.long -(0x1BADB002 + 0x00000003)  # CHECKSUM

.section .bss
.align 16
stack_bottom:
.skip 16384 # 16 KiB
stack_top:

.section .text
.global _start
.type _start, @function
_start:
    mov $stack_top, %esp
    call _rust_start
    
    cli
1:  hlt
    jmp 1b
