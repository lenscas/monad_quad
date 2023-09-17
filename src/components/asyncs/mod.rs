use std::{cell::RefCell, pin::Pin, rc::Rc};

use futures::{executor::block_on, future::select, Future};
use macroquad::{logging, prelude::error};

mod executor;
mod on_loaded;
mod on_loaded_map;
mod with_loading;
pub use executor::AsyncExecutor;
pub use on_loaded::OnLoaded;
pub use on_loaded_map::{AsyncComp, AsyncSelector, MapInto};
pub use with_loading::WithLoading;
pub enum AsyncState<T> {
    Unloaded,
    Loading(
        Option<Rc<RefCell<T>>>,
        Rc<RefCell<Pin<Box<dyn Future<Output = T>>>>>,
    ),
    Loaded(Rc<RefCell<T>>),
}

impl<T> Clone for AsyncState<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Unloaded => Self::Unloaded,
            Self::Loading(arg0, arg1) => Self::Loading(arg0.clone(), arg1.clone()),
            Self::Loaded(arg0) => Self::Loaded(arg0.clone()),
        }
    }
}

impl<T> AsyncState<T> {
    pub fn new_loading<X: Future<Output = T> + 'static>(fut: X) -> Self {
        Self::new_loading_with_cached_val(None, fut)
    }
    pub fn new_loading_with_cache<X: Future<Output = T> + 'static>(cache: T, fut: X) -> Self {
        let cache = Some(Rc::new(RefCell::new(cache)));
        Self::new_loading_with_cached_val(cache, fut)
    }
    pub fn new_done(value: T) -> Self {
        Self::Loaded(Rc::new(RefCell::new(value)))
    }
    pub fn new_unloaded() -> Self {
        Self::Unloaded
    }
    fn new_loading_with_cached_val<X: Future<Output = T> + 'static>(
        a: Option<Rc<RefCell<T>>>,
        fut: X,
    ) -> Self {
        let z = Rc::new(RefCell::new(
            Box::pin(fut) as Pin<Box<dyn Future<Output = T>>>
        ));
        Self::Loading(a, z)
    }
    pub fn to_loading<X: Future<Output = T> + 'static>(&mut self, fut: X) -> &mut Self {
        let old_value = match self {
            AsyncState::Loaded(x) => Some(x.clone()),
            _ => None,
        };
        *self = Self::new_loading_with_cached_val(old_value, fut);
        self
    }
    pub fn to_done(&mut self, value: T) -> &mut Self {
        *self = Self::new_done(value);
        self
    }
    pub fn to_unloaded(&mut self) -> &mut Self {
        *self = Self::new_unloaded();
        self
    }
    pub fn is_loading(&self) -> bool {
        matches!(self, AsyncState::Loading(_, _))
    }
    pub fn is_loaded(&self) -> bool {
        matches!(self, AsyncState::Loaded(_))
    }
    pub fn is_unloaded(&self) -> bool {
        matches!(self, AsyncState::Unloaded)
    }

    pub fn get_value_or_cache(&self) -> Option<(bool, std::cell::Ref<'_, T>)> {
        let (loaded, val) = if let AsyncState::Loaded(x) = self {
            (true, x)
        } else if let AsyncState::Loading(Some(x), _) = self {
            (false, x)
        } else {
            return None;
        };
        let x = match val.as_ref().try_borrow() {
            Ok(x) => x,
            Err(e) => {
                error!(
                    "Tried to get a value from an async state that is already borrowed mutably."
                );
                error!("Returning None instead of panicking");
                error!("Error: {}", e);
                return None;
            }
        };
        Some((loaded, x))
    }

    pub fn get_value(&self) -> Option<std::cell::Ref<'_, T>> {
        if let AsyncState::Loaded(x) = self {
            let x = match x.as_ref().try_borrow() {
                Ok(x) => x,
                Err(e) => {
                    error!("Tried to get a value from an async state that is already borrowed mutably.");
                    error!("Returning None instead of panicking");
                    error!("Error: {}", e);
                    return None;
                }
            };
            Some(x)
        } else {
            None
        }
    }
    pub fn get_value_or_cache_mut(&self) -> Option<(bool, std::cell::RefMut<'_, T>)> {
        let (loaded, val) = if let AsyncState::Loaded(x) = self {
            (true, x)
        } else if let AsyncState::Loading(Some(x), _) = self {
            (false, x)
        } else {
            return None;
        };
        let x = match val.as_ref().try_borrow_mut() {
            Ok(x) => x,
            Err(e) => {
                error!(
                    "Tried to get a value from an async state that is already borrowed mutably."
                );
                error!("Returning None instead of panicking");
                error!("Error: {}", e);
                return None;
            }
        };
        Some((loaded, x))
    }
    pub fn get_value_mut(&self) -> Option<std::cell::RefMut<'_, T>> {
        if let AsyncState::Loaded(x) = self {
            let x = match x.as_ref().try_borrow_mut() {
                Ok(x) => x,
                Err(e) => {
                    error!("Tried to get a value from an async state that is already borrowed.");
                    error!("Returning None instead of panicking");
                    error!("Error: {}", e);
                    return None;
                }
            };
            Some(x)
        } else {
            None
        }
    }
    pub fn process(&mut self) {
        let x = if let AsyncState::Loading(_, x) = self {
            let mut future = match x.try_borrow_mut() {
                Ok(x) => x,
                Err(x) => {
                    logging::error!("Could not borrow future to tick it.");
                    logging::error!(
                        "This means some other part of the code is holding on to the future."
                    );
                    logging::error!("Skipping poll for future");
                    logging::error!("Error : {x}");
                    return;
                }
            };

            let mut future = future.as_mut();
            let z = block_on(select(&mut future, futures::future::ready(())));
            match z {
                futures::future::Either::Left((done, _)) => done,
                futures::future::Either::Right((_, _)) => {
                    return;
                }
            }
        } else {
            return;
        };
        self.to_done(x);
    }
}
