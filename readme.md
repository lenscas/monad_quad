# Monad Quad

Monad quad is a node based, scene tree system built on top of Macroquad.

Unlike normal nodes system, monad quad uses a single state for the entire application that nodes split up, update and use for rendering rather than each node being responsible for their own state.

This makes it _very_ easy to do things like resetting, loading and saving state as everything is easily accessible.

Also, because nodes end up in full control over the signals their children get, pausing updates while still allowing things to be rendered is also really easy. In fact, there is even a node for just that.
