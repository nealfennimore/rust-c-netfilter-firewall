mod nfq;
use nfq::*;

extern "C" fn nfq_callback(
    qh: NfQueueHandle, 
    _nfmsg: NfCallbackGenMsg, 
    nfad: NfLogData, 
    _data: NfData
) {
    let msg_hdr = unsafe { nfq_get_msg_packet_hdr(nfad) as *const NfMsgPacketHdr };
    assert!(!msg_hdr.is_null());
    let id = u32::from_be(unsafe { (*msg_hdr).packet_id });
    unsafe { nfq_set_verdict2(qh, id, Verdict::Accept, 0, 0, std::ptr::null()) }
}

fn main() {
    let h = unsafe { nfq_open() };
    assert!(! h.is_null(), "Could not open handler");
    assert!(
        unsafe { nfq_unbind_pf( h, ProtocolFamily::IPv4 ) } >= 0, 
        "Failed to unbind"
    );
    assert!(
        unsafe { nfq_bind_pf( h, ProtocolFamily::IPv4 ) } >= 0,
        "Failed to bind"
    );
    let qh = unsafe { 
        nfq_create_queue( h, 0, nfq_callback, std::ptr::null_mut() )
    };
    assert!(! qh.is_null(), "Could not create queue");

    assert!(
        unsafe { nfq_set_mode(qh, CopyMode::Packet, 0xffff) } >= 0,
        "Could not set mode"
    );

    let fd: FileDescriptor = unsafe { nfq_fd( h ) };

    const SIZE: usize = u32::MAX as usize;
    let mut buf: Vec<u8> = vec![0; SIZE];
    let buf_ptr = buf.as_mut_ptr() as *mut libc::c_void;
    let buf_len = buf.len() as libc::size_t;

    loop {
        let rc = unsafe { libc::recv(fd, buf_ptr, buf_len, libc::MSG_DONTWAIT) };
        if rc < 0 {
            continue;
        }

        let rv = unsafe { nfq_handle_packet(h, buf_ptr, rc as libc::c_int) };
        if rv < 0 { println!("error in nfq_handle_packet()"); };
    }
    unsafe { nfq_destroy_queue(qh) };
    unsafe { nfq_close(h) };
}
