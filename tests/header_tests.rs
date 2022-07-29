use slmp;

#[test]
fn test_header_build() {
    let mut buf1 = Vec::new();
    let mut buf2 = Vec::new();

    let hdr1 = slmp::Header::default();
    let hdr2 = slmp::Header {
        serial_no: Some(0x1234),
        dst_ne_no: 0xde,
        dst_st_no: 0xad,
        dst_md_no: 0xefbe,
        dst_mt_no: 0xee,
    };

    hdr1.build(&mut buf1);
    hdr2.build(&mut buf2);

    assert_eq!(&buf1, &[0x50, 0x00, 0x00, 0xff, 0xff, 0x03, 0x00]);
    assert_eq!(&buf2, &[0x54, 0x00, 0x34, 0x12, 0x00, 0x00, 0xde, 0xad, 0xbe, 0xef, 0xee]);
}

#[test]
fn test_header_from() {
    let buf1 = [0xd0, 0x00, 0x00, 0xff, 0xff, 0x03, 0x00];
    let buf2 = [0xd4, 0x00, 0x34, 0x12, 0x00, 0x00, 0xde, 0xad, 0xbe, 0xef, 0xee];

    let hdr1 = slmp::Header::from(&buf1);
    let hdr2 = slmp::Header::from(&buf2);

    assert_eq!(hdr1, slmp::Header::default());
    assert_eq!(
        hdr2,
        slmp::Header {
            serial_no: Some(0x1234),
            dst_ne_no: 0xde,
            dst_st_no: 0xad,
            dst_md_no: 0xefbe,
            dst_mt_no: 0xee,
        }
    );
}