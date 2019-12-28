use super::Mirror;
use super::Protocols;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Mirrors {
    cutoff: u64,
    last_check: String,
    num_checks: u64,
    check_frequency: u64,
    urls: Vec<Mirror>,
    version: u64,
}

impl<'a> Mirrors {
    pub fn order_by<F>(&'a mut self, order: F) -> &'a mut Self
    where
        F: FnMut(&Mirror, &Mirror) -> std::cmp::Ordering,
    {
        self.urls.sort_by(order);
        self
    }

    pub fn filter_protocols(&'a mut self, p: Protocols) -> &'a mut Self
    {
        self.urls.retain(|url| url.protocol.intercects(p));
        self
    }

    pub fn protocols<F>(&'a mut self, protocols: F) -> &'a mut Self
    where
        F: Fn(&Protocols) -> bool,
    {
        let urls = &mut self.urls;
        for i in 0..urls.len() {
            if protocols(&mut urls[i].protocol) {
                urls.remove(i);
            }
        }
        self
    }
}

impl IntoIterator for Mirrors {
    type Item = Mirror;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.urls.into_iter()
    }
}
