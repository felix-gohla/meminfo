use cairo::Context;
use glib::translate::*;
use glib::{subclass, Object, ParamFlags, ParamSpec, SignalFlags, Type, Value};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{DrawingArea, Tooltip, Widget};
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::cell::RefCell;

glib::glib_wrapper! {
    pub struct StackedBar(
        Object<subclass::simple::InstanceStruct<StackedBarPriv>,
        subclass::simple::ClassStruct<StackedBarPriv>,
        StackedBarClass>)
        @extends DrawingArea, Widget;

    match fn {
        get_type => || StackedBarPriv::get_type().to_glib(),
    }
}

impl StackedBar {
    pub fn new(num_bars: u32) -> Self {
        let bar: Self = Object::new(Self::static_type(), &[("num-bars", &num_bars)])
            .expect("Failed to create StackedBar Widget")
            .downcast()
            .expect("Created StackedBar Widget is of wrong type");
        bar.set_has_tooltip(true);
        bar.connect_query_tooltip(|s, x, y, kb_mode, tooltip| {
            let priv_ = StackedBarPriv::from_instance(s);
            priv_.query_tooltip(&s, x, y, kb_mode, tooltip)
        });
        bar
    }
}

static PROPERTIES: [subclass::Property; 1] = [subclass::Property("num-bars", |num_bars| {
    ParamSpec::uint(
        num_bars,
        "num-bars",
        "The number of bars to display",
        0,
        100,
        0,
        ParamFlags::READWRITE,
    )
})];

#[derive(Debug)]
pub struct StackedBarPriv {
    num_bars: RefCell<u32>,
}

impl StackedBarPriv {
    fn query_tooltip(
        &self,
        widget: &StackedBar,
        x: i32,
        _y: i32,
        _keyboard_mode: bool,
        tooltip: &Tooltip,
    ) -> bool {
        let width = widget.get_allocated_width() as f64;
        let num_bars = *self.num_bars.borrow();
        if num_bars < 1 {
            return false;
        }

        let bar_width = width / num_bars as f64;
        let idx = ((x as f64) / bar_width).floor();
        tooltip.set_text(Some(&format!("{}", idx)));
        true
    }
}

impl ObjectImpl for StackedBarPriv {
    glib::glib_object_impl!();

    fn constructed(&self, obj: &Object) {
        self.parent_constructed(obj);
        /* ... */
    }

    fn set_property(&self, _obj: &Object, id: usize, value: &Value) {
        let prop = &PROPERTIES[id];
        match *prop {
            subclass::Property("num-bars", ..) => {
                let num_bars = value
                    .get_some()
                    .expect("type conformity checked by `Object::set_property`");
                self.num_bars.replace(num_bars);
            }
            _ => unimplemented!(),
        }
    }

    fn get_property(&self, _obj: &Object, id: usize) -> Result<Value, ()> {
        let prop = &PROPERTIES[id];
        match *prop {
            subclass::Property("num-bars", ..) => Ok(self.num_bars.borrow().to_value()),
            _ => unimplemented!(),
        }
    }
}

impl ObjectSubclass for StackedBarPriv {
    const NAME: &'static str = "StackedBar";
    type ParentType = gtk::DrawingArea;
    type Instance = subclass::simple::InstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib::glib_object_subclass!();

    fn class_init(klass: &mut Self::Class) {
        klass.install_properties(&PROPERTIES);
        klass.add_signal("added", SignalFlags::RUN_LAST, &[Type::U32], Type::Unit);
    }

    fn new() -> Self {
        Self {
            num_bars: RefCell::new(0),
        }
    }
}

impl WidgetImpl for StackedBarPriv {
    fn draw(&self, widget: &Widget, cr: &Context) -> Inhibit {
        let mut rng = SmallRng::seed_from_u64(420);
        let width = widget.get_allocated_width() as f64;
        let height = widget.get_allocated_height() as f64;
        let num_bars = *self.num_bars.borrow();

        if num_bars < 1 {
            return Inhibit(false);
        }

        let bar_width = width / num_bars as f64;
        for b in 0..num_bars {
            let col_rand = rng.next_u32();
            let red = ((col_rand >> 24) & 0xff) as f64 / 256.0;
            let green = ((col_rand >> 16) & 0xff) as f64 / 256.0;
            let blue = ((col_rand >> 8) & 0xff) as f64 / 256.0;
            cr.set_source_rgb(red, green, blue);
            cr.rectangle(b as f64 * bar_width, 0.0, bar_width, height);
            cr.fill();
        }
        Inhibit(false)
    }
}

impl DrawingAreaImpl for StackedBarPriv {}
