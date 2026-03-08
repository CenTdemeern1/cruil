use crate::CruilResult;

pub trait ReadableDevice {
    type State;

    fn read_raw(&mut self, blocking: bool) -> CruilResult<&[u8]>;
    fn read(&mut self, blocking: bool) -> CruilResult<Self::State>;
}
