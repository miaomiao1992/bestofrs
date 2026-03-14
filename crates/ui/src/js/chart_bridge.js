function require_chart() {
  if (!window.Chart) {
    throw new Error(
      "Chart.js is not loaded. Ensure document::Script is added before chart usage.",
    );
  }
  return window.Chart;
}

function resolve_canvas(canvas_id) {
  const element = document.getElementById(canvas_id);
  if (!element) return null;
  if (!(element instanceof HTMLCanvasElement)) {
    throw new Error(`Element is not a canvas: ${canvas_id}`);
  }
  return element;
}
async function wait_for_canvas(canvas_id, max_attempts = 8) {
  for (let attempt = 0; attempt < max_attempts; attempt += 1) {
    const canvas = resolve_canvas(canvas_id);
    if (canvas) return canvas;
    await new Promise((resolve) => requestAnimationFrame(resolve));
  }
  return null;
}

export async function create_chart(canvas_id, config, drop) {
  const Chart = require_chart();
  const canvas = await wait_for_canvas(canvas_id);
  if (!canvas) {
    return null;
  }
  const context = canvas.getContext("2d");
  if (!context) {
    return null;
  }
  const chart = new Chart(context, config);

  drop.then(() => {
    try {
      chart.destroy();
    } catch {
      // ignore
    }
  });
  return chart;
}

export async function update_chart(chart, config) {
  chart.config = config;
  chart.update();
}
