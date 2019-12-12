/// A configurable implementation of the Storage trait in memory.
#[macro_export]
macro_rules! ram_storage { (

    name=$Name:ident,
    trait=$StorageTrait:path,
    erase_value=$erase_value:expr,
    read_size=$read_size:expr,
    write_size=$write_size:expr,
    cache_size_ty=$cache_size:path,
    block_size_ty=$block_size_ty:path,
    block_size=$block_size_num:expr,
    block_count=$block_count:expr,
    lookaheadwords_size_ty=$lookaheadwords_size:path,
    filename_max_ty=$filename_max:path,

) => {
        struct $Name {
            buf: [u8; $block_size_num * $block_count],
        }

        impl Default for $Name {
            fn default() -> Self {
                $Name {
                    buf: [$erase_value; $block_size_num * $block_count],
                }
            }
        }

        impl $Name {
            const ERASE_VALUE: u8 = $erase_value;
        }

        impl $StorageTrait for $Name {
            const READ_SIZE: usize = $read_size;
            const WRITE_SIZE: usize = $write_size;
            type CACHE_SIZE = $cache_size;
            type BLOCK_SIZE = $block_size_ty;
            const BLOCK_COUNT: usize = $block_count;
            type LOOKAHEADWORDS_SIZE = $lookaheadwords_size;
            type FILENAME_MAX = $filename_max;

            fn read(&self, offset: usize, buf: &mut [u8]) -> Result<usize> {
                debug_assert!(buf.len() % Self::READ_SIZE == 0);
                for (from, to) in self.buf[offset..].iter().zip(buf.iter_mut()) {
                    *to = *from;
                }
                Ok(buf.len())
            }

            fn write(&mut self, offset: usize, data: &[u8]) -> Result<usize> {
                debug_assert!(data.len() % Self::WRITE_SIZE == 0);
                for (from, to) in data.iter().zip(self.buf[offset..].iter_mut()) {
                    *to = *from;
                }
                Ok(data.len())
            }

            fn erase(&mut self, offset: usize, len: usize) -> Result<usize> {
                use generic_array::typenum::marker_traits::Unsigned as _;
                let block_size: usize = Self::BLOCK_SIZE::to_usize();
                debug_assert!(offset % block_size == 0);
                debug_assert!(len % block_size == 0);
                for byte in self.buf[offset..offset + len].iter_mut() {
                    *byte = Self::ERASE_VALUE;
                }
                Ok(len)
            }
        }
    }
}