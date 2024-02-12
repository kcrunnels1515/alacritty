use std::fs::File;
use std::path::Path;
use std::io::{Error,Write};
use crate::cli::GenConfigOptions;
use crate::config;

/// Handle migration.
pub fn genconf(options: GenConfigOptions) -> Result<(), Error> {
    let config_text = "
# Additional TOML config files
# Format -> [\"<path>\", ...]
import = []

# shell can either be set directly to a string or
# as shown with two params
[shell]
program = \"$SHELL\"
args = \"\"

# Directory for the shell to start in
working_directory = \"None\"

# Enable live reloading of config file
live_config_reload = true

# Offer IPC using alacritty msg
ipc_socket = true

# All key-value pairs will be added as environment variables
# for any process spawned by alacritty, including the shell
[env]
HELLO_WORLD_VAR = \"Hello world!\"

# Settings related to the alacritty window
[window]
# Must be non-zero to take effect
[dimensions]
columns = 0
lines = 0

# Set as either a None value or the
# pair { x = <integer>, y = <integer> }
postion = \"None\"

# Blank space added around the window in pixels,
# scaled by DPI.
[padding]
x = 0
y = 0

# Spreads additional padding evenly around terminal content
dynamic_padding = false

# Window decorations
# Values: \"Full\" | \"None\" | \"Transparent\" | \"Buttonless\"
decorations = \"Full\"

# Background opacity as a float
opacity = 1.0

# Request compositor to blur content behind transparent
# windows, works on macOS/KDE wayland
blur = false

# Startup mode (requires restart)
# Values: \"Windowed\"  |  \"Maximized\"  |  \"Fullscreen\"  |  \"Simple‐Fullscreen\"
startup_mode = \"Windowed\"

# Title
title = \"Alacritty\"

# Allow terminal applications to change Alacritty's title
dynamic_title = true

# Window class
[class]
instance = \"Alacritty\"
general = \"Alacritty\"

# Override the variant of the System theme:
# \"Dark\" | \"Light\" | \"None\"
decorations_theme_variant = \"None\"

# Prefer resizing window by discrete steps equal
# to cell dimensions
resize_increments = false

# Make Option key behave as specified Alt:
# \"OnlyLeft\" | \"OnlyRight\" | \"Both\" | \"None\"
option_as_alt = \"None\"
";

    if options.dry_run {
        println!("{config_text}");
        return Ok(());
    }
    // Find configuration file path.
    let config_path = options
        .config_file
        .clone()
        .or_else(|| config::installed_config("toml"));

    // Abort if system has no installed configuration.
    //match config_path {
    //    Some(config_path) => todo!(),
    //    None => {
    //        eprintln!("No configuration file found");
    //        std::process::exit(1);
    //    },
    //};


    // If we're doing a wet run, perform a dry run first for safety.
    if !options.dry_run {
        println!("{config_text}")
    }

    // Migrate the root config.
    let output = File::create(config_path.clone().unwrap());
    match output {
        Ok(mut file_handle) => {
            write!(file_handle, "{}", config_text);
            std::process::exit(0);
        },
        Err(err) => {
            eprintln!("Configuration file generation failed:");
            eprintln!("    {config_path:?}: {err}");
            std::process::exit(1);
        },
    }
}
