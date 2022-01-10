.intel_syntax noprefix
.global main
main:
  push 20  pop rdi
  pop rax
  push rax  push 20  pop rdi
  pop rax
  push rax  pop rdi
  pop rax
  add rax, rdi  pop rax  ret