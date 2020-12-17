use winapi::um::{
    handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
    winbase::{
        CreateNamedPipeA, FILE_FLAG_OVERLAPPED, PIPE_ACCESS_DUPLEX, PIPE_READMODE_MESSAGE, PIPE_TYPE_MESSAGE, PIPE_WAIT,
    },
};

pub const IPC_PIPE_BUFFER_SIZE: u32 = 1024;

pub struct NamedPipe {
    handle: usize,
}

impl NamedPipe {
    pub fn create<S: AsRef<str>>(name: S) -> Option<Self> {
        let pipe_name = format!("\\\\.\\pipe\\{}\0", name.as_ref());
        let handle = unsafe {
            CreateNamedPipeA(
                pipe_name.as_ptr() as _,
                PIPE_ACCESS_DUPLEX | FILE_FLAG_OVERLAPPED,
                PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
                1,
                IPC_PIPE_BUFFER_SIZE,
                IPC_PIPE_BUFFER_SIZE,
                0,
                std::ptr::null_mut(),
            )
        };

        if handle != INVALID_HANDLE_VALUE {
            log::trace!("Created the named pipe {:?} = 0x{:x}", name.as_ref(), handle as usize);

            Some(Self {
                handle: handle as usize,
            })
        } else {
            None
        }
    }
}

impl Drop for NamedPipe {
    fn drop(&mut self) {
        log::trace!("Dropping the named pipe");
        unsafe { CloseHandle(self.handle as _) };
    }
}