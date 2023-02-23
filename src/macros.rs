// TODO: should add another backend that randomly returns less
// data than requested, to emphasize the difference between
// `io::Read::read` and `::read_exact`.
/// A configurable implementation of the Storage trait in memory.
#[macro_export]
macro_rules! ram_storage { (
    $Name:ident,
    $Backend:ident,
    $StorageTrait:path,
    $erase_value:expr,
    $read_size:expr,
    $write_size:expr,
    $cache_size:path,
    $block_size:expr,
    $block_count:expr,
    $lookahead_size:path,
    $filename_max_plus_one:path,
    $path_max_plus_one:path,
    $Result:ident

) => {
        pub struct $Backend {
            buf: [u8; $block_size * $block_count],
        }

        impl Default for $Backend {
            fn default() -> Self {
                $Backend {
                    buf: [$erase_value; $block_size * $block_count],
                }
            }
        }

        pub struct $Name<'backend> {
            backend: &'backend mut $Backend,
        }

        impl<'backend> $Name<'backend> {
            const ERASE_VALUE: u8 = $erase_value;
            pub fn new(backend: &'backend mut $Backend) -> Self {
                $Name { backend }
            }
        }

        impl<'backend> $StorageTrait for $Name<'backend> {
            const READ_SIZE: usize = $read_size;
            const WRITE_SIZE: usize = $write_size;
            type CACHE_SIZE = $cache_size;
            const BLOCK_SIZE: usize = $block_size;
            const BLOCK_COUNT: usize = $block_count;
            type LOOKAHEAD_SIZE = $lookahead_size;

            fn read(&mut self, offset: usize, buf: &mut [u8]) -> $Result<usize> {
                let read_size: usize = Self::READ_SIZE;
                debug_assert!(offset % read_size == 0);
                debug_assert!(buf.len() % read_size == 0);
                for (from, to) in self.backend.buf[offset..].iter().zip(buf.iter_mut()) {
                    *to = *from;
                }
                Ok(buf.len())
            }

            fn write(&mut self, offset: usize, data: &[u8]) -> $Result<usize> {
                let write_size: usize = Self::WRITE_SIZE;
                debug_assert!(offset % write_size == 0);
                debug_assert!(data.len() % write_size == 0);
                for (from, to) in data.iter().zip(self.backend.buf[offset..].iter_mut()) {
                    *to = *from;
                }
                Ok(data.len())
            }

            fn erase(&mut self, offset: usize, len: usize) -> $Result<usize> {
                let block_size: usize = Self::BLOCK_SIZE;
                debug_assert!(offset % block_size == 0);
                debug_assert!(len % block_size == 0);
                for byte in self.backend.buf[offset..offset + len].iter_mut() {
                    *byte = Self::ERASE_VALUE;
                }
                Ok(len)
            }
        }
    };
    ($Name:ident, $Backend:ident, $bytes:expr) => {
        ram_storage!(
            $Name,
            $Backend,
            $crate::driver::Storage,
            0xff,
            1,
            1,
            $crate::consts::U32,
            128,
            $bytes/128,
            $crate::consts::U1,
            $crate::consts::U256,
            $crate::consts::U256,
            LfsResult
        );
    };
    (tiny) => {
        ram_storage!(
            RamStorage,
            Ram,
            $crate::driver::Storage,
            0xff,
            32,
            32,
            $crate::consts::U32,
            128,
            8,
            $crate::consts::U1,
            $crate::consts::U256,
            $crate::consts::U256,
            LfsResult
        );
    };
    (large) => {
        ram_storage!(
            RamStorage,
            Ram,
            $crate::driver::Storage,
            0xff,
            32,
            32,
            $crate::consts::U32,
            256,
            512,
            $crate::consts::U4,
            $crate::consts::U256,
            $crate::consts::U256,
            LfsResult
        );
    };
}

#[macro_export]
macro_rules! const_ram_storage { (
    $Name:ident,
    $StorageTrait:path,
    $erase_value:expr,
    $read_size:expr,
    $write_size:expr,
    $cache_size:path,
    $block_size:expr,
    $block_count:expr,
    $lookahead_size:path,
    $filename_max_plus_one:path,
    $path_max_plus_one:path,
    $Result:ident
) => {
        pub struct $Name {
            buf: [u8; $block_size * $block_count],
        }

        impl $Name {
            const ERASE_VALUE: u8 = $erase_value;
            pub const fn new() -> Self {
                // Self::default()
                Self { buf: [$erase_value; $block_size * $block_count] }
            }
        }

        impl Default for $Name {
            fn default() -> Self {
                Self {
                    buf: [$erase_value; $block_size * $block_count],
                }
            }
        }

        impl $StorageTrait for $Name {
            const READ_SIZE: usize = $read_size;
            const WRITE_SIZE: usize = $write_size;
            type CACHE_SIZE = $cache_size;
            const BLOCK_SIZE: usize = $block_size;
            const BLOCK_COUNT: usize = $block_count;
            type LOOKAHEAD_SIZE = $lookahead_size;

            fn read(&mut self, offset: usize, buf: &mut [u8]) -> $Result<usize> {
                let read_size: usize = Self::READ_SIZE;
                debug_assert!(offset % read_size == 0);
                debug_assert!(buf.len() % read_size == 0);
                for (from, to) in self.buf[offset..].iter().zip(buf.iter_mut()) {
                    *to = *from;
                }
                Ok(buf.len())
            }

            fn write(&mut self, offset: usize, data: &[u8]) -> $Result<usize> {
                let write_size: usize = Self::WRITE_SIZE;
                debug_assert!(offset % write_size == 0);
                debug_assert!(data.len() % write_size == 0);
                for (from, to) in data.iter().zip(self.buf[offset..].iter_mut()) {
                    *to = *from;
                }
                Ok(data.len())
            }

            fn erase(&mut self, offset: usize, len: usize) -> $Result<usize> {
                let block_size: usize = Self::BLOCK_SIZE;
                debug_assert!(offset % block_size == 0);
                debug_assert!(len % block_size == 0);
                for byte in self.buf[offset..offset + len].iter_mut() {
                    *byte = Self::ERASE_VALUE;
                }
                Ok(len)
            }
        }
    };
    ($Name:ident, $bytes:expr) => {
        const_ram_storage!(
            $Name,
            $crate::driver::Storage,
            0xff,
            16,
            512,
            $crate::consts::U512,
            512,
            $bytes/512,
            $crate::consts::U1,
            $crate::consts::U256,
            $crate::consts::U256,
            LfsResult
        );
    };
}
