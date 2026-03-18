use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingerprintPreset {
    pub user_agent: String,
    pub platform: String,
    pub vendor: String,
    pub language: String,
    pub timezone: String,
    pub screen_resolution: (u32, u32),
    pub hardware_concurrency: u32,
}

pub struct StealthPatch;

impl StealthPatch {
    pub fn generate_random_fingerprint() -> FingerprintPreset {
        let mut rng = rand::thread_rng();

        let user_agents = vec![
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        ];

        let platforms = vec!["MacIntel", "Win32", "Linux x86_64"];
        let resolutions = vec![(1920, 1080), (2560, 1440), (1366, 768), (1440, 900)];

        let idx = rng.gen_range(0..user_agents.len());

        FingerprintPreset {
            user_agent: user_agents[idx].to_string(),
            platform: platforms[idx].to_string(),
            vendor: "Google Inc.".to_string(),
            language: "zh-CN,zh;q=0.9,en;q=0.8".to_string(),
            timezone: "Asia/Shanghai".to_string(),
            screen_resolution: resolutions[rng.gen_range(0..resolutions.len())],
            hardware_concurrency: rng.gen_range(4..=16),
        }
    }

    pub fn generate_stealth_script(preset: &FingerprintPreset) -> String {
        format!(
            r#"
            // Override navigator properties
            Object.defineProperty(navigator, 'webdriver', {{
                get: () => undefined
            }});

            Object.defineProperty(navigator, 'platform', {{
                get: () => '{}'
            }});

            Object.defineProperty(navigator, 'vendor', {{
                get: () => '{}'
            }});

            Object.defineProperty(navigator, 'hardwareConcurrency', {{
                get: () => {}
            }});

            Object.defineProperty(navigator, 'languages', {{
                get: () => ['zh-CN', 'zh', 'en']
            }});

            // Override screen properties
            Object.defineProperty(screen, 'width', {{
                get: () => {}
            }});

            Object.defineProperty(screen, 'height', {{
                get: () => {}
            }});

            // Block WebRTC IP leak
            const originalRTCPeerConnection = window.RTCPeerConnection;
            window.RTCPeerConnection = function(...args) {{
                throw new Error('WebRTC is disabled');
            }};

            // Canvas fingerprint randomization
            const originalToDataURL = HTMLCanvasElement.prototype.toDataURL;
            HTMLCanvasElement.prototype.toDataURL = function(type) {{
                const context = this.getContext('2d');
                if (context) {{
                    const imageData = context.getImageData(0, 0, this.width, this.height);
                    for (let i = 0; i < imageData.data.length; i += 4) {{
                        imageData.data[i] = imageData.data[i] ^ Math.floor(Math.random() * 3);
                    }}
                    context.putImageData(imageData, 0, 0);
                }}
                return originalToDataURL.apply(this, arguments);
            }};

            // WebGL fingerprint obfuscation
            const getParameter = WebGLRenderingContext.prototype.getParameter;
            WebGLRenderingContext.prototype.getParameter = function(parameter) {{
                if (parameter === 37445) {{
                    return 'Intel Inc.';
                }}
                if (parameter === 37446) {{
                    return 'Intel Iris OpenGL Engine';
                }}
                return getParameter.apply(this, arguments);
            }};

            // Audio context fingerprint
            const AudioContext = window.AudioContext || window.webkitAudioContext;
            if (AudioContext) {{
                const originalCreateOscillator = AudioContext.prototype.createOscillator;
                AudioContext.prototype.createOscillator = function() {{
                    const oscillator = originalCreateOscillator.apply(this, arguments);
                    const originalStart = oscillator.start;
                    oscillator.start = function(when) {{
                        return originalStart.apply(this, [when + Math.random() * 0.0001]);
                    }};
                    return oscillator;
                }};
            }}

            // Remove automation indicators
            delete navigator.__proto__.webdriver;

            // Chrome runtime
            window.chrome = {{
                runtime: {{}}
            }};

            // Permissions
            const originalQuery = window.navigator.permissions.query;
            window.navigator.permissions.query = (parameters) => (
                parameters.name === 'notifications' ?
                    Promise.resolve({{ state: Notification.permission }}) :
                    originalQuery(parameters)
            );

            console.log('Stealth patches applied');
            "#,
            preset.platform,
            preset.vendor,
            preset.hardware_concurrency,
            preset.screen_resolution.0,
            preset.screen_resolution.1
        )
    }
}
