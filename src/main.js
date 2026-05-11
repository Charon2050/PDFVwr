import { open } from "@tauri-apps/plugin-dialog";
import { listen } from "@tauri-apps/api/event";
import { convertFileSrc } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

const welcome  = document.getElementById("welcome");
const pdfFrame = document.getElementById("pdf-frame");
const appWin   = getCurrentWindow();

async function loadPdf(filePath) {
  if (!filePath) return;
  pdfFrame.src = convertFileSrc(filePath);
  pdfFrame.classList.add("visible");
  welcome.classList.add("hidden");
  const name = filePath.split(/[\\/]/).pop();
  // Set native window title
  await appWin.setTitle(`PDFVwr — ${name}`);
}

async function openFilePicker() {
  const selected = await open({
    multiple: false,
    filters: [{ name: "PDF 文件", extensions: ["pdf"] }],
  });
  if (selected) loadPdf(selected);
}

// Click anywhere on welcome screen to open
welcome.addEventListener("click", openFilePicker);

// Ctrl+O anywhere
document.addEventListener("keydown", (e) => {
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "o") {
    e.preventDefault();
    openFilePicker();
  }
});

// Drag visual feedback
document.addEventListener("dragover",  (e) => { e.preventDefault(); welcome.classList.add("drag-over"); });
document.addEventListener("dragleave", (e) => { if (!e.relatedTarget) welcome.classList.remove("drag-over"); });
document.addEventListener("drop",      (e) => { e.preventDefault(); welcome.classList.remove("drag-over"); });

(async () => {
  await listen("tauri://drag-drop", (event) => {
    welcome.classList.remove("drag-over");
    const paths = event.payload?.paths ?? [];
    const pdf   = paths.find((p) => p.toLowerCase().endsWith(".pdf"));
    if (pdf) loadPdf(pdf);
  });

  await listen("open-file", (event) => {
    if (event.payload) loadPdf(event.payload);
  });
})();
