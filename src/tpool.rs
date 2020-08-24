use std::cell::RefCell;
use std::rc::Rc;

pub use crate::bam::errors::{Error, Result};
use crate::htslib;


/// An HTSlib thread pool. Create a thread pool and use `set_thread_pool()` methods
/// to share a thread pool across multiple BAM readers & writier.
/// The Rust wrapper holds the htslib thread pool behind a Rc, and a Rc reference
/// to the thread pool is held by each reader / writer so you don't need to
/// explicitly manage the lifetime of the `ThreadPool`.
#[derive(Clone, Debug)]
pub struct ThreadPool {
    pub(crate) handle: Rc<RefCell<InnerThreadPool>>,
}

impl ThreadPool {
    /// Create a new thread pool with `n_threads` threads.
    pub fn new(n_threads: u32) -> Result<ThreadPool> {

        let ret = unsafe { htslib::hts_tpool_init(n_threads as i32) };
        println!("got ptr: {:?}", ret);

        if ret.is_null() {
            Err(Error::ThreadPool)
        } else { 
            let inner = htslib::htsThreadPool {
                pool: ret,
                qsize: n_threads as i32 * 1,
            };
            let inner = InnerThreadPool { inner };

            let handle = Rc::new(RefCell::new(inner));
            Ok(ThreadPool { handle })
        }
    }
}

/// Internal htsThreadPool
#[derive(Clone, Debug)]
pub struct InnerThreadPool {
    pub(crate) inner: htslib::htsThreadPool,
}


impl Drop for InnerThreadPool { 
    fn drop(&mut self) {

        if self.inner.pool != std::ptr::null_mut() {
            unsafe { htslib::hts_tpool_destroy(self.inner.pool); }
        }

        self.inner.pool = std::ptr::null_mut();
    }
}