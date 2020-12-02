  .section .text.__c_m_sh_syscall
  .global __c_m_sh_syscall
  .thumb_func
__c_m_sh_syscall:
  bkpt 0xAB
  bx lr
