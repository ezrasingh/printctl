mod devices;

use devices::list_devices;

fn main() {
    let devices = list_devices();
    println!("{:#?}", devices);
}
