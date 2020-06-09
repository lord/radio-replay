use lame::Lame;

use std::io::Write;

pub struct Encoder<W: Write> {
    lame: Lame,
    writer: W,
}

impl <W: Write> Encoder<W> {
    pub fn new(writer: W, sample_rate: u32) -> Self {
        let mut lame = Lame::new().unwrap();
        lame.set_quality(3).unwrap();
        lame.set_sample_rate(sample_rate).unwrap();
        lame.set_kilobitrate(64).unwrap();
        lame.init_params().unwrap();
        Self {
            lame,
            writer,
        }
    }

    pub fn add_pcm(&mut self, data: &[i16]) {
        // length calc from libmp3lame's lame.h
        // mp3buf_size in bytes = 1.25*num_samples + 7200
        self.add_pcm0(data, 3*data.len()+7200)
    }

    fn add_pcm0(&mut self, data: &[i16], buffer_size: usize) {
        let mut buf = vec![0; buffer_size];
        match self.lame.encode(data, data, &mut buf) {
            Ok(len) => {
                buf.truncate(len);
                // TODO fix
                if self.writer.write(&buf).unwrap() != len {
                    panic!("failed to write entire packet");
                }
                self.writer.flush().unwrap();
            }
            Err(lame::EncodeError::OutputBufferTooSmall) => {
                self.add_pcm0(data, buffer_size * 2)
            }
            Err(e) => {
                panic!("unexpected lame error: {:?}", e);
            }
        }
    }
}