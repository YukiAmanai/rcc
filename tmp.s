.intel_syntax noprefix
.global main
main:
  push 12  pop rdi
  pop rax
  push rax  push 12  pop rdi
  pop rax
  push rax  pop rdi
  pop rax
  add rax, rdi  pop rax  ret