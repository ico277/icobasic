
global main
    extern puts
    extern malloc
    extern exit
    extern fprintf
    extern stderr
%extern%

section .text
; arguments:
;   rdi = number of vars
init_global_vars:
    mov qword [rel global_vars_count], rdi  ; init counter
    shl rdi, 4                              ; multiply rdi by 16 (8 bytes in a quad + 2 quads each since its pointer + length pairs)
    call malloc
    cmp rax, 0
    je .error
    mov qword [rel global_vars], rax   
    ret
.error:
    mov rdi, [rel stderr]
    lea rsi, [rel err_malloc]
    call fprintf
    mov rax, -1
    call exit

main:
    lea rdi, [rel msg]
    call puts
%main%
.exit:
    mov rax, 0
    ret

section .rodata
    msg db "test123", 0
    err_malloc db "Memory allocation error!", 0
%rodata%

section .bss
    global_vars       resq 1      ; ptr for global_var struct
    global_vars_count resq 1      ; count for gloabl_vars     
