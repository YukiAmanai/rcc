.intel_syntax noprefix
.global main
main:
  push 5  pop rdi
  pop rax
  push rax  push 5  pop rdi
  pop rax
  push rax  pop rdi
  pop rax
 imul rax, rdi  pop rax  ret