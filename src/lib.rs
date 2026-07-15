//! # Multi-Errors
//!
//! An extension to the [`Result`] type to make dealing with multiple errors easier

/// A list of all errors that have occured up to this point
#[derive(Debug)]
#[must_use = "represesnts errors that occurred"]
pub struct MultiErrors<E>(Vec<E>);

impl<E> Default for MultiErrors<E> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<E> MultiErrors<E> {
    /// Create a new, empty, list of errors
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a single error to the list of errors
    pub fn add_error(&mut self, err: E) {
        self.0.push(err);
    }

    /// Get the underlying errors consuming [`Self`]
    ///
    /// If there are no errors, returns [`Ok`], otherwise returns [`Err`]
    ///
    /// ```
    /// use multi_errors::{MultiErrors, ResultExt};
    ///
    /// let mut errors = MultiErrors::<()>::new();
    ///
    /// let mut errors = MultiErrors::new();
    ///
    /// Err(1).handle_err(&mut errors, ());
    /// Err(2).handle_err(&mut errors, 3);
    /// Err(3).handle_err(&mut errors, 3);
    /// Err(4).handle_err(&mut errors, 3);
    /// assert_eq!(errors.get_errors(), [1, 2, 3, 4].as_slice());
    /// ```
    #[must_use = "errors should be handled"]
    pub fn get_errors(&self) -> &[E] {
        &self.0
    }

    /// Checks if there are any errors
    ///
    /// ```
    /// # use multi_errors::MultiErrors;
    ///
    /// let mut errors = MultiErrors::new();
    ///
    /// assert!(errors.is_ok());
    /// errors.add_error(1);
    /// assert!(!errors.is_ok());
    /// ```
    pub fn is_ok(&self) -> bool {
        self.0.is_empty()
    }

    pub fn into_result<T>(self, ok: T) -> PartialResult<T, Self> {
        if self.0.is_empty() {
            Ok(ok)
        } else {
            Err((ok, self))
        }
    }
}

/// A type alias for when a process failed but still returned a value
pub type PartialResult<T, E> = Result<T, (T, E)>;

/// Extension trait for the `Result` type to make it work better with mutliple errors
pub trait ResultExt<T, E> {
    /// Handle if there is an error:
    /// - Track the error
    /// - Calcuates the value from the provided function/closure
    ///
    /// If there is no error, returns the [`Ok`] value
    ///
    /// ```
    /// use multi_errors::{ResultExt, MultiErrors};
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct MyErr(usize);
    ///
    /// let mut errs = MultiErrors::new();
    ///
    /// assert_eq!(Ok(1).handle_err_with(&mut errs, |MyErr(n)| *n), 1);
    /// assert_eq!(Err(MyErr(10)).handle_err_with(&mut errs, |MyErr(n)| *n), 10);
    ///
    /// // The error is added to `errs`
    /// assert_eq!(errs.get_errors(), [MyErr(10)].as_slice());
    ///
    ///
    ///
    /// ```
    fn handle_err_with(self, errs: &mut MultiErrors<E>, get_default: impl FnOnce(&E) -> T) -> T;
    /// Handle if there is an error:
    /// - Track the error
    /// - Returns the provided value
    ///
    /// If there is no error, returns the [`Ok`] value
    ///
    /// This evaluates the value eagarly, if there is a lot of work, use
    /// [`ResultExt::handle_err_with`]
    ///
    /// ```
    /// use multi_errors::{MultiErrors, ResultExt};
    ///
    /// let mut errors = MultiErrors::new();
    ///
    /// assert_eq!(Ok(4).handle_err(&mut errors, 8), 4);
    /// assert_eq!(Err("an error occurred").handle_err(&mut errors, 8), 8);
    /// ```
    fn handle_err(self, errs: &mut MultiErrors<E>, default: T) -> T
    where
        Self: Sized,
    {
        self.handle_err_with(errs, |_| default)
    }
    /// Handle if there is an error:
    /// - Track the error
    /// - Returns `T::default`
    ///
    /// If there is no error, returns the [`Ok`] value
    fn handle_err_default(self, errs: &mut MultiErrors<E>) -> T
    where
        Self: Sized,
        T: Default,
    {
        self.handle_err_with(errs, |_| T::default())
    }
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn handle_err_with(self, errs: &mut MultiErrors<E>, get_default: impl FnOnce(&E) -> T) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                let res = get_default(&err);
                errs.add_error(err);
                res
            }
        }
    }
}

#[cfg(test)]
mod tests {}
