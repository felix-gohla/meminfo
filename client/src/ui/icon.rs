use gdk_pixbuf::Pixbuf;

pub fn icon(width: i32) -> Pixbuf {
    let icon_bytes = include_bytes!("../../resources/ram-memory.png");
    let icon_stream = gio::MemoryInputStream::from_bytes(&glib::Bytes::from(icon_bytes));
    Pixbuf::from_stream_at_scale::<gio::MemoryInputStream, gio::Cancellable>(
        &icon_stream,
        width,
        width,
        true,
        None,
    )
    .expect("Icon should be convertible to Pixbuf")
}
