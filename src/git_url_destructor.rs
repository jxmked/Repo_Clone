
struct GitUrlDestructor {
  priv_url: String
}

impl GitUrlDestructor {
  fn new(&mut self, url: String) {
    self.priv_url = url
  }
}