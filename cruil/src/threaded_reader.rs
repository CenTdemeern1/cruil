use crate::*;
use std::{
    any::Any,
    iter::FusedIterator,
    mem::swap,
    sync::mpsc::{Receiver, Sender, TryRecvError, channel},
    thread::{JoinHandle, Result as ThreadResult, Thread, spawn},
};

/// The internal implementation for [`ThreadedReader`].
///
/// This struct ensures its internal [`Sender`] and device `T` have the same lifetime.
struct InnerThreadedReader<T>
where
    T: ReadableDevice + Send + 'static,
    T::State: Send,
{
    device: T,
    sender: Sender<CruilResult<T::State>>,
}

impl<T> InnerThreadedReader<T>
where
    T: ReadableDevice + Send + 'static,
    T::State: Send,
{
    pub(crate) fn spawn(device: T, sender: Sender<CruilResult<T::State>>) -> JoinHandle<Self> {
        spawn(move || {
            let mut reader = InnerThreadedReader { device, sender };
            reader.start();
            reader
        })
    }

    fn start(&mut self) {
        while self.sender.send(self.device.read(true)).is_ok() {}
    }
}

/// A threaded reader that can be asynchronously polled for events.
///
/// This struct includes semantics for custom [`ReadableDevice`]s that might panic.
/// The built in ones from Cruil should never panic. (If they ever do, please file a bug report!)
///
/// # Deinitializing
/// Dropping this struct will gracefully shut down the thread and close the device in the background.
///
/// To shut down the thread and get back the device, use [`stop`](Self::stop) or [`try_stop`](Self::try_stop).
///
/// Unlike the stop functions, dropping the `ThreadedReader` does not have to wait for the internal thread to stop,
/// and may thus be significantly faster because it's not bottlenecked by I/O.
pub struct ThreadedReader<T>
where
    T: ReadableDevice + Send + 'static,
    T::State: Send,
{
    receiver: Receiver<CruilResult<T::State>>,
    thread: JoinHandle<InnerThreadedReader<T>>,
}

impl<T> ThreadedReader<T>
where
    T: ReadableDevice + Send + 'static,
    T::State: Send,
{
    /// Creates a `ThreadedReader` and spawns its corresponding thread.
    pub fn start(device: T) -> Self {
        let (sender, receiver) = channel();
        ThreadedReader {
            receiver,
            thread: InnerThreadedReader::spawn(device, sender),
        }
    }

    /// Gets the first event in the queue, if there is one.
    ///
    /// If the queue is empty, returns [`TryRecvError::Empty`].
    /// If the inner thread panicked, returns [`TryRecvError::Disconnected`].
    ///
    /// The primary difference between this and [`poll_event`](Self::poll_event) is that this function doesn't panic if the inner thread does.
    pub fn try_poll_event(&self) -> Result<CruilResult<T::State>, TryRecvError> {
        self.receiver.try_recv()
    }

    /// Gets the first event in the queue, if there is one.
    ///
    /// If the queue is empty, returns [`None`].
    ///
    /// # Panics
    /// This function panics if the inner thread panicked.
    ///
    /// For a non-panicking version, see [`try_poll_event`](Self::try_poll_event).
    pub fn poll_event(&self) -> Option<CruilResult<T::State>> {
        match self.try_poll_event() {
            Ok(v) => Some(v),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                // Oh boy...
                debug_assert!(
                    self.thread.is_finished(),
                    "Disconnected (sender dropped) while unfinished"
                );
                panic!("Inner ThreadedReader thread disconnected (panic)");
            }
        }
    }

    /// Gets a reference to the internal thread.
    pub fn thread_ref(&self) -> &Thread {
        self.thread.thread()
    }

    /// Stops and joins the internal thread and returns the device `T`.
    ///
    /// Because the internal thread uses blocking operations and has to finish first, this may not be instant.
    ///
    /// If the internal thread panicked, returns [`Err`] with the value the thread panicked with.
    pub fn try_stop(self) -> ThreadResult<T> {
        drop(self.receiver);
        self.thread.join().map(|i| i.device)
    }

    /// Stops the internal thread and returns the device `T`.
    ///
    /// # Panics
    /// This resumes the panic of the internal thread if it panicked using [`std::panic::resume_unwind`].
    pub fn stop(self) -> T {
        // Apparently you can't pass `Fn(E) -> !` to a function that requires `fn(E) -> T`
        self.try_stop()
            .unwrap_or_else(|e| std::panic::resume_unwind(e))
    }

    /// Returns if the inner thread panicked.
    pub fn panicked(&self) -> bool {
        // Sanity: internal thread should never be stopped while the ThreadedReader object is alive and the InternalThreadedReader is okay
        self.thread.is_finished()
    }

    /// If the inner thread panicked, returns the value the thread panicked with.
    pub fn get_panic(self) -> Option<Box<dyn Any + Send + 'static>> {
        if self.panicked() {
            self.try_stop().err()
        } else {
            None
        }
    }

    /// Returns a [reusable](ThreadedReaderIter#reusability) iterator that polls the `ThreadedReader`.
    pub fn iter(&self) -> ThreadedReaderIter<'_, T> {
        ThreadedReaderIter { reader: self }
    }

    /// Returns an iterator that polls the `ThreadedReader` while catching panics from the internal thread.
    pub fn try_iter(self) -> ThreadedReaderTryIter<T> {
        ThreadedReaderTryIter { reader: Some(self) }
    }
}

impl<T> IntoIterator for ThreadedReader<T>
where
    T: ReadableDevice + Send + 'static,
    T::State: Send,
{
    type IntoIter = OwnedThreadedReaderIter<T>;
    type Item = CruilResult<T::State>;

    fn into_iter(self) -> Self::IntoIter {
        OwnedThreadedReaderIter { reader: self }
    }
}

/// `ThreadedReaderIter` implements [`Iterator`] by calling [`poll_event`](ThreadedReader::poll_event) in the [`next`](Self::next) function.
///
/// # Reusability
/// This iterator returns [`Some`] while there are events left in the queue.
/// It returns [`None`] if there are no events left in the queue.
///
/// This means this iterator can start returning [`Some`] again after it returns [`None`].
///
/// For this reason, `ThreadedReaderIter` does not implement [`FusedIterator`].
/// If you need a version that implements [`FusedIterator`], see [`ThreadedReaderTryIter`].
///
/// # Panics
/// Advancing the iterator shares the same panic semantics as [`poll_event`](ThreadedReader::poll_event).
pub struct ThreadedReaderIter<'r, T>
where
    T: ReadableDevice + Send + 'static,
    T::State: Send,
{
    reader: &'r ThreadedReader<T>,
}

impl<T> Iterator for ThreadedReaderIter<'_, T>
where
    T: ReadableDevice + Send + 'static,
    T::State: Send,
{
    type Item = CruilResult<T::State>;

    /// `<ThreadedReaderIter as Iterator>::next` is equivalent to [`ThreadedReader::poll_event`].
    #[doc(alias = "poll_event")]
    fn next(&mut self) -> Option<Self::Item> {
        self.reader.poll_event()
    }
}

/// A [`ThreadedReaderIter`] that owns the [`ThreadedReader`], obtained through [`ThreadedReader::into_iter`].
///
/// See [`ThreadedReaderIter`] for reusability and panic semantics.
pub struct OwnedThreadedReaderIter<T>
where
    T: ReadableDevice + Send + 'static,
    T::State: Send,
{
    reader: ThreadedReader<T>,
}

impl<T> Iterator for OwnedThreadedReaderIter<T>
where
    T: ReadableDevice + Send + 'static,
    T::State: Send,
{
    type Item = CruilResult<T::State>;

    /// `<OwnedThreadedReaderIter as Iterator>::next` is equivalent to [`ThreadedReader::poll_event`].
    #[doc(alias = "poll_event")]
    fn next(&mut self) -> Option<Self::Item> {
        self.reader.poll_event()
    }
}

/// An iterator that takes ownership of a [`ThreadedReader`] and iterates over it while also catching a possible internal thread panic.
///
/// Obtained through [`ThreadedReader::try_iter`].
///
/// See [`next`](Self::next) for semantics.
///
/// Since the semantics of this iterator only serve to handle a case that should never happen with the built in [`ReadableDevice`]s,
/// you might want to consider using [`ThreadedReaderIter`] via [`ThreadedReader::iter`] instead.
pub struct ThreadedReaderTryIter<T>
where
    T: ReadableDevice + Send + 'static,
    T::State: Send,
{
    reader: Option<ThreadedReader<T>>,
}

impl<T> Iterator for ThreadedReaderTryIter<T>
where
    T: ReadableDevice + Send + 'static,
    T::State: Send,
{
    /// See [`next`](Self::next) for semantics.
    type Item = ThreadResult<Option<CruilResult<T::State>>>;

    /// This is roughly equivalent to [`ThreadedReader::try_poll_event`].
    /// - `Some(Ok(Some(Ok(v))))` = Successfully obtained an event `v`
    /// - `Some(Ok(Some(Err(e))))` = Error `e` while getting an event
    /// - `Some(Ok(None))` = No event is waiting
    /// - `Some(Err(e))` = The internal thread panicked with value `e`
    /// - `None` = The internal thread panicked and the panic value has already been iterated over. (End of iterator)
    ///
    /// Unlike [`ThreadedReaderIter`], this iterator can't start returning [`Some`] again after it starts returning [`None`].
    ///
    /// If these semantics are too complicated, consider using [`ThreadedReader::iter`] instead to get a [`ThreadedReaderIter`].
    #[doc(alias = "try_poll_event")]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(reader) = &self.reader {
            Some(match reader.try_poll_event() {
                Ok(v) => Ok(Some(v)),
                Err(TryRecvError::Empty) => Ok(None),
                Err(TryRecvError::Disconnected) => {
                    let mut reader = None;
                    swap(&mut self.reader, &mut reader);
                    let Some(reader) = reader else {
                        unreachable!();
                    };
                    Err(reader.get_panic().unwrap())
                }
            })
        } else {
            None
        }
    }
}

/// Unlike [`ThreadedReaderIter`], this iterator can't start returning [`Some`] again after it starts returning [`None`].
impl<T> FusedIterator for ThreadedReaderTryIter<T>
where
    T: ReadableDevice + Send + 'static,
    T::State: Send,
{
}
