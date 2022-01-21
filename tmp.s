.intel_syntax noprefix
.global main
main:
  push 12  pop rdi
  pop rax
  push rax
  push 1  pop rdi
  pop rax
  push rax
  push 21  pop rdi
  pop rax
  push rax
  pop rdi
  pop rax
  cqo
  idiv rdi
  push rax
  pop rdi
  pop rax
  imul rax, rdi
  push rax
  pop rax  ret