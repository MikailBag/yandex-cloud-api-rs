mod catalog;
mod service;

fn main() {
    let services = catalog::services();
    for svc in services {
        svc.emit();
    }
}
