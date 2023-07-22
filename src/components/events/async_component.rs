use std::{cell::RefCell, pin::Pin, rc::Rc};

use futures::{executor::block_on, future::select, Future};
use macroquad::{
    logging,
    prelude::{error, info},
};

use crate::Component;

pub enum AsyncState<T> {
    Unloaded,
    Loading(Rc<RefCell<Pin<Box<dyn Future<Output = T>>>>>),
    Loaded(Rc<RefCell<T>>),
}

impl<T> Clone for AsyncState<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Unloaded => Self::Unloaded,
            Self::Loading(arg0) => Self::Loading(arg0.clone()),
            Self::Loaded(arg0) => Self::Loaded(arg0.clone()),
        }
    }
}

impl<T> AsyncState<T> {
    pub fn new_loading<X: Future<Output = T> + 'static>(fut: X) -> Self {
        let z = Rc::new(RefCell::new(
            Box::pin(fut) as Pin<Box<dyn Future<Output = T>>>
        ));
        Self::Loading(z)
    }
    pub fn new_done(value: T) -> Self {
        Self::Loaded(Rc::new(RefCell::new(value)))
    }
    pub fn new_unloaded() -> Self {
        Self::Unloaded
    }
    pub fn to_loading<X: Future<Output = T> + 'static>(&mut self, fut: X) -> &mut Self {
        *self = Self::new_loading(fut);
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
        matches!(self, AsyncState::Loading(_))
    }
    pub fn is_loaded(&self) -> bool {
        matches!(self, AsyncState::Loaded(_))
    }
    pub fn is_unloaded(&self) -> bool {
        matches!(self, AsyncState::Unloaded)
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
}

pub struct AsyncExecutor;

impl<T> Component<AsyncState<T>> for AsyncExecutor {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn process(&mut self, state: &mut AsyncState<T>) {
        let x = if let AsyncState::Loading(x) = state {
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
            info!("Start the poll!");
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
        state.to_done(x);
    }

    fn render(&self, _: &AsyncState<T>) {}

    fn ui(&mut self, _: &mut macroquad::ui::Ui, _: &mut AsyncState<T>) {}
}
