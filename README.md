# Hyprland_switchmons

Basic hyprland program to switch windows between monitors(Only two for now)

Add your .local/bin to your PATH

```bash
cargo build --release
cp ./target/release/hyprland_switchmons .local/bin
```

----------
### Into your hyprland config file bind the program
Example:

```
# Change monitors
bind = $mainMod SHIFT, TAB, exec, $HOME/.local/bin/hyprland_switchmons
```
