// Material You Theme — sistema de tema completo para Forja GUI
//
// Este módulo implementa el sistema de theming Material Design 3 (Material You)
// con colores dinámicos (Monet), tipografía, formas, elevación, estados, motion y animaciones.

pub mod animation;
pub mod color;
pub mod dynamic_color;
pub mod elevation;
pub mod motion;
pub mod palette;
pub mod scheme;
pub mod shape;
pub mod state;
pub mod system_theme;
pub mod typography;

pub use animation::*;
pub use color::*;
pub use dynamic_color::*;
pub use elevation::*;
pub use motion::*;
pub use palette::*;
pub use scheme::*;
pub use shape::*;
pub use state::*;
pub use system_theme::*;
pub use typography::*;

/// Diseño visual base del tema
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DesignStyle {
    /// Material You (Material Design 3, colores dinámicos)
    #[default]
    MaterialYou,
    /// Liquid Glass (estilo Apple: superficies vidriosas, esquinas muy redondeadas)
    LiquidGlass,
    /// Fluent (estilo Windows 11: Mica, esquinas de 4-8px, acento azul)
    Fluent,
}

/// Tema Material You completo con todos los subsistemas
#[derive(Clone, Debug)]
pub struct MaterialTheme {
    /// Esquema de color (roles de color light/dark)
    pub scheme: ColorScheme,
    /// Escala tipográfica (15 estilos)
    pub typography: TypeScale,
    /// Sistema de formas (radios de borde)
    pub shapes: ShapeSystem,
    /// Sistema de elevación (sombras)
    pub elevation: ElevationSystem,
    /// Sistema de movimiento (animaciones)
    pub motion: MotionSystem,
    /// Indica si es modo oscuro
    pub is_dark: bool,
    /// Color semilla usado para generar el tema
    pub seed_color: String,
    /// Diseño visual base (Material You, Liquid Glass o Fluent)
    pub design: DesignStyle,
}

impl MaterialTheme {
    /// Crea un tema desde un color semilla (hex o nombre)
    ///
    /// # Argumentos
    /// - `seed_color`: color semilla en formato hex (#RRGGBB) o nombre en español
    /// - `is_dark`: `true` para modo oscuro, `false` para modo claro
    ///
    /// # Ejemplo
    /// ```ignore
    /// let tema = MaterialTheme::from_seed("#FF5722", false);
    /// let tema_oscuro = MaterialTheme::from_seed("rojo", true);
    /// ```
    pub fn from_seed(seed_color: &str, is_dark: bool) -> Self {
        // Parsear el color semilla
        let seed: RgbColor = if let Some(color) = color::nombre_a_color(seed_color) {
            color
        } else {
            RgbColor::from_hex(seed_color).unwrap_or(RgbColor(103, 80, 164))
        };

        // Generar paletas desde el seed usando algoritmo Monet
        let palettes = Palettes::from_seed(&seed);

        // Crear esquema de color
        let scheme = ColorScheme::from_palettes(&palettes, is_dark);

        MaterialTheme {
            scheme,
            typography: TypeScale::default(),
            shapes: ShapeSystem::default(),
            elevation: ElevationSystem::default(),
            motion: MotionSystem::default(),
            is_dark,
            seed_color: seed.to_hex(),
            design: DesignStyle::MaterialYou,
        }
    }

    /// Crea un tema por nombre de diseño o color semilla
    ///
    /// Nombres de diseño reconocidos:
    /// - `"liquid_glass"`, `"vidrio"`, `"glass"` → [`Self::liquid_glass`]
    /// - `"fluent"`, `"windows11"`, `"win11"` → [`Self::fluent`]
    ///
    /// Sufijos `:dark` / `:oscuro` fuerzan el modo oscuro.
    /// Cualquier otro valor se trata como color semilla (`from_seed`).
    pub fn named(name: &str, is_dark: bool) -> Self {
        let lower = name.trim().to_lowercase();
        let (base, modo) = match lower.split_once(':') {
            Some((b, mode)) => (b, Some(mode)),
            None => (lower.as_str(), None),
        };
        // El sufijo manda sobre el modo heredado: `tema_fluent` es claro por
        // definición, `tema_fluent_oscuro` es oscuro, sin importar el SO.
        let dark = match modo {
            Some("dark") | Some("oscuro") => true,
            Some("light") | Some("claro") => false,
            _ => is_dark,
        };
        match base {
            "liquid_glass" | "liquidglass" | "vidrio" | "vidrio_liquido" | "glass" => {
                Self::liquid_glass(dark)
            }
            "fluent" | "windows11" | "windows_11" | "win11" => Self::fluent(dark),
            _ => Self::from_seed(name, is_dark),
        }
    }

    /// Diseño Liquid Glass (estilo Apple): superficies claras vidriosas,
    /// acento azul iOS (#007AFF / #0A84FF) y esquinas muy redondeadas (20-32px)
    pub fn liquid_glass(is_dark: bool) -> Self {
        let mut theme = MaterialTheme::from_seed("#007AFF", is_dark);
        let s = &mut theme.scheme;
        if is_dark {
            s.background = RgbColor(10, 12, 16);
            s.surface = RgbColor(16, 19, 24);
            s.surface_variant = RgbColor(28, 32, 40);
            s.on_surface = RgbColor(235, 240, 248);
            s.on_background = RgbColor(235, 240, 248);
            s.on_surface_variant = RgbColor(180, 192, 208);
            s.primary = RgbColor(10, 132, 255); // #0A84FF
            s.on_primary = RgbColor(255, 255, 255);
            s.primary_container = RgbColor(0, 62, 126);
            s.on_primary_container = RgbColor(214, 232, 255);
            s.secondary = RgbColor(100, 210, 255); // #64D2FF
            s.tertiary = RgbColor(191, 90, 242); // #BF5AF2
            s.outline = RgbColor(60, 66, 76);
            s.outline_variant = RgbColor(38, 43, 52);
        } else {
            s.background = RgbColor(242, 246, 252); // blanco vidrioso azulado
            s.surface = RgbColor(250, 252, 255);
            s.surface_variant = RgbColor(228, 236, 248);
            s.on_surface = RgbColor(15, 20, 28);
            s.on_background = RgbColor(15, 20, 28);
            s.on_surface_variant = RgbColor(90, 104, 122);
            s.primary = RgbColor(0, 122, 255); // #007AFF
            s.on_primary = RgbColor(255, 255, 255);
            s.primary_container = RgbColor(209, 231, 255);
            s.on_primary_container = RgbColor(0, 48, 108);
            s.secondary = RgbColor(90, 200, 250); // #5AC8FA
            s.tertiary = RgbColor(175, 82, 222); // #AF52DE
            s.outline = RgbColor(176, 190, 208);
            s.outline_variant = RgbColor(214, 224, 238);
        }
        theme.shapes = ShapeSystem {
            medium: 20.0,
            large: 24.0,
            extra_large: 32.0,
            button: 24.0,
            ..ShapeSystem::default()
        };
        theme.design = DesignStyle::LiquidGlass;
        theme.seed_color = if is_dark { "#0A84FF" } else { "#007AFF" }.to_string();
        theme
    }

    /// Diseño Fluent (estilo Windows 11): fondos Mica (#F3F3F3 / #202020),
    /// acento azul Windows (#005FB8 / #4CC2FF) y esquinas de 4-8px
    pub fn fluent(is_dark: bool) -> Self {
        let mut theme = MaterialTheme::from_seed("#0078D4", is_dark);
        let s = &mut theme.scheme;
        if is_dark {
            s.background = RgbColor(32, 32, 32); // Mica oscuro
            s.surface = RgbColor(40, 40, 40);
            s.surface_variant = RgbColor(51, 51, 51);
            s.on_surface = RgbColor(255, 255, 255);
            s.on_background = RgbColor(255, 255, 255);
            s.on_surface_variant = RgbColor(204, 204, 204);
            s.primary = RgbColor(76, 194, 255); // #4CC2FF (acento oscuro)
            s.on_primary = RgbColor(0, 51, 78);
            s.secondary = RgbColor(108, 180, 238); // #6CB4EE
            s.tertiary = RgbColor(76, 194, 176); // #4CC2B0
            s.primary_container = RgbColor(0, 67, 119);
            s.on_primary_container = RgbColor(204, 232, 255);
            s.error = RgbColor(255, 153, 164);
            s.on_error = RgbColor(68, 0, 12);
            s.outline = RgbColor(118, 118, 118);
            s.outline_variant = RgbColor(68, 68, 68);
        } else {
            s.background = RgbColor(243, 243, 243); // Mica claro
            s.surface = RgbColor(255, 255, 255);
            s.surface_variant = RgbColor(249, 249, 249);
            s.on_surface = RgbColor(27, 27, 27);
            s.on_background = RgbColor(27, 27, 27);
            s.on_surface_variant = RgbColor(95, 95, 95);
            s.primary = RgbColor(0, 95, 184); // #005FB8 (acento claro)
            s.on_primary = RgbColor(255, 255, 255);
            s.secondary = RgbColor(0, 103, 192); // #0067C0
            s.tertiary = RgbColor(3, 131, 135); // #038387 (teal)
            s.primary_container = RgbColor(229, 241, 251);
            s.on_primary_container = RgbColor(0, 29, 54);
            s.error = RgbColor(196, 43, 28); // #C42B1C
            s.on_error = RgbColor(255, 255, 255);
            s.outline = RgbColor(117, 117, 117);
            s.outline_variant = RgbColor(209, 209, 209);
        }
        // Fluent 2: ControlCornerRadius = 4, OverlayCornerRadius = 8.
        theme.shapes = ShapeSystem {
            none: 0.0,
            extra_small: 4.0,
            small: 4.0,
            medium: 4.0,
            large: 8.0,
            extra_large: 8.0,
            full: -1.0,
            button: 4.0,
        };
        theme.design = DesignStyle::Fluent;
        theme.seed_color = if is_dark { "#4CC2FF" } else { "#005FB8" }.to_string();
        theme
    }

    /// Crea un tema claro por defecto (seed: #6750A4 — púrpura)
    pub fn light() -> Self {
        MaterialTheme::from_seed(color::SEED_DEFAULT, false)
    }

    /// Crea un tema oscuro por defecto (seed: #6750A4 — púrpura)
    pub fn dark() -> Self {
        MaterialTheme::from_seed(color::SEED_DEFAULT, true)
    }

    /// Crea un tema dinámico que detecta automáticamente claro/oscuro
    ///
    /// NOTA: La detección automática del modo del sistema requiere
    /// acceso a `winit::window::Theme`. Si no está disponible,
    /// devuelve modo claro.
    pub fn dynamic(seed_color: &str) -> Self {
        // Por defecto usamos modo claro
        // En el futuro, aquí se integrará detección del theme del sistema
        MaterialTheme::from_seed(seed_color, false)
    }

    /// Crea un tema que sigue la preferencia del sistema operativo
    ///
    /// Detecta automáticamente si el sistema está en modo oscuro
    /// (Windows: registro, Linux: gsettings, macOS: defaults)
    /// y aplica el tema correspondiente.
    ///
    /// Si no se puede detectar, fallback a modo claro.
    pub fn system(seed_color: &str) -> Self {
        Self::from_seed(seed_color, is_system_dark())
    }

    /// Cambia entre modo claro y oscuro, preservando el diseño
    pub fn toggle_dark_mode(&self) -> Self {
        match self.design {
            DesignStyle::LiquidGlass => MaterialTheme::liquid_glass(!self.is_dark),
            DesignStyle::Fluent => MaterialTheme::fluent(!self.is_dark),
            DesignStyle::MaterialYou => MaterialTheme::from_seed(&self.seed_color, !self.is_dark),
        }
    }

    /// Crea un tema con un nuevo color semilla, manteniendo el modo
    pub fn with_seed(&self, seed_color: &str) -> Self {
        MaterialTheme::from_seed(seed_color, self.is_dark)
    }
}

impl Default for MaterialTheme {
    fn default() -> Self {
        MaterialTheme::light()
    }
}

impl std::fmt::Display for MaterialTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MaterialTheme(seed={}, dark={})",
            self.seed_color, self.is_dark
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_light() {
        let theme = MaterialTheme::light();
        assert!(!theme.is_dark);
        assert_eq!(theme.seed_color, "#6750A4");
        assert_eq!(theme.scheme.primary, RgbColor(103, 80, 164));
    }

    #[test]
    fn test_theme_dark() {
        let theme = MaterialTheme::dark();
        assert!(theme.is_dark);
        // El primary dark es tone 80 de la paleta primaria simplificada
        // Puede diferir ligeramente del valor oficial de Material You
        // debido a la implementación HCT simplificada
        assert_ne!(theme.scheme.primary, RgbColor(0, 0, 0));
        assert!(theme.scheme.primary.0 > 200); // Debe ser claro
        assert!(theme.scheme.primary.2 > 200); // Debe tener componente azul
    }

    #[test]
    fn test_theme_from_seed_hex() {
        let theme = MaterialTheme::from_seed("#FF5722", false);
        assert_eq!(theme.seed_color.to_uppercase(), "#FF5722");
    }

    #[test]
    fn test_theme_from_seed_name() {
        let theme = MaterialTheme::from_seed("rojo", false);
        assert_eq!(theme.seed_color, "#F44336".to_string());
    }

    #[test]
    fn test_theme_toggle() {
        let light = MaterialTheme::light();
        let dark = light.toggle_dark_mode();
        assert!(dark.is_dark);
        assert_eq!(dark.seed_color, light.seed_color);
        let back_to_light = dark.toggle_dark_mode();
        assert!(!back_to_light.is_dark);
    }

    #[test]
    fn test_theme_default() {
        let theme = MaterialTheme::default();
        assert!(!theme.is_dark);
        assert_eq!(theme.seed_color, "#6750A4");
    }

    #[test]
    fn test_all_subsystems_present() {
        let theme = MaterialTheme::light();
        // Verificar que todos los subsistemas están accesibles
        let _ = theme.typography.body_medium;
        let _ = theme.shapes.small;
        let _ = theme.elevation.level1;
        let _ = theme.motion.durations.duration_300;
    }

    #[test]
    fn test_liquid_glass() {
        let claro = MaterialTheme::liquid_glass(false);
        assert!(!claro.is_dark);
        assert_eq!(claro.design, DesignStyle::LiquidGlass);
        assert_eq!(claro.scheme.primary, RgbColor(0, 122, 255));
        assert_eq!(claro.shapes.large, 24.0);
        let oscuro = MaterialTheme::liquid_glass(true);
        assert!(oscuro.is_dark);
        assert_eq!(oscuro.scheme.primary, RgbColor(10, 132, 255));
    }

    #[test]
    fn test_fluent() {
        let claro = MaterialTheme::fluent(false);
        assert!(!claro.is_dark);
        assert_eq!(claro.design, DesignStyle::Fluent);
        assert_eq!(claro.scheme.primary, RgbColor(0, 95, 184));
        assert_eq!(claro.scheme.background, RgbColor(243, 243, 243));
        assert_eq!(claro.shapes.small, 4.0);
        assert_eq!(claro.shapes.button, 4.0);
        let oscuro = MaterialTheme::fluent(true);
        assert!(oscuro.is_dark);
        assert_eq!(oscuro.scheme.primary, RgbColor(76, 194, 255));
        assert_eq!(oscuro.scheme.background, RgbColor(32, 32, 32));
    }

    #[test]
    fn test_named() {
        assert_eq!(MaterialTheme::named("fluent", false).design, DesignStyle::Fluent);
        assert_eq!(
            MaterialTheme::named("liquid_glass", false).design,
            DesignStyle::LiquidGlass
        );
        assert_eq!(MaterialTheme::named("vidrio", false).design, DesignStyle::LiquidGlass);
        let oscuro = MaterialTheme::named("fluent:dark", false);
        assert!(oscuro.is_dark);
        assert_eq!(oscuro.design, DesignStyle::Fluent);
        // Un color semilla sigue funcionando como Material You
        let seed = MaterialTheme::named("#FF5722", false);
        assert_eq!(seed.design, DesignStyle::MaterialYou);
        assert_eq!(seed.seed_color.to_uppercase(), "#FF5722");
    }

    #[test]
    fn test_toggle_preserva_diseno() {
        let fluent = MaterialTheme::fluent(false).toggle_dark_mode();
        assert!(fluent.is_dark);
        assert_eq!(fluent.design, DesignStyle::Fluent);
    }

    #[test]
    fn test_with_seed() {
        let original = MaterialTheme::light();
        let nuevo = original.with_seed("#4CAF50");
        assert_eq!(nuevo.seed_color, "#4CAF50");
        assert!(!nuevo.is_dark);
        assert_ne!(nuevo.scheme.primary, original.scheme.primary);
    }
}
