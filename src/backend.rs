use buckets::Buckets;
use backends::console;
use backends::wavefront;
use backends::librato;

/// A 'backend' is a sink for metrics.
pub trait Backend {
    fn flush(&mut self, buckets: &Buckets) -> ();
}


/// Creates the collection of backends based on the paraemeters
///
pub fn factory(console: &bool,
               wavefront: &bool,
               librato: &bool,
               metric_source: &str,
               wavefront_host: &str,
               wavefront_port: &u16,
               librato_username: &str,
               librato_token: &str)
               -> Box<[Box<Backend>]> {
    let mut backends: Vec<Box<Backend>> = Vec::with_capacity(3);
    if *console {
        backends.push(Box::new(console::Console::new()));
    }
    if *wavefront {
        backends.push(Box::new(wavefront::Wavefront::new(wavefront_host,
                                                         *wavefront_port,
                                                         metric_source)));
    }
    if *librato {
        backends.push(Box::new(librato::Librato::new(librato_username, librato_token,
                                                     metric_source)));
    }
    backends.into_boxed_slice()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn factory_makes_wavefront() {
        let backends = factory(&false,
                               &false,
                               &true,
                               &false,
                               "127.0.0.1",
                               &2300,
                               "127.0.0.1",
                               &2878,
                               "src",
                               "username",
                               "token",
                               "src");
        assert_eq!(1, backends.len());
    }

    #[test]
    fn factory_makes_librato() {
        let backends = factory(&false,
                               &false,
                               &false,
                               &true,
                               "127.0.0.1",
                               &2300,
                               "127.0.0.1",
                               &2878,
                               "src",
                               "username",
                               "token",
                               "src");
        assert_eq!(1, backends.len());
    }

    #[test]
    fn factory_makes_console() {
        let backends = factory(&true,
                               &false,
                               &false,
                               &false,
                               "127.0.0.1",
                               &2300,
                               "127.0.0.1",
                               &2878,
                               "src",
                               "username",
                               "token",
                               "src");
        assert_eq!(1, backends.len());
    }

    #[test]
    fn factory_makes_all() {
        let backends = factory(&true,
                               &true,
                               &true,
                               &true,
                               "127.0.0.1",
                               &2300,
                               "127.0.0.1",
                               &2878,
                               "src",
                               "username",
                               "token",
                               "src");
        assert_eq!(4, backends.len());
    }
}
