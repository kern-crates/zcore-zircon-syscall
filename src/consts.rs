#![allow(dead_code)]

pub const SYS_CLOCK_GET: u32 = 0;
pub const SYS_CLOCK_GET_NEW: u32 = 1;
pub const SYS_CLOCK_GET_MONOTONIC: u32 = 2;
pub const SYS_NANOSLEEP: u32 = 3;
pub const SYS_CLOCK_ADJUST: u32 = 4;
pub const SYS_SYSTEM_GET_EVENT: u32 = 5;
pub const SYS_HANDLE_CLOSE: u32 = 6;
pub const SYS_HANDLE_CLOSE_MANY: u32 = 7;
pub const SYS_HANDLE_DUPLICATE: u32 = 8;
pub const SYS_HANDLE_REPLACE: u32 = 9;
pub const SYS_OBJECT_WAIT_ONE: u32 = 10;
pub const SYS_OBJECT_WAIT_MANY: u32 = 11;
pub const SYS_OBJECT_WAIT_ASYNC: u32 = 12;
pub const SYS_OBJECT_SIGNAL: u32 = 13;
pub const SYS_OBJECT_SIGNAL_PEER: u32 = 14;
pub const SYS_OBJECT_GET_PROPERTY: u32 = 15;
pub const SYS_OBJECT_SET_PROPERTY: u32 = 16;
pub const SYS_OBJECT_GET_INFO: u32 = 17;
pub const SYS_OBJECT_GET_CHILD: u32 = 18;
pub const SYS_OBJECT_SET_PROFILE: u32 = 19;
pub const SYS_CHANNEL_CREATE: u32 = 20;
pub const SYS_CHANNEL_READ: u32 = 21;
pub const SYS_CHANNEL_READ_ETC: u32 = 22;
pub const SYS_CHANNEL_WRITE: u32 = 23;
pub const SYS_CHANNEL_WRITE_ETC: u32 = 24;
pub const SYS_CHANNEL_CALL_NORETRY: u32 = 25;
pub const SYS_CHANNEL_CALL_FINISH: u32 = 26;
pub const SYS_SOCKET_CREATE: u32 = 27;
pub const SYS_SOCKET_WRITE: u32 = 28;
pub const SYS_SOCKET_READ: u32 = 29;
pub const SYS_SOCKET_SHARE: u32 = 30;
pub const SYS_SOCKET_ACCEPT: u32 = 31;
pub const SYS_SOCKET_SHUTDOWN: u32 = 32;
pub const SYS_THREAD_EXIT: u32 = 33;
pub const SYS_THREAD_CREATE: u32 = 34;
pub const SYS_THREAD_START: u32 = 35;
pub const SYS_THREAD_READ_STATE: u32 = 36;
pub const SYS_THREAD_WRITE_STATE: u32 = 37;
pub const SYS_PROCESS_EXIT: u32 = 38;
pub const SYS_PROCESS_CREATE: u32 = 39;
pub const SYS_PROCESS_START: u32 = 40;
pub const SYS_PROCESS_READ_MEMORY: u32 = 41;
pub const SYS_PROCESS_WRITE_MEMORY: u32 = 42;
pub const SYS_JOB_CREATE: u32 = 43;
pub const SYS_JOB_SET_POLICY: u32 = 44;
pub const SYS_TASK_BIND_EXCEPTION_PORT: u32 = 45;
pub const SYS_TASK_SUSPEND: u32 = 46;
pub const SYS_TASK_SUSPEND_TOKEN: u32 = 47;
pub const SYS_TASK_RESUME_FROM_EXCEPTION: u32 = 48;
pub const SYS_TASK_CREATE_EXCEPTION_CHANNEL: u32 = 49;
pub const SYS_TASK_KILL: u32 = 50;
pub const SYS_EXCEPTION_GET_THREAD: u32 = 51;
pub const SYS_EXCEPTION_GET_PROCESS: u32 = 52;
pub const SYS_EVENT_CREATE: u32 = 53;
pub const SYS_EVENTPAIR_CREATE: u32 = 54;
pub const SYS_FUTEX_WAIT: u32 = 55;
pub const SYS_FUTEX_WAKE: u32 = 56;
pub const SYS_FUTEX_REQUEUE: u32 = 57;
pub const SYS_FUTEX_WAKE_SINGLE_OWNER: u32 = 58;
pub const SYS_FUTEX_REQUEUE_SINGLE_OWNER: u32 = 59;
pub const SYS_FUTEX_GET_OWNER: u32 = 60;
pub const SYS_PORT_CREATE: u32 = 61;
pub const SYS_PORT_QUEUE: u32 = 62;
pub const SYS_PORT_WAIT: u32 = 63;
pub const SYS_PORT_CANCEL: u32 = 64;
pub const SYS_TIMER_CREATE: u32 = 65;
pub const SYS_TIMER_SET: u32 = 66;
pub const SYS_TIMER_CANCEL: u32 = 67;
pub const SYS_VMO_CREATE: u32 = 68;
pub const SYS_VMO_READ: u32 = 69;
pub const SYS_VMO_WRITE: u32 = 70;
pub const SYS_VMO_GET_SIZE: u32 = 71;
pub const SYS_VMO_SET_SIZE: u32 = 72;
pub const SYS_VMO_OP_RANGE: u32 = 73;
pub const SYS_VMO_CREATE_CHILD: u32 = 74;
pub const SYS_VMO_SET_CACHE_POLICY: u32 = 75;
pub const SYS_VMO_REPLACE_AS_EXECUTABLE: u32 = 76;
pub const SYS_VMAR_ALLOCATE: u32 = 77;
pub const SYS_VMAR_DESTROY: u32 = 78;
pub const SYS_VMAR_MAP: u32 = 79;
pub const SYS_VMAR_UNMAP: u32 = 80;
pub const SYS_VMAR_PROTECT: u32 = 81;
pub const SYS_CPRNG_DRAW_ONCE: u32 = 82;
pub const SYS_CPRNG_ADD_ENTROPY: u32 = 83;
pub const SYS_FIFO_CREATE: u32 = 84;
pub const SYS_FIFO_READ: u32 = 85;
pub const SYS_FIFO_WRITE: u32 = 86;
pub const SYS_PROFILE_CREATE: u32 = 87;
pub const SYS_DEBUGLOG_CREATE: u32 = 88;
pub const SYS_DEBUGLOG_WRITE: u32 = 89;
pub const SYS_DEBUGLOG_READ: u32 = 90;
pub const SYS_KTRACE_READ: u32 = 91;
pub const SYS_KTRACE_CONTROL: u32 = 92;
pub const SYS_KTRACE_WRITE: u32 = 93;
pub const SYS_MTRACE_CONTROL: u32 = 94;
pub const SYS_DEBUG_READ: u32 = 95;
pub const SYS_DEBUG_WRITE: u32 = 96;
pub const SYS_DEBUG_SEND_COMMAND: u32 = 97;
pub const SYS_INTERRUPT_CREATE: u32 = 98;
pub const SYS_INTERRUPT_BIND: u32 = 99;
pub const SYS_INTERRUPT_WAIT: u32 = 100;
pub const SYS_INTERRUPT_DESTROY: u32 = 101;
pub const SYS_INTERRUPT_ACK: u32 = 102;
pub const SYS_INTERRUPT_TRIGGER: u32 = 103;
pub const SYS_INTERRUPT_BIND_VCPU: u32 = 104;
pub const SYS_IOPORTS_REQUEST: u32 = 105;
pub const SYS_VMO_CREATE_CONTIGUOUS: u32 = 106;
pub const SYS_VMO_CREATE_PHYSICAL: u32 = 107;
pub const SYS_IOMMU_CREATE: u32 = 108;
pub const SYS_BTI_CREATE: u32 = 109;
pub const SYS_BTI_PIN: u32 = 110;
pub const SYS_BTI_RELEASE_QUARANTINE: u32 = 111;
pub const SYS_PMT_UNPIN: u32 = 112;
pub const SYS_FRAMEBUFFER_GET_INFO: u32 = 113;
pub const SYS_FRAMEBUFFER_SET_RANGE: u32 = 114;
pub const SYS_PCI_GET_NTH_DEVICE: u32 = 115;
pub const SYS_PCI_ENABLE_BUS_MASTER: u32 = 116;
pub const SYS_PCI_RESET_DEVICE: u32 = 117;
pub const SYS_PCI_CONFIG_READ: u32 = 118;
pub const SYS_PCI_CONFIG_WRITE: u32 = 119;
pub const SYS_PCI_CFG_PIO_RW: u32 = 120;
pub const SYS_PCI_GET_BAR: u32 = 121;
pub const SYS_PCI_MAP_INTERRUPT: u32 = 122;
pub const SYS_PCI_QUERY_IRQ_MODE: u32 = 123;
pub const SYS_PCI_SET_IRQ_MODE: u32 = 124;
pub const SYS_PCI_INIT: u32 = 125;
pub const SYS_PCI_ADD_SUBTRACT_IO_RANGE: u32 = 126;
pub const SYS_PC_FIRMWARE_TABLES: u32 = 127;
pub const SYS_SMC_CALL: u32 = 128;
pub const SYS_RESOURCE_CREATE: u32 = 129;
pub const SYS_GUEST_CREATE: u32 = 130;
pub const SYS_GUEST_SET_TRAP: u32 = 131;
pub const SYS_VCPU_CREATE: u32 = 132;
pub const SYS_VCPU_RESUME: u32 = 133;
pub const SYS_VCPU_INTERRUPT: u32 = 134;
pub const SYS_VCPU_READ_STATE: u32 = 135;
pub const SYS_VCPU_WRITE_STATE: u32 = 136;
pub const SYS_SYSTEM_MEXEC: u32 = 137;
pub const SYS_SYSTEM_MEXEC_PAYLOAD_GET: u32 = 138;
pub const SYS_SYSTEM_POWERCTL: u32 = 139;
pub const SYS_PAGER_CREATE: u32 = 140;
pub const SYS_PAGER_CREATE_VMO: u32 = 141;
pub const SYS_PAGER_DETACH_VMO: u32 = 142;
pub const SYS_PAGER_SUPPLY_PAGES: u32 = 143;
pub const SYS_SYSCALL_TEST_0: u32 = 144;
pub const SYS_SYSCALL_TEST_1: u32 = 145;
pub const SYS_SYSCALL_TEST_2: u32 = 146;
pub const SYS_SYSCALL_TEST_3: u32 = 147;
pub const SYS_SYSCALL_TEST_4: u32 = 148;
pub const SYS_SYSCALL_TEST_5: u32 = 149;
pub const SYS_SYSCALL_TEST_6: u32 = 150;
pub const SYS_SYSCALL_TEST_7: u32 = 151;
pub const SYS_SYSCALL_TEST_8: u32 = 152;
pub const SYS_SYSCALL_TEST_WRAPPER: u32 = 153;
pub const SYS_COUNT: u32 = 154;
