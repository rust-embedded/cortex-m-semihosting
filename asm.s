.global __syscall

__syscall:
  bkpt 0xAB
  bx lr
