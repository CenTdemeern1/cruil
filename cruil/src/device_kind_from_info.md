Tries to figure out what kind of device the [`DeviceInfo`] is describing.
This is achieved through looking at its Usage info.

On Linux and other platforms using the hidapi libusb backend,
this requires opening the device, and then parsing the report descriptor,
so this operation is not free on these platforms.

If the type of device is unsupported (not a [Mouse] or [Keyboard]),
this function returns [`None`].