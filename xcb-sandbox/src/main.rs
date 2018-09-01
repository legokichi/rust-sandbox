extern crate xcb;
use xcb::shape;

fn main() {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let foreground = conn.generate_id();
    xcb::create_gc(&conn, foreground, screen.root(), &[
            (xcb::GC_FOREGROUND, screen.black_pixel()),
            (xcb::GC_GRAPHICS_EXPOSURES, 0),
    ]);

    let win = conn.generate_id();
    xcb::create_window(&conn,
        xcb::COPY_FROM_PARENT as u8,
        win,
        screen.root(),
        0, 0,
        150, 150,
        10,
        xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
        screen.root_visual(), &[
            (xcb::CW_BACK_PIXEL, screen.white_pixel()),
            (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS),
        ]
    );
    xcb::map_window(&conn, win);

    let pixmap = conn.generate_id();
    xcb::xproto::create_pixmap(&conn, 1, pixmap, screen.root(), 150, 150);
    

    conn.flush();

    let arcs: &[xcb::Arc] = &[
        xcb::Arc::new(10, 100, 60, 40, 0, 90 << 6),
        xcb::Arc::new(90, 100, 55, 40, 0, 270 << 6)
    ];

    loop {
        let event = conn.wait_for_event();
        match event {
            None => { break; }
            Some(event) => {
                let r = event.response_type() & !0x80;
                match r {
                    xcb::EXPOSE => {
                        
                        xcb::poly_arc(&conn, win, foreground, &arcs);
                        shape::mask(&conn, 0, 0, win, 0, 0, pixmap);
                        conn.flush();
                    },
                    _ => {}
                }
            }
        }
    }
}