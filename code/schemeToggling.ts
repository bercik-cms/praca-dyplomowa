export function toggleTheme() {
  let current = getTheme() || defaultTheme;
  let newTheme: string;

  if (current.indexOf("light") !== -1) {
    newTheme = current.replace("light", "dark");
  } else {
    newTheme = current.replace("dark", "light");
  }

  setTheme(newTheme);
}

export function setTheme(scheme: string) {
  if (Object.keys(colorschemes).indexOf(scheme) === -1) {
    console.error(`Colorscheme ${scheme} NOT DEFINED!`);
    return;
  }

  for (let [key, value] of Object.entries(colorschemes[scheme])) {
    document.documentElement.style.setProperty(`--${key}`, value as string);
  }

  // Odwrócenie kolorów w diagramie ER mermaid.js, gdy wybrano motyw ciemny
  let mermaidFilter = "";
  if (scheme.indexOf("dark") !== -1) mermaidFilter = "invert(1)";
  document.documentElement.style.setProperty("--mermaid-filter", mermaidFilter);

  localStorage.setItem("colorscheme", scheme);
}
