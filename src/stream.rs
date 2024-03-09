use std::{
    io::{Read, Seek, SeekFrom},
    pin::Pin,
};

use futures::{AsyncRead, AsyncReadExt, StreamExt, TryStreamExt};
use tokio::runtime::Handle;

pub struct SyncReader {
    reader: Pin<Box<dyn AsyncRead + Send + Sync>>,
    handle: Handle,
}

impl SyncReader {
    fn new(reader: impl AsyncRead + Send + Sync + 'static, handle: Handle) -> SyncReader {
        SyncReader {
            reader: Box::pin(reader),
            handle,
        }
    }
}

impl Read for SyncReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.handle.block_on(async { self.reader.read(buf).await })
    }
}

pub struct SongStream<R: Read> {
    stream: R,
    loaded: Vec<u8>,
    index: usize,
}

impl<R: Read> SongStream<R> {
    pub fn new(stream: R) -> SongStream<R> {
        SongStream {
            stream,
            loaded: Vec::new(),
            index: 0,
        }
    }

    fn ensure(&mut self, pos: usize) -> std::io::Result<()> {
        let current = self.loaded.len();

        if pos <= current {
            return Ok(());
        }

        self.loaded.resize(pos, 0);

        let unfilled = {
            let mut buf = &mut self.loaded[current..pos];

            while !buf.is_empty() {
                match self.stream.read(buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        buf = &mut buf[n..];
                    }
                    Err(e) => return Err(e),
                }
            }

            buf.len()
        };

        self.loaded.resize(pos - unfilled, 0);

        Ok(())
    }
}

impl<R: Read> Read for SongStream<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_requested = buf.len();

        self.ensure(self.index + bytes_requested)?;

        let bytes_available = self.loaded.len() - self.index;

        let to_write = std::cmp::min(bytes_requested, bytes_available);

        let loaded_end = self.index + to_write;

        buf[..to_write].copy_from_slice(&self.loaded[self.index..loaded_end]);

        self.index = loaded_end;

        Ok(to_write)
    }
}

impl<R: Read> Seek for SongStream<R> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        use SeekFrom::*;
        match pos {
            Start(pos) => self.index = pos as usize,
            End(pos) => self.index = ((self.loaded.len() as i64) + pos) as usize,
            Current(pos) => self.index = ((self.index as i64) + pos) as usize,
        }

        Ok(0)
    }
}

pub fn from_response(response: reqwest::Response) -> SongStream<SyncReader> {
    let s = response
        .bytes_stream()
        .fuse()
        .map_err(std::io::Error::other)
        .into_async_read();

    let sync_reader = SyncReader::new(s, Handle::current());

    SongStream::new(sync_reader)
}
