use super::Result;

pub struct Remote<'repo> {
    inner: git2::Remote<'repo>,
}

impl<'repo> From<git2::Remote<'repo>> for Remote<'repo> {
    fn from(inner: git2::Remote<'repo>) -> Self {
        Self { inner }
    }
}

impl<'repo> Remote<'repo> {
    pub fn name(&self) -> Option<&str> {
        self.inner.name()
    }

    pub fn url(&self) -> Option<&str> {
        self.inner.url()
    }

    pub fn push(
        &mut self,
        refspec: &[&str],
        opts: Option<&mut git2::PushOptions<'_>>,
    ) -> Result<()> {
        self.inner.push(refspec, opts)
    }

    pub fn fetch(
        &mut self,
        refspec: &[&str],
        opts: Option<&mut git2::FetchOptions<'_>>,
    ) -> Result<()> {
        self.inner.fetch(refspec, opts, None)
    }
}