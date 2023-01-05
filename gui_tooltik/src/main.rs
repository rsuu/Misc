use minifb::{Key, Scale, ScaleMode, Window};

fn main() {
    let mut canvas = Canvas::new(600, 600);
    let mut data = vec![0; 600 * 600];
    let kit = Toolkit::new(400, 200, 200, 200);

    for f in 0..kit.w as usize * kit.h as usize {
        data[kit.elems[0].x1 as usize * kit.elems[0].y1 as usize + kit.w as usize] = (f as u32);
    }

    'l1: while canvas.window.is_open() {
        match canvas.window.get_keys().as_slice() {
            &[Key::Q] => {
                break 'l1;
            }

            _ => {}
        }

        //log::debug!("Key: {:?}", canvas.window.get_keys().iter().as_slice());
        canvas.flush(data.as_slice());

        std::thread::sleep(std::time::Duration::from_millis(40));
    }
}

#[derive(Debug)]
struct Elem {
    x1: u32,
    x2: u32,
    x3: u32,
    x4: u32,

    y1: u32,
    y2: u32,
    y3: u32,
    y4: u32,

    bg: u32,
    fg: u32,
    data: Vec<u8>,
}

#[derive(Debug)]
struct Toolkit {
    x: u32,
    y: u32,
    w: u32,
    h: u32,

    pos: usize,
    elems: Vec<Elem>,
}

pub struct Canvas {
    pub window: Window,
    pub size: (usize, usize),
}

impl Toolkit {
    fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self {
            x,
            y,
            w,
            h,
            pos: 0,
            elems: vec![Elem::new(x, y, w, h)],
        }
    }
}

impl Elem {
    fn new(x1: u32, y1: u32, w: u32, h: u32) -> Self {
        Self {
            x1,
            y1,

            x2: x1 + w,
            y2: y1,

            x3: x1,
            y3: y1 + h,

            x4: x1 + w,
            y4: y1 + h,

            bg: 0,
            fg: 0,
            data: vec![125; (w * h) as usize],
        }
    }
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let windowoptions = minifb::WindowOptions {
            borderless: false,
            transparency: false,
            title: true,
            resize: false,
            topmost: false,
            none: true,
            scale_mode: ScaleMode::Center,
            scale: Scale::X1,
        };

        let mut window = Window::new("rmg", width, height, windowoptions).unwrap();

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // Limit to max ~60 fps update rate

        //window.set_position(720, 0);

        Self {
            window,
            size: (width, height),
        }
    }

    #[inline(always)]
    pub fn flush(&mut self, data: &[u32]) {
        self.window
            .update_with_buffer(data, self.size.0, self.size.1)
            .unwrap();
    }
}
