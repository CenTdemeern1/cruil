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
    /// The type of the parsed input state associated with this type of device.
    type State;

    /// Reads the raw bytes of a HID report into the given buffer, returning how many bytes were read.
    ///
    /// It is generally recommended to use a buffer of at least [`MAX_HID_PACKET_SIZE`](crate::MAX_HID_PACKET_SIZE) bytes.
    ///
    /// `read_raw` does not parse the read data or update the internal state using said parsed data.
    ///
    /// [`read`](Self::read) uses this parsed data and internal state to track which keys were "just pressed"
    /// or "just released", so calling this function will make [`read`](Self::read) miss this frame and possibly drop the input.
    ///
    /// If no report is available, the behavior depends on the value of `blocking`:
    /// - If `blocking` is `true`: Blocks and waits for the next report, then returns that.
    /// - If `blocking` is `false`: Returns an empty slice.
    fn read_raw(&self, buffer: &mut [u8], blocking: bool) -> CruilResult<usize>;

    /// Reads and parses a HID report, and returns it, if one is available.
    ///
    /// This function reads the raw report into the internal buffer, and thus requires a mutable reference.
    ///
    /// This method does not block and returns <code>[Ok]\([None])</code> if no report is available.
    fn try_read(&mut self) -> CruilResult<Option<Self::State>>;

    /// Reads and parses a HID report, and returns it.
    ///
    /// If no report is available, the behavior depends on the value of `blocking`:
    /// - If `blocking` is `true`: Blocks and waits for the next report, then parses and returns that.
    /// - If `blocking` is `false`: Creates a "ghost report" that reports the state as unchanged from the previous report.
    fn read(&mut self, blocking: bool) -> CruilResult<Self::State>;

    /// Returns a [reusable](ReadableDeviceIter#reusability) iterator that polls the `ReadableDevice`.
    fn iter(&mut self) -> ReadableDeviceIter<'_, Self> {
        ReadableDeviceIter { device: self }
    }

    /// Consumes `self` and creates an [`OwnedReadableDeviceIter`].
    ///
    /// Equivalent to [`IntoIterator::into_iter`].
    /// Types implementing `ReadableDevice` should implement [`IntoIterator`] by calling this function when possible.
    #[doc(alias = "into_iter")]
    fn owned_iter(self) -> OwnedReadableDeviceIter<Self>
    where
        Self: Sized,
    {
        OwnedReadableDeviceIter { device: self }
    }
}

/// `ReadableDeviceIter` implements [`Iterator`] by calling [`try_read`](ReadableDevice::try_read) in the [`next`](Self::next) function.
///
/// # Reusability
/// This iterator returns [`Some`] upon a successful read or an error, and [`None`] otherwise.
/// This means this iterator can start returning [`Some`] again after it returns [`None`].
///
/// For this reason, `ReadableDeviceIter` does not implement [`FusedIterator`](std::iter::FusedIterator).
pub struct ReadableDeviceIter<'r, T: ReadableDevice + ?Sized> {
    device: &'r mut T,
}

impl<T: ReadableDevice> Iterator for ReadableDeviceIter<'_, T> {
    type Item = CruilResult<T::State>;

    #[doc(alias = "try_read")]
    fn next(&mut self) -> Option<Self::Item> {
        match self.device.try_read() {
            Ok(Some(v)) => Some(Ok(v)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

/// A [`ReadableDeviceIter`] that owns the [`ReadableDevice`],
/// obtained through [`ReadableDevice::owned_iter`], or an [`IntoIterator::into_iter`] implementation.
///
/// See [`ReadableDeviceIter`] for reusability and panic semantics.
pub struct OwnedReadableDeviceIter<T: ReadableDevice> {
    device: T,
}

impl<T: ReadableDevice> OwnedReadableDeviceIter<T> {
    /// Consumes the `OwnedReadableDeviceIter` and returns the inner device `T`.
    pub fn into_inner(self) -> T {
        self.device
    }
}

impl<T: ReadableDevice> Iterator for OwnedReadableDeviceIter<T> {
    type Item = CruilResult<T::State>;

    #[doc(alias = "try_read")]
    fn next(&mut self) -> Option<Self::Item> {
        match self.device.try_read() {
            Ok(Some(v)) => Some(Ok(v)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}
