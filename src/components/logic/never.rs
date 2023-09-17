use crate::Component;

pub struct Never;

impl<T: Clone, X> Component<T, X> for Never {
    type Input = ();

    fn instantiate(_: Self::Input) -> Self
    where
        Self: Sized,
    {
        Self
    }
}
