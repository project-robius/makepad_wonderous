# makepad_wonderous

Makepad version of the [Wonderous](https://flutter.gskinner.com/wonderous/) application

* work in progress
* starting with screens for China first

## 1. Setup Makepad

### Clone the Makepad repository

```bash
git clone git@github.com:makepad/makepad.git
```

### Change to latest 'rik' branch

```bash
git branch rik
```

### Install makepad subcommand for cargo

```bash
cd ~/makepad
cargo install --path ./tools/cargo_makepad
```

## 2. Get Project

### Clone the makepad_wonderous repo

```bash
git clone https://github.com/project-robius/makepad_wonderous
```

## 3. MacOS / PC

Running on Desktop is the quickest way to try out an example app.

```bash
cd ~/projects/makepad_wonderous
cargo run
```

or

```bash
cd ~/projects/makepad_wonderous
cargo run -p makepad_wonderous
```

And there should be a desktop application window now running (may need to click on the icon on MacOS's Dock to show it)

## 4. Android Build

### Install Android toolchain (First time)

```bash
rustup toolchain install nightly
cargo makepad android install-toolchain
```

### Install app on Android devivce or Android emulator

Open either the Android emulator or connect to a real Android device
use `adb` command to make sure there's a device connected properly

```bash
cd ~/projects/makepad_wonderous
cargo makepad android run -p makepad_wonderous --release
```


## 5. iOS Setup & Install

### Install IOS toolchain (First time)

```bash
rustup toolchain install nightly
cargo makepad apple ios install-toolchain
```

### Install app on Apple devivce or iOS simulator

### iOS Setup

For iOS, the process is slightly more complicated. The steps involved are:

1. Enable your iPhone's Developer Mode, please see instructions here: [Enable Developer Mode](https://www.delasign.com/blog/how-to-turn-on-developer-mode-on-an-iphone/)
1. Setup an Apple Developer account
1. Setup an empty skeleton project in XCode
    1. File -> New -> Project to create a new "App"
    1. Set the Product Name as **`makepad_wonderous`**  (used in --org later)
    1. Set the Organization Identifier to a value of your choice, for this example we will use **`rs.robius`**. (used in --app later)
    1. Setup the Project Signing & Capabilities to select the proper team account
1. In XCode, Build/Run this project to install and run the app on the simulator and device
1. Once the simulator and device has the "skeleton" app installed and running properly, then it is ready for Makepad to install its application.

### Makepad Install

We will run the `cargo makepad apple ios` command, similar to Android build above, but there are some 2 to 6 additional parameters that need to be filled in:

`--org`

First few parts of the organization identifier (which makes up the Bundle Identifier), usually in the form of **com.somecompany** or **org.someorg**

This is the same value used to setup the initial skeleton app above. For this example:
> `rs.robius`

`--app`

The name of the application or the project. This is the same as the Product Name used to setup the initial skeleton app above. In this case:
> `makepad_wonderous`

### Install app on IOS simulator

```bash
cd ~/makepad_wonderous
cargo makepad apple ios \
  --org=rs.robius \
  --app=makepad_wonderous \
  run-sim -p makepad_wonderous --release
```

### Install app on IOS device

First run the following command:

```bash
cd ~/projects/makepad
cargo makepad apple list
```

This command will print out the list of all provisioning profiles, signing identities, and device identifiers on the current system. The user has to decide and choose the ones that he/she needs to use for each type.

Once decided, run the folloiwng command and fill in the **unique starting characters** chosen from the output.

```bash
cargo makepad apple ios \
 --provisioning-profile=unique-starting-hex-string \
 --signing-identity=UNIQUE_STARTING_HEX_STRING \
 --device-identifier=UNIQUE-STARTING-HEX-STRING \
 --org=rs.robius \
 --app=makepad_wonderous \
 run-device -p makepad_wonderous –release
```

## 6. WASM Build

Running the Makepad application as a WASM build is as simple as a single command. The sript will automatically generate the necessary index.html and other files and also start a local webserver at port 8010.

### Install WASM toolchain (First time)

```bash
cargo makepad wasm install-toolchain
```

### Install app as WASM binary for browsers

```bash
cargo makepad wasm run -p makepad_wonderous --release
```

After running the command below, just open your browser to <http://127.0.0.1:8010/> in order for the app to load and run.
