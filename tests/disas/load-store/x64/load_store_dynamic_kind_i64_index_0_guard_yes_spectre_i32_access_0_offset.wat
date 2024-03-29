;;! target = "x86_64"
;;! test = "compile"
;;! flags = " -C cranelift-enable-heap-access-spectre-mitigation -W memory64 -O static-memory-maximum-size=0 -O static-memory-guard-size=0 -O dynamic-memory-guard-size=0"

;; !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
;; !!! GENERATED BY 'make-load-store-tests.sh' DO NOT EDIT !!!
;; !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

(module
  (memory i64 1)

  (func (export "do_store") (param i64 i32)
    local.get 0
    local.get 1
    i32.store offset=0)

  (func (export "do_load") (param i64) (result i32)
    local.get 0
    i32.load offset=0))

;; wasm[0]::function[0]:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    0x58(%rdi), %r11
;;       subq    $4, %r11
;;       xorq    %rsi, %rsi
;;       movq    %rdx, %rax
;;       addq    0x50(%rdi), %rax
;;       cmpq    %r11, %rdx
;;       cmovaq  %rsi, %rax
;;       movl    %ecx, (%rax)
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[1]:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    0x58(%rdi), %r11
;;       subq    $4, %r11
;;       xorq    %rsi, %rsi
;;       movq    %rdx, %rax
;;       addq    0x50(%rdi), %rax
;;       cmpq    %r11, %rdx
;;       cmovaq  %rsi, %rax
;;       movl    (%rax), %eax
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
