use std::io::Write;

use hex_writer;

fn main() {
  let stdout_ = std::io::stdout();
  let mut stdout = stdout_.lock();
  let mut writer = hex_writer::HexWriter::new(&mut stdout);

  writer.write(include_bytes!("./Hello.txt")).expect("Failed to write to stdout");
}