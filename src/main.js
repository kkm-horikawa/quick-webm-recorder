const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// ── State ──
let mode = "fullscreen"; // "fullscreen" | "region"
let saveDir = null;
let region = null; // { x, y, width, height } or null
let timerInterval = null;
let recordStartTime = null;
let lastSavedPath = null;
let isRecording = false;

// ── Elements ──
const toolbar = document.getElementById("toolbar");
const regionBar = document.getElementById("region-bar");
const recordingBar = document.getElementById("recording-bar");
const completeBar = document.getElementById("complete-bar");
const gifBar = document.getElementById("gif-bar");

const allBars = [toolbar, regionBar, recordingBar, completeBar, gifBar];

const btnFullscreen = document.getElementById("btn-fullscreen");
const btnRegion = document.getElementById("btn-region");
const btnSavedir = document.getElementById("btn-savedir");
const btnRecord = document.getElementById("btn-record");
const btnReselect = document.getElementById("btn-reselect");
const btnRegionRecord = document.getElementById("btn-region-record");
const btnStop = document.getElementById("btn-stop");
const btnOpen = document.getElementById("btn-open");
const btnGif = document.getElementById("btn-gif");
const btnNew = document.getElementById("btn-new");

const savedirLabel = document.getElementById("savedir-label");
const regionInfo = document.getElementById("region-info");
const timerEl = document.getElementById("timer");
const completePath = document.getElementById("complete-path");

// ── Helpers ──
function showOnly(el) {
  allBars.forEach((bar) => bar.classList.add("hidden"));
  el.classList.remove("hidden");
}

function formatTime(seconds) {
  const m = String(Math.floor(seconds / 60)).padStart(2, "0");
  const s = String(seconds % 60).padStart(2, "0");
  return `${m}:${s}`;
}

function generateFilename() {
  const now = new Date();
  const ts = now.toISOString().replace(/[-:T]/g, "").slice(0, 14);
  return `recording_${ts}.webm`;
}

async function getDesktopPath() {
  try {
    return await window.__TAURI__.path.desktopDir();
  } catch {
    return ".";
  }
}

async function buildOutputPath() {
  const dir = saveDir || (await getDesktopPath());
  const sep = dir.includes("\\") ? "\\" : "/";
  return dir + sep + generateFilename();
}

// ── Recording logic ──
async function startRecording(regionOverride) {
  try {
    const outputPath = await buildOutputPath();
    await invoke("start_recording", {
      region: regionOverride || null,
      outputPath: outputPath,
    });

    lastSavedPath = outputPath;
    isRecording = true;

    showOnly(recordingBar);
    recordStartTime = Date.now();
    timerEl.textContent = "00:00";
    timerInterval = setInterval(() => {
      const elapsed = Math.floor((Date.now() - recordStartTime) / 1000);
      timerEl.textContent = formatTime(elapsed);
    }, 1000);
  } catch (e) {
    alert("録画開始に失敗しました: " + e);
  }
}

async function stopRecording() {
  if (!isRecording) return;
  try {
    clearInterval(timerInterval);
    isRecording = false;
    const savedPath = await invoke("stop_recording");
    lastSavedPath = savedPath;

    const filename = savedPath.split(/[/\\]/).pop();
    completePath.textContent = filename;
    completePath.title = savedPath;
    showOnly(completeBar);
  } catch (e) {
    alert("録画停止に失敗しました: " + e);
    showOnly(toolbar);
  }
}

// ── Mode selection ──
btnFullscreen.addEventListener("click", () => {
  mode = "fullscreen";
  region = null;
  btnFullscreen.classList.add("active");
  btnRegion.classList.remove("active");
  showOnly(toolbar);
});

btnRegion.addEventListener("click", async () => {
  mode = "region";
  btnRegion.classList.add("active");
  btnFullscreen.classList.remove("active");
  try {
    await invoke("open_region_selector");
  } catch (e) {
    alert("範囲選択ウィンドウを開けませんでした: " + e);
    mode = "fullscreen";
    btnFullscreen.classList.add("active");
    btnRegion.classList.remove("active");
  }
});

// ── Region selection events ──
listen("region-selected", (event) => {
  region = event.payload;
  regionInfo.textContent = `${region.width} × ${region.height} (${region.x}, ${region.y})`;
  showOnly(regionBar);
});

listen("region-cancelled", () => {
  mode = "fullscreen";
  region = null;
  btnFullscreen.classList.add("active");
  btnRegion.classList.remove("active");
  showOnly(toolbar);
});

// ── Re-select region ──
btnReselect.addEventListener("click", async () => {
  try {
    await invoke("open_region_selector");
  } catch (e) {
    alert("範囲選択ウィンドウを開けませんでした: " + e);
  }
});

// ── Save directory selection ──
btnSavedir.addEventListener("click", async () => {
  try {
    const selected = await window.__TAURI__.dialog.open({
      directory: true,
      title: "保存先を選択",
    });
    if (selected) {
      saveDir = selected;
      const parts = selected.split(/[/\\]/);
      savedirLabel.textContent = parts[parts.length - 1] || parts[parts.length - 2];
    }
  } catch (e) {
    console.error("Failed to open directory picker:", e);
  }
});

// ── Start recording (fullscreen) ──
btnRecord.addEventListener("click", () => startRecording(null));

// ── Start recording (region) ──
btnRegionRecord.addEventListener("click", () => startRecording(region));

// ── Stop recording ──
btnStop.addEventListener("click", stopRecording);

// ── Keyboard shortcut: Escape to stop ──
document.addEventListener("keydown", (e) => {
  if (e.key === "Escape" && isRecording) {
    stopRecording();
  }
});

// ── Open saved file ──
btnOpen.addEventListener("click", async () => {
  if (lastSavedPath) {
    try {
      await window.__TAURI__.opener.openPath(lastSavedPath);
    } catch (e) {
      console.error("Failed to open file:", e);
    }
  }
});

// ── Convert to GIF ──
btnGif.addEventListener("click", async () => {
  if (!lastSavedPath) return;
  const videoPath = lastSavedPath;
  showOnly(gifBar);
  try {
    const gifPath = await invoke("convert_to_gif", {
      videoPath: videoPath,
      fps: 10,
      width: 800,
    });
    lastSavedPath = gifPath;
    const filename = gifPath.split(/[/\\]/).pop();
    completePath.textContent = filename;
    completePath.title = gifPath;
    showOnly(completeBar);
  } catch (e) {
    alert("GIF変換に失敗しました: " + e);
    showOnly(completeBar);
  }
});

// ── New recording ──
btnNew.addEventListener("click", () => {
  lastSavedPath = null;
  region = null;
  mode = "fullscreen";
  btnFullscreen.classList.add("active");
  btnRegion.classList.remove("active");
  showOnly(toolbar);
});
