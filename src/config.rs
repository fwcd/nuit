use nuit_core::View;

use crate::Backend;

pub struct Config<T> {
    view: T,
    preferred_backend: Option<Backend>,
}

impl<T> Config<T> {
    pub fn builder(view: T) -> ConfigBuilder<T> {
        view.into()
    }

    pub fn preferred_backend(&self) -> Option<Backend> {
        self.preferred_backend
    }

    pub fn into_view(self) -> T {
        self.view
    }
}

pub struct ConfigBuilder<T> {
    view: T,
    preferred_backend: Option<Backend>,
}

impl<T> ConfigBuilder<T> {
    pub fn preferred_backend(mut self, backend: Backend) -> Self {
        self.preferred_backend = Some(backend);
        self
    }
}

impl<T> From<T> for ConfigBuilder<T> {
    fn from(view: T) -> Self {
        Self {
            view,
            preferred_backend: None,
        }
    }
}

impl<T> From<ConfigBuilder<T>> for Config<T> {
    fn from(builder: ConfigBuilder<T>) -> Self {
        Self {
            view: builder.view,
            preferred_backend: builder.preferred_backend,
        }
    }
}

impl<T> From<T> for Config<T> where T: View {
    fn from(view: T) -> Self {
        Self::builder(view).into()
    }
}
