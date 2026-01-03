use proc_macro0::Span;

fn main() {
    fn requires_send<T: Send>() {}
    requires_send::<Span>();
}
