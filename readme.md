## Debugging:
1. Install VSCode
2. Install `probe-rs` with the following command:
   - `cargo install probe-rs --features cli`
3. The rest "should" already be setup & work. You can add a new breakpoint by clicking on the left margin next to any line of code.

## Setting up debugging for a new project
To set up debugging within a new project, you can use the `launch.json` and `tasks.json` files as a template. The main important points to remember are the following:
- To debug you **must** be using a build which has no link-time optimizations or compiler optimizations. Easiest way to guarantee this is to just use a non-release build.

### Automating the build process
To automate the build process, you can use vscodes "tasks" feature. It allows you to define different tasks within the `tasks.json` file, which are just console commands. These tasks can then be invoked from launch scripts within the `launch.json` file.

## J-Link Setup Instructions:
1. Download Zedig from [here](https://zadig.akeo.ie/)
2. Install the WinUSB driver
3. (Still within Zedig) replace the J-Link devices driver with WinUSB
4. Verify your setup by running `probe-rs-cli info`. This should not report an error.
