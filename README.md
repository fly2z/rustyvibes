# Rustyvibes

This is a fork of rustyvibes (https://github.com/KunalBagaria/rustyvibes).
Rustyvibes is a Rust application that simulates mechanical keyboard sound effects with every key press.

https://user-images.githubusercontent.com/61944452/135816568-400c5053-8a60-4af2-b43e-e5f15d7b3d74.mp4


# Installation

You can download pre-built binaries of Rustyvibes from the [Releases](https://github.com/fly2z/rustyvibes/releases) page.

1. Clone this repository:
```
git clone https://github.com/fly2z/rustyvibes.git
cd rustyvibes
```

2. Build Rustyvibes

```
cargo build --release
```
The executable will be located at target/release/rustyvibes.

# Usage

1. Place the config.toml file in the same directory as the Rustyvibes executable.
2. Create a packs folder in the same directory.
3. Inside the packs folder, create a folder with the desired pack name. This name will be used as the pack name in the configuration.
4. In the config.toml file, set the pack name and volume level:

```
soundpack_name = "pack-name"
volume = 100
```

### Download Soundpacks: [Here](https://drive.google.com/file/d/1LQEQ9aOVQAs_wgVecXkjaA9K4LXnCdp_/view?usp=sharing)

---

### Mechvibes vs. Rustyvibes

How does Rustyvibes compare to its competitors like Mechvibes? Mechvibes uses Electron and Chromium which is very resource intensive. Rustyvibes on the other hand is made with Rust and can be upto 10x-100x more resource efficient.

Mechvibes Soundpacks: [Here](https://docs.google.com/spreadsheets/d/1PimUN_Qn3CWqfn-93YdVW8OWy8nzpz3w3me41S8S494/edit#gid=0)

Certain custom soundpacks may not work with Rustyvibes, you can use [this tool](https://github.com/kb24x7/packfixer-rustyvibes) to fix those


---


### Privacy and Permissions

Rustyvibes is a fully open-sourced project and never uses any network activity at all. macOS by default will ask you for input monitoring permissions when you start the app for the first time, if you were unable to enable it the first time, you'll need to add your default terminal you're using in the allowed input monitoring apps

![image](https://user-images.githubusercontent.com/61944452/135572648-4358c459-aa06-42e5-a347-ea4feced4efe.png)




## Contribute to this project

[![buymeacoffee](https://user-images.githubusercontent.com/61944452/135130205-4ae387f7-fb32-482e-931c-1b393588872f.png)](https://www.buymeacoffee.com/kb24x7)
