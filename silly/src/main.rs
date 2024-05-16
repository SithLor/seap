trait Plugin {
    fn run(&self);
    fn exit(&self);
}

struct MyPlugin;

impl Plugin for MyPlugin {
    fn run(&self) {
        println!("MyPlugin run");
    }
    fn exit(&self) {
        println!("MyPlugin exit");
    }
}

impl Drop for MyPlugin {
    fn drop(&mut self) {
        self.exit();
    }
}


fn main() {
    let my_plugin: MyPlugin = MyPlugin;
    my_plugin.run();
    // my_plugin will be dropped here, and the exit method will be called
}