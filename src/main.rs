mod detection;
mod scoring;

fn main () {
    detection::uptime::main();
    detection::processor::main();
}