use libc;

const NF_DROP: u32 = 0;
const NF_ACCEPT: u32 = 1;
// const NF_STOLEN: u32 = 2;
const NF_QUEUE: u32 = 3;
const NF_REPEAT: u32 = 4;
const NF_STOP: u32 = 5;

#[repr(u32)]
pub enum Verdict {
    Accept = NF_ACCEPT,
    Drop = NF_DROP,
    Queue = NF_QUEUE,
    Repeat = NF_REPEAT,
    Stop = NF_STOP,
}

pub type NfHandle = *const libc::c_void;
pub type NfQueueData = *const libc::c_void;
pub type NfQueueHandle = *const libc::c_void;

pub type NfCallbackGenMsg = *const libc::c_void;
pub type NfLogData = *const libc::c_void;
pub type NfData = *const libc::c_void;

pub type QueueNum = u16;

pub type NfQueueCallback = extern "C" fn(
    qh: NfQueueHandle,
    nfgenmsg: NfCallbackGenMsg,
    nfq_data: NfLogData,
    data: NfData,
);

pub type FileDescriptor = libc::c_int;

#[repr(C)]
pub struct NfMsgPacketHdr {
    pub packet_id: u32,
    pub hw_protocol: u16,
    pub hook: u8,
}

#[derive(Copy, Clone)]
#[repr(i32)]
pub enum ProtocolFamily {
    IP = libc::AF_UNSPEC,
    UNIX = libc::AF_UNIX,
    IPv4 = libc::AF_INET,
    IPv6 = libc::AF_INET6,
}

const NF_COPY_NONE: u8 = 0;
const NF_COPY_META: u8 = 1;
const NF_COPY_PACKET: u8 = 2;

#[repr(u8)]
pub enum CopyMode {
    None = NF_COPY_NONE,
    Meta = NF_COPY_META,
    Packet = NF_COPY_PACKET,
}

#[link(name = "netfilter_queue")]
extern "C" {
    pub fn nfq_open() -> NfHandle;
    pub fn nfq_close(h: NfHandle) -> libc::c_int;
    pub fn nfq_bind_pf(h: NfHandle, pf: ProtocolFamily) -> libc::c_int;
    pub fn nfq_unbind_pf(h: NfHandle, pf: ProtocolFamily) -> libc::c_int;

    pub fn nfq_fd(h: NfHandle) -> FileDescriptor;

    pub fn nfq_create_queue(
        h: NfHandle,
        num: QueueNum,
        cb: NfQueueCallback,
        data: *mut libc::c_void,
    ) -> NfQueueHandle;
    pub fn nfq_destroy_queue(qh: NfQueueHandle) -> libc::c_int;

    pub fn nfq_handle_packet(h: NfHandle, buf: *mut libc::c_void, rc: libc::c_int) -> libc::c_int;
    pub fn nfq_set_mode(qh: NfQueueHandle, mode: CopyMode, range: u32) -> libc::c_int;
    pub fn nfq_set_queuelen(qh: NfQueueHandle, queuelen: u32) -> libc::c_int;
    pub fn nfq_set_verdict2(
        qh: NfQueueHandle,
        id: u32,
        verdict: Verdict,
        mark: u32,
        data_len: u32,
        data: *const libc::c_uchar,
    );

    pub fn nfq_get_msg_packet_hdr(nfad: NfLogData) -> *const libc::c_void;
}