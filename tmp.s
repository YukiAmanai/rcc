.intel_syntax noprefix
.global main
main:
  push 1  pop rdi
  pop rax
  push rax
  push 1  pop rdi
  pop rax
  push rax
  pop rdi
  pop rax
  imul rax, rdi
  push rax
  push 12  pop rdi
  pop rax
  push rax
  pop rdi
  pop rax
  add rax, rdi
  push rax
  pop rax
  ret
