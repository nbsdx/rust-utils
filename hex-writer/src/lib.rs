
pub struct HexWriter<'a> {
    writer: &'a mut dyn std::io::Write,
    count: usize,
    data: [u8; 16],
}

impl<'a> HexWriter<'a> {
    pub fn new(writer: &'a mut dyn std::io::Write) -> Self {
        Self {
            writer,
            count: 0,
            data: [0; 16],
        }
    }

    fn write_(&mut self, buf: &[u8], print: bool) -> std::io::Result<usize> {
        for byte in buf {
            if self.count % 16 == 0 {
                let prefix = format!("0x{:08X}: ", self.count);
                self.writer.write(prefix.as_bytes())?;
            }
            if print {
                self.writer.write(format!("{:02X}", byte).as_bytes())?;
            } else {
                self.writer.write(b"  ")?;
            }
            self.data[self.count % 16] = *byte;
            self.count += 1;

            if self.count % 16 == 0 {
                self.writer.write(b" | ")?;
                for c in &self.data {
                    match c {
                        0x00 ..= 0x1F | 0x7F => self.writer.write(b".")?,
                        c => self.writer.write(&[*c])?
                    };
                }
                self.data = [0; 16];
                self.writer.write(b" |\n")?;
            } else if self.count % 8 == 0 {
                self.writer.write(b"  ")?;
            } else {
                self.writer.write(b" ")?;
            }
        }
        Ok(buf.len())
    }
}

impl std::ops::Drop for HexWriter<'_> {
    fn drop(&mut self) {
        let extra = 16 - self.count % 16;
        let buf = vec![0; extra];
        self.write_(&buf, false).expect("hmmm");
    }
}

impl std::io::Write for HexWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_(buf, true)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}