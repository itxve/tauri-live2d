import fs from "fs";
import path from "path";

const TAURI_CONFIG = "src-tauri/tauri.conf.json";

export default function updateTauriVersion(new_version) {
  const file = path.join(process.cwd(), TAURI_CONFIG);

  if (!fs.existsSync(file)) {
    console.log("Could not found tauri.conf.json");
    process.exit(1);
  }

  let content = fs.readFileSync(file, { encoding: "utf8" });
  content = JSON.parse(content);
  content.package.version = new_version;
  fs.writeFileSync(file, JSON.stringify(content, null, 2));
}
