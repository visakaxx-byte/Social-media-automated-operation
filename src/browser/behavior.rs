use chromiumoxide::Page;
use anyhow::Result;
use rand::Rng;
use std::time::Duration;
use tokio::time::sleep;

pub struct HumanBehavior;

impl HumanBehavior {
    /// Simulate human-like mouse movement using Bezier curve
    pub async fn move_mouse_to(page: &Page, x: f64, y: f64) -> Result<()> {
        // Get current mouse position (simplified - in real implementation would track state)
        let start_x = 0.0;
        let start_y = 0.0;

        // Generate Bezier curve points
        let points = Self::generate_bezier_curve(start_x, start_y, x, y, 20);

        for point in points {
            let script = format!(
                r#"
                const event = new MouseEvent('mousemove', {{
                    clientX: {},
                    clientY: {},
                    bubbles: true
                }});
                document.dispatchEvent(event);
                "#,
                point.0, point.1
            );
            page.evaluate(script.as_str()).await?;

            // Random delay between movements
            let delay = rand::thread_rng().gen_range(5..15);
            sleep(Duration::from_millis(delay)).await;
        }

        Ok(())
    }

    /// Generate Bezier curve points for smooth mouse movement
    fn generate_bezier_curve(
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
        steps: usize,
    ) -> Vec<(f64, f64)> {
        let mut rng = rand::thread_rng();
        let mut points = Vec::new();

        // Control points for Bezier curve
        let cp1_x = start_x + (end_x - start_x) * 0.25 + rng.gen_range(-50.0..50.0);
        let cp1_y = start_y + (end_y - start_y) * 0.25 + rng.gen_range(-50.0..50.0);
        let cp2_x = start_x + (end_x - start_x) * 0.75 + rng.gen_range(-50.0..50.0);
        let cp2_y = start_y + (end_y - start_y) * 0.75 + rng.gen_range(-50.0..50.0);

        for i in 0..=steps {
            let t = i as f64 / steps as f64;
            let t2 = t * t;
            let t3 = t2 * t;
            let mt = 1.0 - t;
            let mt2 = mt * mt;
            let mt3 = mt2 * mt;

            let x = start_x * mt3 + 3.0 * cp1_x * mt2 * t + 3.0 * cp2_x * mt * t2 + end_x * t3;
            let y = start_y * mt3 + 3.0 * cp1_y * mt2 * t + 3.0 * cp2_y * mt * t2 + end_y * t3;

            points.push((x, y));
        }

        points
    }

    /// Simulate human-like typing with random delays
    pub async fn type_text(page: &Page, selector: &str, text: &str) -> Result<()> {
        let mut rng = rand::thread_rng();

        for ch in text.chars() {
            let script = format!(
                r#"
                const element = document.querySelector('{}');
                if (element) {{
                    element.value += '{}';
                    element.dispatchEvent(new Event('input', {{ bubbles: true }}));
                }}
                "#,
                selector,
                ch.escape_default()
            );
            page.evaluate(script.as_str()).await?;

            // Random typing delay (50-150ms)
            let delay = rng.gen_range(50..150);
            sleep(Duration::from_millis(delay)).await;
        }

        Ok(())
    }

    /// Random scroll with human-like behavior
    pub async fn random_scroll(page: &Page) -> Result<()> {
        let mut rng = rand::thread_rng();

        let scroll_amount = rng.gen_range(100..500);
        let scroll_steps = rng.gen_range(5..15);

        for _ in 0..scroll_steps {
            let step = scroll_amount / scroll_steps;
            let script = format!("window.scrollBy(0, {})", step);
            page.evaluate(script.as_str()).await?;

            let delay = rng.gen_range(50..200);
            sleep(Duration::from_millis(delay)).await;
        }

        // Random pause after scrolling
        let pause = rng.gen_range(500..2000);
        sleep(Duration::from_millis(pause)).await;

        Ok(())
    }

    /// Random delay to simulate human reading/thinking time
    pub async fn random_delay(min_ms: u64, max_ms: u64) {
        let mut rng = rand::thread_rng();
        let delay = rng.gen_range(min_ms..max_ms);
        sleep(Duration::from_millis(delay)).await;
    }

    /// Simulate clicking with human-like behavior
    pub async fn click_element(page: &Page, selector: &str) -> Result<()> {
        // Move mouse to element first
        let script = format!(
            r#"
            const element = document.querySelector('{}');
            if (element) {{
                const rect = element.getBoundingClientRect();
                const x = rect.left + rect.width / 2;
                const y = rect.top + rect.height / 2;
                [x, y]
            }}
            "#,
            selector
        );

        if let Ok(result) = page.evaluate(script.as_str()).await {
            if let Ok(coords) = result.into_value::<Vec<f64>>() {
                if coords.len() == 2 {
                    Self::move_mouse_to(page, coords[0], coords[1]).await?;
                }
            }
        }

        // Small delay before click
        Self::random_delay(100, 300).await;

        // Perform click
        let click_script = format!(
            r#"
            const element = document.querySelector('{}');
            if (element) {{
                element.click();
            }}
            "#,
            selector
        );
        page.evaluate(click_script.as_str()).await?;

        Ok(())
    }
}
