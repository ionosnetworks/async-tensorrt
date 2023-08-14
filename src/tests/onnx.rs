pub static SIMPLE_ONNX: &[u8; 155] = &[
    0x08, 0x07, 0x12, 0x0c, 0x6f, 0x6e, 0x6e, 0x78, 0x2d, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65,
    0x3a, 0x84, 0x01, 0x0a, 0x26, 0x0a, 0x01, 0x58, 0x0a, 0x04, 0x50, 0x61, 0x64, 0x73, 0x12, 0x01,
    0x59, 0x22, 0x03, 0x50, 0x61, 0x64, 0x2a, 0x13, 0x0a, 0x04, 0x6d, 0x6f, 0x64, 0x65, 0x22, 0x08,
    0x63, 0x6f, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x74, 0xa0, 0x01, 0x03, 0x12, 0x0a, 0x74, 0x65, 0x73,
    0x74, 0x2d, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x2a, 0x10, 0x08, 0x04, 0x10, 0x07, 0x3a, 0x04, 0x00,
    0x00, 0x01, 0x01, 0x42, 0x04, 0x50, 0x61, 0x64, 0x73, 0x5a, 0x13, 0x0a, 0x01, 0x58, 0x12, 0x0e,
    0x0a, 0x0c, 0x08, 0x01, 0x12, 0x08, 0x0a, 0x02, 0x08, 0x01, 0x0a, 0x02, 0x08, 0x02, 0x5a, 0x12,
    0x0a, 0x04, 0x50, 0x61, 0x64, 0x73, 0x12, 0x0a, 0x0a, 0x08, 0x08, 0x07, 0x12, 0x04, 0x0a, 0x02,
    0x08, 0x04, 0x62, 0x13, 0x0a, 0x01, 0x59, 0x12, 0x0e, 0x0a, 0x0c, 0x08, 0x01, 0x12, 0x08, 0x0a,
    0x02, 0x08, 0x01, 0x0a, 0x02, 0x08, 0x04, 0x42, 0x02, 0x10, 0x0c,
];

macro_rules! simple_onnx_file {
    () => {{
        use std::io::Write;
        let mut simple_onnx_file = tempfile::NamedTempFile::new().unwrap();
        simple_onnx_file
            .as_file_mut()
            .write_all($crate::tests::onnx::SIMPLE_ONNX)
            .unwrap();
        simple_onnx_file
    }};
}

pub(crate) use simple_onnx_file;
