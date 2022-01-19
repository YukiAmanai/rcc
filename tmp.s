.intel_syntax noprefix
.global main
main:
  push 7  pop rdi
  pop rax
  push rax
  push 0  pop rdi
  pop rax
  push rax
  pop rdi
  pop rax
  sub rax, rdi
  push rax
  push 12  pop rdi
  pop rax
  push rax
  pop rdi
  pop rax
  add rax, rdi
  push rax
  pop rax  ret