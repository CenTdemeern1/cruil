use crate::CruilResult;

/// A device which HID reports (aka "events" or "frames") can be read from.
///
/// # Usage
/// Reports can be read from a `ReadableDevice` by calling [`read`](Self::read),
/// which will return a high level, parsed report in a "State" such as
/// [`KeyboardInputState`](crate::keyboard::KeyboardInputState) or [`MouseInputState`](crate::mouse::MouseInputState).
///
/// Raw reports can also be read using [`read_raw`](Self::read_raw),
/// which will return the raw bytes read without parsing them.
///
/// # Polling
/// It is generally recommended to poll often, and exhaust all reports when polling, to not miss or delay reports.
///
/// The easiest way to do this is to use [`ThreadedReader`](crate::ThreadedReader),
/// which will use a background thread to continuously read the device and send reports back.
/// See its documentation for more information.
pub trait ReadableDevice {
    /// The type of input state associated with this type of device.
    type State;

    /// Reads a HID report, and returns the raw bytes.
    ///
    /// Because `read_raw` does not parse the read data,
    /// it also doesn't update the internal state using said parsed data.
    ///
    /// [`read`](Self::read) uses this parsed data and internal state to track which keys were "just pressed"
    /// or "just released", so calling this function will make [`read`](Self::read) miss this frame and possibly drop the input.
    ///
    /// The returned slice is a reference to the internal buffer.
    fn read_raw(&mut self, blocking: bool) -> CruilResult<&[u8]>;

    /// Reads and parses a HID report, and returns it.
    fn read(&mut self, blocking: bool) -> CruilResult<Self::State>;
}
