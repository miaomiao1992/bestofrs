export async function redirect_to(url) {
  window.location.assign(url);
}

export async function blur_active_element() {
  const activeElement = document.activeElement;
  if (activeElement && typeof activeElement.blur === "function") {
    activeElement.blur();
  }
}

export async function focus_element_by_id(id) {
  const element = document.getElementById(id);
  if (element && typeof element.focus === "function") {
    element.focus();
  }
}

export async function rewrite_markdown_links(root_id) {
  const root = document.getElementById(root_id);
  if (!root) {
    return;
  }

  const linkBase = root.getAttribute("data-md-link-base-url") || "";
  const imageBase = root.getAttribute("data-md-image-base-url") || "";
  const isAbsolute = (value) => /^(?:[a-zA-Z][a-zA-Z\d+\-.]*:|\/\/)/.test(value);

  for (const element of root.querySelectorAll("[href], [src]")) {
    if (element.hasAttribute("href")) {
      const raw = element.getAttribute("href") || "";
      if (raw && linkBase) {
        if (raw.startsWith("#") || !isAbsolute(raw)) {
          try {
            element.setAttribute("href", new URL(raw, linkBase).toString());
          } catch {
            // ignore
          }
        }
      }
    }

    if (element.hasAttribute("src")) {
      const raw = element.getAttribute("src") || "";
      const base = imageBase || linkBase;
      if (raw && base && !isAbsolute(raw)) {
        try {
          element.setAttribute("src", new URL(raw, base).toString());
        } catch {
          // ignore
        }
      }
    }
  }
}
