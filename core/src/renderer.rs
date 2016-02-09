use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

use servo::net::image_cache_task::new_image_cache_task;
use servo::net::resource_task::new_resource_task;
use servo::net::storage_task::StorageTaskFactory;
use servo::gfx::font_cache_task::FontCacheTask;
use servo::profile;

pub use servo::net_traits::ResourceTask as Resource;
pub use servo::net_traits::storage_task::StorageTask as Storage;
pub use servo::net_traits::image_cache_task::ImageCacheTask as ImageCache;
pub use servo::net_traits::font_cache_task::FontCacheTask as FontCache;
pub use servo::net_traits::time::ProfilerChan as TimeProfiler;
pub use servo::net_traits::mem::ProfilerChan as MemProfiler;

use servo::util::opts;

use servo::gl;
use glutin;

use browser::{buffer, Buffer, Tab, Event};

// Profiler for the servo renderer
pub struct Profiler {
    time: Option<f64>,
    mem: Option<f64>,
}

impl Profiler {
    pub fn time(&self) -> TimeProfiler {
        profile::time::Profiler::create(self.time)
    }

    pub fn mem(&self) -> MemProfiler {
        profile::mem::Profiler::create(self.mem)
    }
}
