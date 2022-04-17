mod catalog;
mod service;
#[cfg(test)]
mod tests;

fn main() {
    let services = catalog::services();
    for svc in services {
        svc.emit();
    }
}
