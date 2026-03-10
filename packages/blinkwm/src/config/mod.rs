use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub gaps: GapsConfig,
    pub border: BorderConfig,
    pub compositor: CompositorConfig,
    pub rules: Vec<WindowRule>,
    pub keybindings: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WindowRule {
    pub class: Option<String>,
    pub title: Option<String>,
    pub workspace: Option<usize>,
    pub floating: Option<bool>,
    pub borderless: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GapsConfig {
    pub inner: u16,
    pub outer: u16,
    pub smart_gaps: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BorderConfig {
    pub width: u16,
    pub focused: String,
    pub unfocused: String,
    pub smart_borders: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CompositorConfig {
    pub enabled: bool,
    pub shadow: bool,
    pub fading: bool,
    pub corner_radius: u16,
    pub blur_method: String,
    pub backend: String,
}

impl Config {
    pub fn load() -> Self {
        let config_path = dirs::config_dir()
            .map(|p| p.join("blinkwm/config.toml"))
            .unwrap_or_else(|| std::path::PathBuf::from("/etc/blinkwm/config.toml"));

        if let Ok(content) = std::fs::read_to_string(&config_path) {
            if let Ok(config) = toml::from_str(&content) {
                return config;
            }
        }

        Self::default()
    }

    pub fn generate_picom_conf(&self) -> String {
        format!(
            r#"backend = "{}";
vsync = true;
shadow = {};
fading = {};
corner-radius = {};
blur: {{
  method = "{}";
  strength = 5;
}};
rounded-corners-exclude = [
  "window_type = 'dock'",
  "window_type = 'desktop'"
];
"#,
            self.compositor.backend,
            self.compositor.shadow,
            self.compositor.fading,
            self.compositor.corner_radius,
            self.compositor.blur_method
        )
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut keybindings = HashMap::new();
        keybindings.insert("Mod4+Return".to_string(), "terminal".to_string());
        keybindings.insert("Mod4+q".to_string(), "close".to_string());
        keybindings.insert("Mod4+d".to_string(), "launcher".to_string());
        keybindings.insert("Mod4+space".to_string(), "next_layout".to_string());
        for i in 1..=9 {
            keybindings.insert(format!("Mod4+{}", i), format!("workspace_{}", i));
        }

        Self {
            gaps: GapsConfig {
                inner: 10,
                outer: 10,
                smart_gaps: true,
            },
            border: BorderConfig {
                width: 2,
                focused: "#007AFF".to_string(),
                unfocused: "#3D3D3D".to_string(),
                smart_borders: true,
            },
            compositor: CompositorConfig {
                enabled: true,
                shadow: true,
                fading: true,
                corner_radius: 12,
                blur_method: "dual_kawase".to_string(),
                backend: "glx".to_string(),
            },
            rules: Vec::new(),
            keybindings,
        }
    }
}
