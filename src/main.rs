use std::os;

use std::io;
use std::io::File;

mod frame;
mod header;
mod peeker;
mod bitreader;
mod layer1;

fn main() {
  let f = File::open(&Path::new(os::args()[1].clone()));

  let mut working = true;
  let mut reader = f.unwrap();

  while working {
    working = false;
    match frame::MpegFrame::read_from(&mut reader) {
      Ok(h) => match h {
        Some(h) => {
          let samples = layer1::decode_layer1(&mut reader, h.header);
          // for i in range(0, 2) {
          //   for j in range(0, 12) {
          //     for k in range(0, 32) {
          //       println!("ch = {}, sample = {}, sb = {}: {}", i, j, k, samples[i][j][k]);
          //     }
          //   }
          // }
        },
        None => {
          reader.seek(1, io::SeekCur).unwrap();
        }
      },
      Err(e) =>  if e.kind == io::EndOfFile { working = false }
    }
  }
}